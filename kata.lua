-- kata.lua — 4-pane kata runner for Neovim, with a Claude Code coach
--
--  ┌────────────────┬────────────────┐
--  │ 1 Editor       │ 3 Instruction  │
--  │                │   + status     │
--  ├────────────────┼────────────────┤
--  │ 2 Final result │ 4 Claude Code  │
--  │   (read-only)  │   (terminal)   │
--  └────────────────┴────────────────┘
--
-- Launched by the `nkata` script:  nkata ~/katas/org-basics [start-at]
-- The script runs `NVIM_APPNAME=kickstart nvim -S ~/.config/nkata/kata.lua`,
-- so your normal kickstart config loads first and this file runs on top.
--
-- Kata set layout:
--   <set>/001-name/instruction.md   (required)
--   <set>/001-name/expected.<ext>   (required — ext decides the filetype)
--   <set>/001-name/start.<ext>      (optional starter text for the editor)
--   <set>/.progress.json            (written by the runner)
--
-- Claude Code runs in pane 4 with the set folder as its working directory.
-- It gets standing rules via --append-system-prompt-file, and reads the
-- current kata from <tmp_root>/.session-<pid>/current.md, which the runner
-- rewrites on every kata load. When Claude edits the current kata's files,
-- panes 2 and 3 refresh on their own.

local M = {}

-- ── Config ──────────────────────────────────────────────────────────────
local config = {
  prefix = "<leader>k", -- next = n, prev = p, goto = g, reset = r, copy = w, run = x, claude = c, editor = e, quit = q
  nav_keys = { h = "<M-h>", j = "<M-j>", k = "<M-k>", l = "<M-l>" }, -- any pane → neighbouring pane, like vim's <C-w>hjkl
  to_instr_key = "<M-i>", -- any pane → instruction pane
  to_solution_key = "<M-s>", -- any pane → solution (final result) pane
  kill_pane_key = "<M-x>", -- any pane → close the focused pane (not the editor); the rest grow into its space
  restore_key = "<M-r>", -- any pane → bring back every closed pane
  tmp_root = "/tmp/nkata",
  claude_cmd = "claude", -- override with the NKATA_CLAUDE_CMD env var
  notify_claude_on_switch = false, -- tell Claude when you move to another kata (costs a model turn)
  clear_claude_on_switch = true, -- send /clear to Claude when you move to another kata (no model call)
  pass_color = "#a6e3a1",
  fail_color = "#f38ba8",
}

local ns = vim.api.nvim_create_namespace("kata")
local uv = vim.uv or vim.loop
local api = vim.api

local S = {
  set_dir = nil,
  set_name = nil,
  katas = {},
  idx = 1,
  win = {}, -- editor, result, instr, claude
  buf = {}, -- editor, result, instr, claude
  killed = {}, -- pane names the user closed on purpose; they stay closed across kata switches
  want = {}, -- normalized expected lines
  passed = false,
  diff_count = 0,
  started = 0,
  pass_secs = nil,
  progress = { completed = {} },
  augroup = nil,
  session_dir = nil,
  rules_path = nil,
  current_path = nil,
  claude_job = nil,
  watch = nil,
}

-- Standing instructions for Claude. {{CURRENT}} is replaced with the path
-- of the live current-kata file.
local RULES = [==[
# nkata kata coach

You are running in the bottom-right pane of nkata, a Neovim kata runner.
The user practises by typing into an editor pane until it matches a final
result. Your job is to coach them through the current kata and to help them
maintain the kata files. The pane is small: keep answers short.

## Which kata is current
The current kata changes while this session runs. Before answering anything
about "this kata", read {{CURRENT}} — it names the kata and gives the paths
of its files. When a message starts with "[nkata]", the user has moved to
another kata: re-read that file and reply with one short line naming the
new kata. Nothing more.

Moving between katas does not message you; the pane is cleared instead. Do
nothing until the user asks. When a message is just "kata?" (optionally
followed by a question), read {{CURRENT}}, then read the kata's final result
and the user's live attempt, and give a short hint on what differs. Without
a question after "kata?", reply with the kata's name and one hint.

## Hints, not answers
You may read the final result file to compare it with the user's live
attempt. Give hints: point at what differs, name the editor motion or org
syntax that would help, suggest an approach. Do not paste the full final
result or write out the solution unless the user explicitly asks for the
answer.

## Saving a hint into the instructions
When the user asks to save, keep or add a hint (for example "save that hint"
or "add this to the instructions"), add it to the current kata's
instruction.md as a short bullet under the "## Help" heading, creating that
heading at the end of the file if it is missing. Write it in your own short
words, as a nudge rather than the answer. Leave the rest of the file as it
is. Pane 3 refreshes by itself, so the user sees it straight away and on
every future run of the kata.

## What you may change
- The current kata's instruction.md, expected.<ext> and start.<ext>.
  Changes to expected.<ext> show in pane 2 immediately; start.<ext> applies
  the next time the user resets the kata.
- Do not edit the user's live attempt file, other katas, or .progress.json
  unless the user explicitly asks.
]==]

-- ── Helpers ─────────────────────────────────────────────────────────────
local function notify(msg, level)
  vim.notify(msg, level or vim.log.levels.INFO, { title = "kata" })
end

local function read_lines(path)
  if path and vim.fn.filereadable(path) == 1 then
    return vim.fn.readfile(path)
  end
  return {}
end

