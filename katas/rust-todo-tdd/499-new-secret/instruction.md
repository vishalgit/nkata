# Green — bin/new-secret

**File:** `bin/new-secret`  
**Save to:** `~/code/todo/bin/new-secret`

## Goal
Type the script that pipes a generated Secret straight into SOPS, so plaintext never touches the disk.

## Why
- `--filename-override` lets SOPS apply the `.sops.yaml` rule for the target path while reading from stdin.
- Hex passwords need no escaping inside `DATABASE_URL`.
- It refuses to overwrite: Postgres reads `POSTGRES_PASSWORD` only when its volume is first created, so a new password alone would lock the app out.

**Rotating** takes two steps: `ALTER ROLE todo PASSWORD '<new>'` in the database first (`kubectl -n todo-prod exec statefulset/postgres -- psql -U todo -d todo -c …`), then the same value in both keys with `sops`, commit, and restart the app pods after the next promotion. **If the age key ever leaks:** new key, new public key in `.sops.yaml`, `sops updatekeys` on each file, and rotate every password — Git history still holds the old ciphertext.

Never paste a decrypted value into an issue, a PR or an approval comment.

## When it passes
```sh
$ chmod +x bin/new-secret
$ for env in dev test prod; do bin/new-secret "$env"; done
$ bin/check-secrets           # ok dev / ok test / ok prod
$ grep POSTGRES_PASSWORD deploy/k8s/overlays/prod/secret.enc.yaml    # ENC[AES256_GCM,data:...]
```
`sops deploy/k8s/overlays/prod/secret.enc.yaml` opens the decrypted file in Neovim and encrypts it again on save.

## Help
- Save with `:w! ~/code/todo/bin/new-secret`.
