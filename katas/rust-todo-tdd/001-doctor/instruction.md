# Check the host — scripts/doctor

**File:** `scripts/doctor`  
**Save to:** `~/code/todo/scripts/doctor`

## Goal
Type `scripts/doctor`: one script that tells you what is missing on the Arch host before anything fails halfway. It only reports; it never installs.

## Run first
```sh
$ mkdir -p ~/code/todo/scripts && cd ~/code/todo
```

## Why
- Items needed from the start fail the run (`✗`). Items for later chapters (kubectl, k3d, helm, k9s, sops, age) only warn (`!`) and name the chapter that needs them.
- `need now|later <command> <how to get it>` is the whole pattern; `--all` turns every warning into a failure. Run it with `--all` before Chapter 12.
- `at_least` compares versions with `sort -V`: the smaller of the two sorts first.
- `set -uo pipefail` without `-e`: the script must keep going after a failed check, so it can list everything at once.

## Check
The script runs and prints a report. Fix every `✗` line it shows, then run it again.

## When it passes
```sh
$ chmod +x scripts/doctor
$ scripts/doctor
```
It is committed with the rest of Part 0 (kata 011), so it travels with the repository. Re-run it at the start of any chapter.

## Help
- Save with `:w! ~/code/todo/scripts/doctor`.
