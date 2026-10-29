# The key — .sops.yaml

**File:** `.sops.yaml`  
**Save to:** `~/code/todo/.sops.yaml`

## Goal
Install the tools, make an age key, and record its **public** half in `.sops.yaml`: which files, which fields, for whom.

## Run first
```sh
$ sudo pacman -S sops age
$ mkdir -p ~/.config/sops/age
$ age-keygen -o ~/.config/sops/age/keys.txt && chmod 600 ~/.config/sops/age/keys.txt
```

## Why
- `path_regex` picks the files; `encrypted_regex` encrypts only `data`/`stringData`, so `kind` and `metadata` stay readable in diffs.
- Copy `keys.txt` into your password manager now. Without it nobody can decrypt the files again.

## Help
- The kata expects the placeholder `AGE_PUBLIC_KEY`. Once it passes, swap in your real public key and save:
  `:%s#AGE_PUBLIC_KEY#\=trim(system('age-keygen -y ~/.config/sops/age/keys.txt'))#` (with `#` as the delimiter, the slashes in the path need no escaping).
- Save with `:w! ~/code/todo/.sops.yaml`.
