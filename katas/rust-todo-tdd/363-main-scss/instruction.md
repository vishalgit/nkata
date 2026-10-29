# The "1" of 7-1 — main.scss

**File:** `style/main.scss` (part 2 of 2)  
**Save to:** `~/code/todo/style/main.scss`

**▶ Task**
```sh
scripts/backlog task TODO-18 8 ui "7-1 folders, tokens, themes and components (8.7)" scss-7-1
```

## Goal
Create the seven folders and their partials, then replace the skeleton's one line with the six `@use`s.

## Run first
```sh
$ cd style
$ mkdir -p abstracts vendors base layout components pages themes
$ touch abstracts/{_index,_variables,_functions,_mixins}.scss vendors/{_index,_reset}.scss \
        base/{_index,_typography,_base}.scss layout/{_index,_site-header,_main}.scss \
        components/{_index,_button,_form,_alert,_todo-form,_todo-list,_todo-item}.scss \
        pages/{_index,_auth,_todos}.scss themes/{_index,_light,_dark}.scss
$ cd ..
```

## The 7-1 SCSS architecture
Seven folders of partials, one file that pulls them together, with the Sass module system (`@use` / `@forward`) and BEM names matching the classes the components emit.

| Folder | Holds | Emits CSS? |
| --- | --- | --- |
| `abstracts/` | tokens as Sass variables, functions, mixins | No — `@use`d where needed |
| `vendors/` | third-party CSS (a small reset) | Yes |
| `themes/` | CSS custom properties for light and dark | Yes |
| `base/` | element defaults, typography | Yes |
| `layout/` | page regions: header, main container | Yes |
| `components/` | reusable blocks: button, form, alert, todo-item | Yes |
| `pages/` | page-specific blocks: auth card, todo board | Yes |
## Why
- `abstracts` is not listed: it emits no CSS. Each partial that needs a token `@use`s it itself.
- Order matters: later layers may override earlier ones.
- Partials are created for the blocks that exist today; later chapters add one per new block.
- The build fails until the remaining partials exist; cargo-leptos recompiles on every save, so it turns green as you go.

## Help
- Save with `:w! ~/code/todo/style/main.scss`.
