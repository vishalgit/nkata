# Story card TODO-7 · Add a todo

**File:** `backlog/stories/TODO-7.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-7.md`

## Goal
Write the card for **TODO-7 · Add a todo** (E1 Manage my todos, 5 points).

Chapter 3 turns the todo vocabulary into types — ids, a constrained title, one type per lifecycle state — one test at a time, with no IO anywhere. The domain crate starts with zero dependencies; each arrives when a test cannot compile without it.

## *Domain Modeling Made Functional* in Rust
| DMMF idea | F# | Rust here |
| --- | --- | --- |
| Record (AND type) | `type T = { A: X; B: Y }` | `struct T { a: X, b: Y }` |
| Choice (OR type) | `type T = A \| B of X` | `enum T { A, B(X) }` |
| Simple constrained type | single-case union + private ctor | newtype with a private field + `parse` |
| Smart constructor | `create : string -> Result<T, E>` | `fn parse(raw) -> Result<T, E>` |
| State as types | one type per state | one struct per state, an `enum` to hold any |
| State transition | `ActiveTodo -> CompletedTodo` | `fn complete(self, ..) -> CompletedTodo` (consumes `self`) |
| Workflow | `Command -> Result<Event, Error>` | `fn(Command, deps) -> Result<Event, Error>` |
| Dependencies | function parameters | plain values and closures as parameters |

Rust adds **ownership**: a transition that takes `self` by value makes the old state unusable after the call.

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-7.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog epic E1 "Manage my todos" "Add, see, complete, rename and delete todos"
$ scripts/backlog story TODO-7 E1 story 5 "Add a todo"
```

A new epic starts here, so its issue comes first.

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-7.md`.
