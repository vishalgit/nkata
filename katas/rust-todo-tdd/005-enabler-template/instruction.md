# The enabler issue template

**File:** `.github/ISSUE_TEMPLATE/enabler.yml`  
**Save to:** `~/code/todo/.github/ISSUE_TEMPLATE/enabler.yml`

## Goal
The enabler form is the story form with four changes: its name, its description, the story prompt and the label.

## Why
Enablers are work the developer or operator gains (a pipeline, a dashboard); stories are work a user can do. Same shape, different label, so the Backlog view can tell them apart.

## Help
- Rather than typing it, you can fill the buffer from the story form: `:%d` then `:r !sed -e 's/^name: User story/name: Enabler/' -e 's/a user or operator can do/the developer or operator gains/' -e 's/"As a … I want/"As the developer … I want/' -e 's/type:story/type:enabler/' .github/ISSUE_TEMPLATE/story.yml`, then delete the empty first line.
- Save with `:w! ~/code/todo/.github/ISSUE_TEMPLATE/enabler.yml`.
