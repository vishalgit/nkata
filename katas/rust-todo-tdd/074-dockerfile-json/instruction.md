# JSON logs in the production image

**File:** `Dockerfile` (part 2 of 3)  
**Save to:** `~/code/todo/Dockerfile`

## Goal
In the production `Dockerfile`, after the runtime `ENV` block, turn on JSON logs and expose the metrics port.

## Why
`EXPOSE` documents the port; the Service in the next kata is what actually routes to it.

## Help
- Save with `:w! ~/code/todo/Dockerfile`.
