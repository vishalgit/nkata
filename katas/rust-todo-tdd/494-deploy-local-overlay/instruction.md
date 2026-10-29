# bin/deploy uses the local overlay

**File:** `bin/deploy` (part 2 of 2)  
**Save to:** `~/code/todo/bin/deploy`

## Goal
Point `bin/deploy` at `deploy/k8s/overlays/local`.

## Check
```sh
$ bin/check-overlays          # ok: dev, test and prod render
$ bin/deploy                  # ok <- deployed todo:<sha>, exactly as before
```

## Help
- Save with `:w! ~/code/todo/bin/deploy`.