-- Trailing whitespace on each line and trailing blank lines don't count.
local function normalize(lines)
  local out = {}
  for i, l in ipairs(lines) do
    out[i] = (l:gsub("%s+$", ""))
  end
  while #out > 0 and out[#out] == "" do
    table.remove(out)
  end
  return out
end

local function win_ok(w)
  return w and api.nvim_win_is_valid(w) and api.nvim_win_get_tabpage(w) == api.nvim_get_current_tabpage()
end

local function buf_ok(b)
  return b and api.nvim_buf_is_valid(b)
end

local function fmt_time(s)
  if s >= 60 then
    return string.format("%dm %02ds", math.floor(s / 60), s % 60)
  end
  return s .. "s"
end

-- ── Kata discovery & progress ───────────────────────────────────────────
local function scan_katas(dir)
  local katas = {}
  for name, typ in vim.fs.dir(dir) do
    local kdir = dir .. "/" .. name
    local is_dir = typ == "directory" or (typ == "link" and vim.fn.isdirectory(kdir) == 1)
    if is_dir and not name:match("^%.") then
      local expected
      for f, ft in vim.fs.dir(kdir) do
        if ft == "file" and f:match("^expected%.") then
          expected = kdir .. "/" .. f
        end
      end
      if expected then
        local ext = expected:match("/expected%.([^/]+)$")
        table.insert(katas, {
          name = name,
          dir = kdir,
          ext = ext,
          expected = expected,
          start = kdir .. "/start." .. ext,
          instruction = kdir .. "/instruction.md",
        })
      end
    end
  end
  table.sort(katas, function(a, b)
    return a.name < b.name
  end)
  return katas
end

local function progress_path()
  return S.set_dir .. "/.progress.json"
end

local function load_progress()
  local lines = read_lines(progress_path())
  if #lines > 0 then
    local ok, data = pcall(vim.json.decode, table.concat(lines, "\n"))
    if ok and type(data) == "table" then
      if type(data.completed) ~= "table" then
        data.completed = {}
      end
      return data
    end
  end
  return { completed = {} }
end

local function save_progress()
  local k = S.katas[S.idx]
  S.progress.last = k and k.name or nil
  local data = { last = S.progress.last, completed = S.progress.completed }
  if next(data.completed) == nil then
    data.completed = vim.empty_dict()
  end
  pcall(vim.fn.writefile, { vim.json.encode(data) }, progress_path())
end

local function solved_count()
  local n = 0
  for _, k in ipairs(S.katas) do
    if S.progress.completed[k.name] then
      n = n + 1
    end
  end
  return n
end

