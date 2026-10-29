# Publish the app and live-reload ports

**File:** `compose.yaml` (part 2 of 3)  
**Save to:** `~/code/todo/compose.yaml`

## Goal
Under `dev:` in `compose.yaml`, after `volumes:`, publish ports 3000 and 3001.

## Why
3000 is the app; 3001 is the websocket cargo-leptos uses to reload your browser after each build.

## Help
- Save with `:w! ~/code/todo/compose.yaml`.
