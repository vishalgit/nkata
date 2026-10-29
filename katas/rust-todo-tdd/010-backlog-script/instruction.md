# Just-in-time issues — scripts/backlog

**File:** `scripts/backlog`  
**Save to:** `~/code/todo/scripts/backlog`

## Goal
Type `scripts/backlog`: one command per epic, story and task, each landing on the board with its fields, parent link and (for tasks) a branch.

## Why
- **Issues arrive just in time.** An epic's issue is created right before its first story, a story's when its card appears, a task's when you start it.
- `find_issue` makes `epic` and `story` idempotent: running one twice prints "already exists".
- `task` creates its chapter's milestone if it is new, so the milestone view reads as a table of contents, one chapter at a time.
- `TASK_ONLY=1` makes the issue without a branch: for a second task closed by the same PR, or a task with no code.
- `${story,,}` lower-cases `TODO-7` to `todo-7` for the branch name.

## Help
- Save with `:w! ~/code/todo/scripts/backlog`.