-- query: a number ("2"), or a name prefix ("002" / "002-todo") → index, or nil
local function find_kata(query)
  local n = tonumber(query)
  if n and S.katas[n] then
    return n
  end
  for i, k in ipairs(S.katas) do
    if k.name:sub(1, #query) == query then
      return i
    end
  end
end

-- ── Highlights ──────────────────────────────────────────────────────────
local function setup_highlights()
  api.nvim_set_hl(0, "KataPass", { fg = config.pass_color, bold = true })
  api.nvim_set_hl(0, "KataFail", { fg = config.fail_color, bold = true })
  api.nvim_set_hl(0, "KataTitle", { link = "Title" })
  api.nvim_set_hl(0, "KataDim", { link = "Comment" })
end

-- ── Layout ──────────────────────────────────────────────────────────────
local function placeholder(w)
  local ph = api.nvim_create_buf(false, true)
  vim.bo[ph].bufhidden = "wipe"
  api.nvim_win_set_buf(w, ph)
end

-- Close a pane's window. Claude's terminal buffer is kept (and keeps running)
-- so restoring the pane brings the same session back.
local function close_pane(name)
  local w = S.win[name]
  S.win[name] = nil
  if name == "claude" and buf_ok(S.buf.claude) then
    vim.bo[S.buf.claude].bufhidden = "hide"
  end
  if win_ok(w) then
    api.nvim_win_close(w, true)
  end
end

local function build_layout()
  vim.cmd("silent! diffoff!")
  if buf_ok(S.buf.claude) then
    vim.bo[S.buf.claude].bufhidden = "hide"
  end
  vim.cmd("silent! only!")
  local editor = api.nvim_get_current_win()
  vim.cmd("rightbelow vsplit")
  local instr = api.nvim_get_current_win()
  vim.cmd("rightbelow split")
  local claude = api.nvim_get_current_win()
  api.nvim_set_current_win(editor)
  vim.cmd("rightbelow split")
  local result = api.nvim_get_current_win()
  -- New splits start out showing the editor's buffer; give them throwaway
  -- placeholders so deleting the old editor buffer can't close them.
  for _, w in ipairs({ result, instr }) do
    placeholder(w)
  end
  if buf_ok(S.buf.claude) then
    api.nvim_win_set_buf(claude, S.buf.claude) -- keep the running session
  else
    placeholder(claude)
  end
  S.win = { editor = editor, result = result, instr = instr, claude = claude }
  for name in pairs(S.killed) do
    close_pane(name)
  end
  vim.cmd("wincmd =")
end

local function layout_ok()
  for _, name in ipairs({ "editor", "result", "instr", "claude" }) do
    if not (S.killed[name] or win_ok(S.win[name])) then
      return false
    end
  end
  return true
end

local function ensure_layout()
  if not layout_ok() then
    build_layout()
  end
end

local function equalize()
  if layout_ok() then
    vim.cmd("wincmd =")
  end
end

local function apply_win_opts()
  for _, w in ipairs({ S.win.editor, S.win.result }) do
    if win_ok(w) then
      vim.wo[w].foldenable = false
      vim.wo[w].conceallevel = 0 -- show the real text, markers and all
      vim.wo[w].wrap = false
    end
  end
  for _, w in ipairs({ S.win.instr, S.win.claude }) do
    if win_ok(w) then
      vim.wo[w].number = false
      vim.wo[w].relativenumber = false
      vim.wo[w].signcolumn = "no"
      vim.wo[w].foldcolumn = "0"
      vim.wo[w].cursorline = false
      vim.wo[w].spell = false
    end
  end
  if win_ok(S.win.instr) then
    vim.wo[S.win.instr].wrap = true
    vim.wo[S.win.instr].linebreak = true
  end
end

local function set_border(pass)
  local v = pass and "WinSeparator:KataPass,VertSplit:KataPass" or ""
  for _, w in pairs(S.win) do
    if win_ok(w) then
      vim.wo[w].winhighlight = v
    end
  end
end

local function setup_diff()
  vim.cmd("silent! diffoff!")
  if win_ok(S.win.editor) and win_ok(S.win.result) then
    for _, w in ipairs({ S.win.editor, S.win.result }) do
      api.nvim_win_call(w, function()
        vim.cmd("diffthis")
      end)
    end
  end
  apply_win_opts()
end

-- ── Claude Code pane ────────────────────────────────────────────────────
local function write_rules()
  vim.fn.mkdir(S.session_dir, "p")
  S.rules_path = S.session_dir .. "/claude-rules.md"
  S.current_path = S.session_dir .. "/current.md"
  local text = RULES:gsub("{{CURRENT}}", (S.current_path:gsub("%%", "%%%%")))
  vim.fn.writefile(vim.split(text, "\n"), S.rules_path)
end

local function editor_path(k)
  local dir = string.format("%s/%s/%s", config.tmp_root, S.set_name, k.name)
  vim.fn.mkdir(dir, "p")
  return dir .. "/solution." .. k.ext
end

local function write_current(k)
  local start_note = vim.fn.filereadable(k.start) == 1 and "" or " (does not exist yet; blank editor)"
  local solved = S.progress.completed[k.name] and "solved before" or "not solved yet"
  vim.fn.writefile({
    "# Current kata",
    "",
    string.format("- Set: %s (%s)", S.set_name, S.set_dir),
    string.format("- Kata: %d of %d, %s (%s)", S.idx, #S.katas, k.name, solved),
    "",
    "## Files",
    "- Instruction, shown in pane 3 (you may edit): " .. k.instruction,
    "- Final result, shown in pane 2 (you may edit): " .. k.expected,
    "- Starter text for the editor (you may edit or create): " .. k.start .. start_note,
    "- The user's live attempt, pane 1 (read only for you): " .. editor_path(k),
  }, S.current_path)
end

local function claude_argv()
  local cmd = vim.env.NKATA_CLAUDE_CMD
  if not cmd or cmd == "" then
    cmd = config.claude_cmd
  end
  return vim.split(cmd, "%s+", { trimempty = true })
end

local function claude_alive()
  return S.claude_job and S.claude_job > 0 and vim.fn.jobwait({ S.claude_job }, 0)[1] == -1
end

local function start_claude()
  local w = S.win.claude
  if not win_ok(w) then
    return
  end
  local old = S.buf.claude
  local b = api.nvim_create_buf(false, true)
  api.nvim_win_set_buf(w, b)
  S.buf.claude = b
  S.claude_job = nil
  if buf_ok(old) and old ~= b then
    pcall(api.nvim_buf_delete, old, { force = true })
  end

  local argv = claude_argv()
  if #argv == 0 or vim.fn.executable(argv[1]) ~= 1 then
    api.nvim_buf_set_lines(b, 0, -1, false, {
      "",
      "  `" .. (argv[1] or "claude") .. "` was not found on your PATH.",
      "",
      "  Install Claude Code, or set NKATA_CLAUDE_CMD,",
      "  then press " .. config.prefix .. "c to start it here.",
    })
    vim.bo[b].modifiable = false
    return
  end

  vim.list_extend(argv, { "--append-system-prompt-file", S.rules_path, "--add-dir", config.tmp_root })
  local opts = {
    cwd = S.set_dir,
    on_exit = function()
      S.claude_job = nil
    end,
  }
  local job
  api.nvim_win_call(w, function()
    if vim.fn.has("nvim-0.11") == 1 then
      opts.term = true
      job = vim.fn.jobstart(argv, opts)
    else
      job = vim.fn.termopen(argv, opts)
    end
  end)
  if job and job > 0 then
    S.claude_job = job
  end
  S.buf.claude = api.nvim_win_get_buf(w)
  vim.wo[w].number = false
  vim.wo[w].relativenumber = false
  vim.wo[w].signcolumn = "no"
end

-- Start Claude once per session. After that, only make sure its buffer is
-- the one showing in pane 4; if Claude exited, <prefix>c restarts it.
local function ensure_claude()
  if not S.claude_started then
    S.claude_started = true
    start_claude()
    return
  end
  if win_ok(S.win.claude) and buf_ok(S.buf.claude) and api.nvim_win_get_buf(S.win.claude) ~= S.buf.claude then
    api.nvim_win_set_buf(S.win.claude, S.buf.claude)
  end
end

-- Type a one-line notice into Claude's prompt, then press Enter separately
-- so it isn't treated as part of a paste.
local function notify_claude(k)
  if not (config.notify_claude_on_switch and claude_alive()) then
    return
  end
  local msg = string.format("[nkata] Now on kata %d/%d: %s. Re-read %s", S.idx, #S.katas, k.name, S.current_path)
  vim.fn.chansend(S.claude_job, msg)
  local job = S.claude_job
  vim.defer_fn(function()
    if S.claude_job == job and claude_alive() then
      vim.fn.chansend(job, "\r")
    end
  end, 200)
end

-- Wipe Claude's conversation so old katas aren't re-sent with every turn.
-- /clear is handled locally by Claude Code, so it costs no tokens. Ctrl-U
-- first drops anything half-typed in the prompt.
local function clear_claude()
  if not (config.clear_claude_on_switch and claude_alive()) then
    return
  end
  local job = S.claude_job
  vim.fn.chansend(job, "\21/clear")
  vim.defer_fn(function()
    if S.claude_job == job and claude_alive() then
      vim.fn.chansend(job, "\r")
    end
  end, 200)
end

-- ── Instruction pane ────────────────────────────────────────────────────
local function render()
  local b = S.buf.instr
  local k = S.katas[S.idx]
  if not (buf_ok(b) and k) then
    return
  end

  local p = config.prefix
  local rule = string.rep("─", 40)
  local lines, marks = {}, {}
  local function add(text, hl)
    table.insert(lines, text)
    if hl then
      marks[#lines - 1] = hl
    end
  end

  add(string.format(" Kata %d/%d · %s", S.idx, #S.katas, k.name), "KataTitle")
  add(string.format(" Set: %s · solved %d/%d", S.set_name, solved_count(), #S.katas), "KataDim")
  add("")

  if S.passed then
    add(string.format(" ✓ PASSED in %s", fmt_time(S.pass_secs or 0)), "KataPass")
    if S.idx < #S.katas then
      add(string.format("   press %sn for the next kata", p), "KataPass")
    elseif solved_count() == #S.katas then
      add("   every kata in this set is solved 🎉", "KataPass")
    else
      add("   last kata — " .. p .. "p to go back to unsolved ones", "KataPass")
    end
  else
    local noun = S.diff_count == 1 and "line differs" or "lines differ"
    add(string.format(" ✗ %d %s from the final result", S.diff_count, noun), "KataFail")
    local prev = S.progress.completed[k.name]
    if prev then
      add(string.format("   (solved before in %s)", fmt_time(prev.seconds or 0)), "KataDim")
    end
  end

  add(rule, "KataDim")
  local instr = read_lines(k.instruction)
  if #instr == 0 then
    instr = { "(no instruction.md for this kata)" }
  end
  for _, l in ipairs(instr) do
    add(l)
  end
  add("")
  add(rule, "KataDim")
  add(string.format(" %sn next   %sp previous   %sg go to", p, p, p), "KataDim")
  add(string.format(" %sr reset   %sw copy to…   %sx run in shell", p, p, p), "KataDim")
  add(string.format(" %sc Claude   %se editor   %sq quit", p, p, p), "KataDim")
  add(" <M-h/j/k/l> move between panes", "KataDim")
  add(string.format(" %s close pane · %s restore panes", config.kill_pane_key, config.restore_key), "KataDim")

  vim.bo[b].modifiable = true
  api.nvim_buf_set_lines(b, 0, -1, false, lines)
  vim.bo[b].modifiable = false
  vim.bo[b].modified = false

  api.nvim_buf_clear_namespace(b, ns, 0, -1)
  for row, hl in pairs(marks) do
    api.nvim_buf_set_extmark(b, ns, row, 0, { line_hl_group = hl })
  end
end

-- ── Checking ────────────────────────────────────────────────────────────
local function on_pass()
  local k = S.katas[S.idx]
  local secs = math.floor((uv.hrtime() - S.started) / 1e9)
  S.pass_secs = secs
  local prev = S.progress.completed[k.name]
  if not prev or secs < (prev.seconds or math.huge) then
    S.progress.completed[k.name] = { seconds = secs, at = os.date("!%Y-%m-%dT%H:%M:%SZ") }
  end
  save_progress()
  notify(string.format("✓ %s passed in %s", k.name, fmt_time(secs)))
end

local function check()
  if not (buf_ok(S.buf.editor) and S.katas[S.idx]) then
    return
  end
  local got = normalize(api.nvim_buf_get_lines(S.buf.editor, 0, -1, false))
  local want = S.want
  local diff = 0
  for i = 1, math.max(#got, #want) do
    if got[i] ~= want[i] then
      diff = diff + 1
    end
  end
  S.diff_count = diff
  local now = diff == 0
  if now and not S.passed then
    S.passed = true
    on_pass()
  elseif not now then
    S.passed = false
  end
  pcall(vim.cmd, "diffupdate")
  set_border(S.passed)
  render()
end

-- ── Live reload when kata files change on disk ──────────────────────────
local function refresh_from_disk(k)
  if S.katas[S.idx] ~= k then
    return
  end
  if vim.fn.filereadable(k.expected) == 1 then
    local expected = read_lines(k.expected)
    if buf_ok(S.buf.result) then
      local shown = api.nvim_buf_get_lines(S.buf.result, 0, -1, false)
      if not vim.deep_equal(expected, shown) then
        vim.bo[S.buf.result].modifiable = true
        api.nvim_buf_set_lines(S.buf.result, 0, -1, false, expected)
        vim.bo[S.buf.result].modifiable = false
        vim.bo[S.buf.result].modified = false
      end
    end
    S.want = normalize(expected)
  end
  write_current(k) -- e.g. a start.<ext> that now exists
  check() -- re-renders the instruction pane too
end

local function stop_watch()
  if S.watch then
    S.watch:stop()
    if not S.watch:is_closing() then
      S.watch:close()
    end
    S.watch = nil
  end
end

local function start_watch(k)
  stop_watch()
  local h = uv.new_fs_event()
  if not h then
    return
  end
  local pending = false
  h:start(k.dir, {}, function(err)
    if err or pending then
      return
    end
    pending = true
    vim.schedule(function()
      vim.defer_fn(function()
        pending = false
        refresh_from_disk(k)
      end, 150) -- let editors finish write-then-rename saves
    end)
  end)
  S.watch = h
end

-- ── Loading a kata ──────────────────────────────────────────────────────
local function show_scratch(win, b, name, ft)
  api.nvim_win_set_buf(win, b) -- old scratch buffer wipes itself here
  pcall(api.nvim_buf_set_name, b, name)
  vim.bo[b].filetype = ft
  vim.bo[b].modified = false
end

local function kata_ft(k)
  return vim.filetype.match({ filename = k.expected }) or k.ext
end

-- Final result (pane 2) and instruction (pane 3); a closed pane is skipped.
local function show_side_panes(k)
  local expected = read_lines(k.expected)
  S.want = normalize(expected)
  if win_ok(S.win.result) then
    local rb = api.nvim_create_buf(false, true)
    api.nvim_buf_set_lines(rb, 0, -1, false, expected)
    vim.bo[rb].bufhidden = "wipe"
    vim.bo[rb].modifiable = false
    show_scratch(S.win.result, rb, "kata://final-result/" .. k.name, kata_ft(k))
    S.buf.result = rb
  end
  if win_ok(S.win.instr) then
    local ib = api.nvim_create_buf(false, true)
    vim.bo[ib].bufhidden = "wipe"
    show_scratch(S.win.instr, ib, "kata://instruction/" .. k.name, "markdown")
    vim.bo[ib].modifiable = false
    S.buf.instr = ib
  end
end

local function load_kata(idx, tell_claude)
  local k = S.katas[idx]
  if not k then
    return
  end
  S.idx = idx
  S.passed = false
  S.pass_secs = nil
  S.started = uv.hrtime()
  ensure_layout()

  -- 1. Editor: a real file in /tmp, blank or seeded from start.<ext>
  local old_editor = S.buf.editor
  local path = editor_path(k)
  vim.fn.writefile(read_lines(k.start), path)
  api.nvim_win_call(S.win.editor, function()
    vim.cmd("silent edit! " .. vim.fn.fnameescape(path))
  end)
  S.buf.editor = api.nvim_win_get_buf(S.win.editor)
  if buf_ok(old_editor) and old_editor ~= S.buf.editor then
    pcall(api.nvim_buf_delete, old_editor, { force = true })
  end
  if vim.bo[S.buf.editor].filetype == "" then
    vim.bo[S.buf.editor].filetype = kata_ft(k)
  end

  -- 2 + 3. Final result (read-only) and instruction
  show_side_panes(k)

  -- 4. Claude: point it at this kata; start it on first load
  write_current(k)
  ensure_claude()
  if tell_claude then
    clear_claude()
    notify_claude(k)
  end

  -- Diff + live checking + file watching
  setup_diff()
  S.augroup = api.nvim_create_augroup("KataLive", { clear = true })
  api.nvim_create_autocmd({ "TextChanged", "TextChangedI", "TextChangedP" }, {
    group = S.augroup,
    buffer = S.buf.editor,
    callback = check,
  })
  start_watch(k)

  save_progress()
  equalize()
  api.nvim_set_current_win(S.win.editor)
  check()
end

-- ── Actions ─────────────────────────────────────────────────────────────
function M.next()
  if S.idx >= #S.katas then
    notify("Already on the last kata")
    return
  end
  load_kata(S.idx + 1, true)
end

function M.prev()
  if S.idx <= 1 then
    notify("Already on the first kata")
    return
  end
  load_kata(S.idx - 1, true)
end

-- Jump to any kata by number or name prefix. `query` is optional:
-- :KataGoto <query> skips the prompt.
function M.go_to(query)
  local function go(input)
    input = input and vim.trim(input) or ""
    if input == "" then
      return -- prompt cancelled
    end
    local idx = find_kata(input)
    if not idx then
      notify("No kata matches '" .. input .. "'", vim.log.levels.WARN)
      return
    end
    if idx == S.idx then
      notify("Already on kata " .. idx)
      return
    end
    load_kata(idx, true)
  end

  if query and query ~= "" then
    go(query)
    return
  end
  vim.ui.input({ prompt = string.format("Go to kata (1-%d): ", #S.katas) }, go)
end

function M.reset()
  local k = S.katas[S.idx]
  if not (k and buf_ok(S.buf.editor)) then
    return
  end
  local path = editor_path(k)
  vim.fn.writefile(read_lines(k.start), path)
  ensure_layout()
  api.nvim_win_call(S.win.editor, function()
    vim.cmd("silent edit! " .. vim.fn.fnameescape(path))
  end)
  S.buf.editor = api.nvim_win_get_buf(S.win.editor)
  S.passed = false
  setup_diff()
  check()
end

-- Focus the Claude pane in insert mode, restarting Claude if it has exited.
local function pane_closed(name, label)
  if S.killed[name] then
    notify(string.format("%s pane is closed — %s to restore", label, config.restore_key))
    return true
  end
  return false
end

function M.claude()
  ensure_layout()
  if pane_closed("claude", "Claude") then
    return
  end
  if not claude_alive() then
    start_claude()
  end
  if win_ok(S.win.claude) then
    api.nvim_set_current_win(S.win.claude)
    if claude_alive() then
      vim.cmd("startinsert")
    end
  end
end

-- Leave the Claude pane (or any pane) and return to the editor.
function M.editor()
  if win_ok(S.win.editor) then
    api.nvim_set_current_win(S.win.editor)
  end
end

-- Focus the instruction pane.
function M.instruction()
  ensure_layout()
  if pane_closed("instr", "Instruction") then
    return
  end
  if win_ok(S.win.instr) then
    api.nvim_set_current_win(S.win.instr)
  end
end

-- Focus the solution (final result) pane.
function M.solution()
  ensure_layout()
  if pane_closed("result", "Solution") then
    return
  end
  if win_ok(S.win.result) then
    api.nvim_set_current_win(S.win.result)
  end
end

-- Close the focused pane (never the editor) so its neighbours grow into the
-- space. It stays closed across kata switches until M.restore_layout().
function M.kill_pane()
  local cur = api.nvim_get_current_win()
  local name
  for n, w in pairs(S.win) do
    if w == cur then
      name = n
    end
  end
  if not name then
    notify("Not in a kata pane", vim.log.levels.WARN)
    return
  end
  if name == "editor" then
    notify("The editor pane can't be closed", vim.log.levels.WARN)
    return
  end
  S.killed[name] = true
  close_pane(name)
  if name == "result" then
    S.buf.result = nil
    vim.cmd("silent! diffoff!")
  elseif name == "instr" then
    S.buf.instr = nil
  end
  equalize()
  M.editor()
end

-- Bring back every closed pane. The editor text is untouched, Claude's
-- session returns as it was, and panes 2 and 3 are refilled for this kata.
function M.restore_layout()
  if next(S.killed) == nil then
    notify("All panes are already open")
    return
  end
  S.killed = {}
  if win_ok(S.win.editor) then
    api.nvim_set_current_win(S.win.editor)
  end
  build_layout()
  local k = S.katas[S.idx]
  if k then
    show_side_panes(k)
  end
  ensure_claude()
  setup_diff()
  set_border(S.passed)
  check()
  equalize()
  api.nvim_set_current_win(S.win.editor)
end

-- Write a copy of the editor pane's current text to a path you type in.
-- Copies what's in the buffer right now (saved or not); the kata itself is
-- untouched and the editor keeps working on its own scratch file.
-- `path` is optional: :KataCopy <path> skips the prompt.
function M.copy(path)
  local k = S.katas[S.idx]
  if not (k and buf_ok(S.buf.editor)) then
    return
  end
  local default_name = k.name .. "." .. k.ext

  local function write_to(input)
    if not input then
      return -- prompt cancelled
    end
    input = vim.trim(input)
    if input == "" then
      return
    end
    local target = vim.fn.fnamemodify(vim.fn.expand(input), ":p")
    -- A folder (existing, or typed with a trailing /) gets the default name.
    if vim.fn.isdirectory(target) == 1 or input:match("/$") then
      target = target:gsub("/+$", "") .. "/" .. default_name
    end
    if target == editor_path(k) then
      notify("That's the editor's own scratch file — pick another path", vim.log.levels.WARN)
      return
    end
    if vim.fn.filereadable(target) == 1 then
      local choice = vim.fn.confirm(target .. " already exists. Overwrite it?", "&Yes\n&No", 2)
      if choice ~= 1 then
        notify("Copy cancelled")
        return
      end
    end
    local parent = vim.fn.fnamemodify(target, ":h")
    if vim.fn.isdirectory(parent) == 0 and vim.fn.mkdir(parent, "p") == 0 then
      notify("Couldn't create " .. parent, vim.log.levels.ERROR)
      return
    end
    local lines = api.nvim_buf_get_lines(S.buf.editor, 0, -1, false)
    local ok, res = pcall(vim.fn.writefile, lines, target)
    if ok and res == 0 then
      notify("Copied the editor to " .. vim.fn.fnamemodify(target, ":~"))
    else
      notify("Couldn't write " .. target .. (ok and "" or (": " .. tostring(res))), vim.log.levels.ERROR)
    end
  end

  if path and path ~= "" then
    write_to(path)
    return
  end
  vim.ui.input({
    prompt = "Copy editor to: ",
    default = vim.fn.fnamemodify(vim.fn.getcwd(), ":~") .. "/" .. default_name,
    completion = "file",
  }, write_to)
end

-- ── Run selected text in the shell ──────────────────────────────────────
-- The text runs as one script through your 'shell' (so multi-line selections,
-- pipes and `cd` work), from the folder nkata was launched in. Output goes to
-- a floating window that streams as the command runs; q / <Esc> closes it and
-- stops the command if it's still going.
local function selected_text()
  local mode = vim.fn.mode()
  if mode == "v" or mode == "V" or mode == "\22" then
    local lines = vim.fn.getregion(vim.fn.getpos("v"), vim.fn.getpos("."), { type = mode })
    api.nvim_feedkeys(vim.keycode("<Esc>"), "nx", false)
    return table.concat(lines, "\n")
  end
  return api.nvim_get_current_line()
end

local function open_output_float(title)
  local b = api.nvim_create_buf(false, true)
  vim.bo[b].bufhidden = "wipe"
  local width = math.max(40, math.floor(vim.o.columns * 0.7))
  local height = math.max(8, math.floor(vim.o.lines * 0.6))
  local w = api.nvim_open_win(b, true, {
    relative = "editor",
    width = width,
    height = height,
    row = math.floor((vim.o.lines - height) / 2) - 1,
    col = math.floor((vim.o.columns - width) / 2),
    style = "minimal",
    border = "rounded",
    title = " " .. title .. " ",
    title_pos = "center",
  })
  vim.wo[w].wrap = true
  return b, w
end

function M.run(cmd)
  cmd = cmd or selected_text()
  if vim.trim(cmd) == "" then
    notify("Nothing to run — select a command first", vim.log.levels.WARN)
    return
  end
  local first = vim.split(vim.trim(cmd), "\n")[1]
  local b, w = open_output_float("$ " .. (#first > 50 and first:sub(1, 47) .. "…" or first))
  local header = vim.tbl_map(function(l)
    return "$ " .. l
  end, vim.split(cmd, "\n"))
  table.insert(header, "")
  api.nvim_buf_set_lines(b, 0, -1, false, header)
  for i = 0, #header - 2 do
    api.nvim_buf_set_extmark(b, ns, i, 0, { line_hl_group = "KataTitle" })
  end

  local partial = { stdout = "", stderr = "" }
  local function append(lines, hl)
    if not buf_ok(b) or #lines == 0 then
      return
    end
    local start = api.nvim_buf_line_count(b)
    api.nvim_buf_set_lines(b, -1, -1, false, lines)
    if hl then
      for i = start, start + #lines - 1 do
        api.nvim_buf_set_extmark(b, ns, i, 0, { line_hl_group = hl })
      end
    end
    if win_ok(w) then
      api.nvim_win_set_cursor(w, { api.nvim_buf_line_count(b), 0 })
    end
  end
  -- Split stream chunks into whole lines; keep the unfinished tail for later.
  local function on_chunk(stream)
    return function(_, data)
      if not data then
        return
      end
      vim.schedule(function()
        local text = partial[stream] .. data:gsub("\r\n", "\n")
        local lines = vim.split(text, "\n")
        partial[stream] = table.remove(lines)
        append(lines, stream == "stderr" and "KataFail" or nil)
      end)
    end
  end

  local ok, proc = pcall(vim.system, { vim.o.shell, vim.o.shellcmdflag, cmd }, {
    cwd = vim.fn.getcwd(),
    text = true,
    stdin = false,
    detach = true,
    stdout = on_chunk("stdout"),
    stderr = on_chunk("stderr"),
  }, function(res)
    vim.schedule(function()
      if partial.stdout ~= "" then
        append({ partial.stdout })
      end
      if partial.stderr ~= "" then
        append({ partial.stderr }, "KataFail")
      end
      local code = res.code
      local msg = res.signal and res.signal ~= 0 and string.format("[stopped by signal %d]", res.signal)
        or string.format("[exit %d]", code)
      append({ "", msg }, code == 0 and "KataPass" or "KataFail")
    end)
  end)
  if not ok then
    append({ "Couldn't start " .. vim.o.shell .. ": " .. tostring(proc) }, "KataFail")
    proc = nil
  end

  local closed = false
  local function close(from_leave)
    if closed then
      return
    end
    closed = true
    -- The shell runs in its own process group (detach), so kill the group:
    -- that takes down `sleep`, servers etc. the shell started, not just the shell.
    if proc and not proc:is_closing() then
      if not pcall(uv.kill, -proc.pid, "sigterm") then
        pcall(proc.kill, proc, "sigterm")
      end
    end
    if win_ok(w) then
      api.nvim_win_close(w, true)
    end
    if not from_leave and win_ok(S.win.editor) then
      api.nvim_set_current_win(S.win.editor)
    end
  end
  for _, key in ipairs({ "q", "<Esc>" }) do
    vim.keymap.set("n", key, function()
      close(false)
    end, { buffer = b, nowait = true, silent = true })
  end
  -- Don't leave a detached command running after nkata quits.
  api.nvim_create_autocmd("VimLeavePre", {
    once = true,
    callback = function()
      close(true)
    end,
  })
  -- Clicking or hopping to another pane also closes the float.
  api.nvim_create_autocmd("WinLeave", {
    buffer = b,
    once = true,
    callback = function()
      vim.schedule(function()
        close(true)
      end)
    end,
  })
end

function M.quit()
  save_progress()
  vim.cmd("qa!")
end

local function setup_keymaps()
  local p = config.prefix
  local maps = {
    { "n", M.next, "Kata: next" },
    { "p", M.prev, "Kata: previous" },
    { "g", M.go_to, "Kata: go to kata by number" },
    { "r", M.reset, "Kata: reset editor" },
    { "c", M.claude, "Kata: focus/restart Claude" },
    { "e", M.editor, "Kata: back to editor" },
    { "w", M.copy, "Kata: write a copy of the editor to a path" },
    { "q", M.quit, "Kata: quit" },
  }
  for _, m in ipairs(maps) do
    vim.keymap.set("n", p .. m[1], function()
      m[2]()
    end, { desc = m[3], silent = true })
  end
  -- Visual: run the selection. Normal: run the current line.
  vim.keymap.set({ "n", "x" }, p .. "x", function()
    M.run()
  end, { desc = "Kata: run selection/line in the shell", silent = true })

  -- Quick hops that don't need <leader>, so they also work mid-typing.
  -- Vim-style pane movement from any mode. Landing in the Claude terminal
  -- resumes typing; leaving it needs <C-\\><C-n> first.
  for dir, key in pairs(config.nav_keys) do
    vim.keymap.set({ "n", "i", "t" }, key, function()
      vim.cmd("stopinsert")
      vim.cmd("wincmd " .. dir)
      if vim.bo.buftype == "terminal" then
        vim.cmd("startinsert")
      end
    end, { desc = "Kata: move to the " .. dir .. " pane", silent = true })
  end
  -- Instruction / solution panes, from any mode (terminal mode needs the
  -- extra <C-\><C-n> to leave the Claude terminal first).
  vim.keymap.set({ "n", "i", "t" }, config.to_instr_key, [[<C-\><C-n><Cmd>lua Kata.instruction()<CR>]],
    { desc = "Kata: to instruction pane", silent = true })
  vim.keymap.set({ "n", "i", "t" }, config.to_solution_key, [[<C-\><C-n><Cmd>lua Kata.solution()<CR>]],
    { desc = "Kata: to solution pane", silent = true })

  vim.keymap.set({ "n", "i", "t" }, config.kill_pane_key, [[<C-\><C-n><Cmd>lua Kata.kill_pane()<CR>]],
    { desc = "Kata: close focused pane", silent = true })
  vim.keymap.set({ "n", "i", "t" }, config.restore_key, [[<C-\><C-n><Cmd>lua Kata.restore_layout()<CR>]],
    { desc = "Kata: restore closed panes", silent = true })

  api.nvim_create_user_command("KataKill", M.kill_pane, {})
  api.nvim_create_user_command("KataRestore", M.restore_layout, {})
  api.nvim_create_user_command("KataNext", M.next, {})
  api.nvim_create_user_command("KataPrev", M.prev, {})
  api.nvim_create_user_command("KataGoto", function(o)
    M.go_to(o.args)
  end, { nargs = "?" })
  api.nvim_create_user_command("KataReset", M.reset, {})
  api.nvim_create_user_command("KataClaude", M.claude, {})
  api.nvim_create_user_command("KataEditor", M.editor, {})
  api.nvim_create_user_command("KataInstruction", M.instruction, {})
  api.nvim_create_user_command("KataSolution", M.solution, {})
  api.nvim_create_user_command("KataCopy", function(o)
    M.copy(o.args)
  end, { nargs = "?", complete = "file" })
  api.nvim_create_user_command("KataRun", function(o)
    if o.args ~= "" then
      M.run(o.args)
    else
      local lines = api.nvim_buf_get_lines(0, o.line1 - 1, o.line2, false)
      M.run(table.concat(lines, "\n"))
    end
  end, { nargs = "*", range = true, complete = "shellcmd" })
  api.nvim_create_user_command("KataQuit", M.quit, {})
end

local function setup_session_autocmds()
  local g = api.nvim_create_augroup("KataSession", { clear = true })
  api.nvim_create_autocmd("VimResized", {
    group = g,
    callback = function()
      vim.schedule(equalize)
    end,
  })
  api.nvim_create_autocmd("ColorScheme", { group = g, callback = setup_highlights })
  api.nvim_create_autocmd("VimLeavePre", {
    group = g,
    callback = function()
      stop_watch()
      if S.session_dir then
        vim.fn.delete(S.session_dir, "rf")
      end
    end,
  })
end

-- start_at: a number ("2"), a name prefix ("002" / "002-todo"), or nil → first unsolved
local function pick_start(start_at)
  if start_at and start_at ~= "" then
    local idx = find_kata(start_at)
    if idx then
      return idx
    end
    notify("No kata matches '" .. start_at .. "', resuming instead", vim.log.levels.WARN)
  end
  for i, k in ipairs(S.katas) do
    if not S.progress.completed[k.name] then
      return i
    end
  end
  notify("Every kata in this set is solved — starting from the first")
  return 1
end

function M.start(set_dir, start_at)
  set_dir = vim.fs.normalize(set_dir):gsub("/+$", "")
  if vim.fn.isdirectory(set_dir) == 0 then
    notify("Not a directory: " .. set_dir, vim.log.levels.ERROR)
    return
  end
  S.set_dir = set_dir
  S.set_name = vim.fs.basename(set_dir)
  S.katas = scan_katas(set_dir)
  if #S.katas == 0 then
    notify("No katas found in " .. set_dir .. " (each needs an expected.<ext>)", vim.log.levels.ERROR)
    return
  end
  S.progress = load_progress()
  S.session_dir = string.format("%s/.session-%d", config.tmp_root, vim.fn.getpid())
  write_rules()
  setup_highlights()
  setup_session_autocmds()
  setup_keymaps()
  load_kata(pick_start(start_at), false)
end

M.state = S -- handy for debugging: :lua =Kata.state

-- ── Bootstrap ───────────────────────────────────────────────────────────
_G.Kata = M
local set = vim.env.KATA_SET
if set and set ~= "" then
  local function go()
    M.start(set, vim.env.KATA_START)
  end
  if vim.v.vim_did_enter == 1 then
    vim.schedule(go)
  else
    api.nvim_create_autocmd("VimEnter", {
      once = true,
      callback = function()
        vim.schedule(go)
      end,
    })
  end
end

return M
