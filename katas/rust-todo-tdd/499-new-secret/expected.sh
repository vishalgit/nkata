#!/usr/bin/env bash
# Create an environment's database secret, encrypted with SOPS. Usage: bin/new-secret dev|test|prod
set -euo pipefail
cd "$(dirname "$0")/.."
ENV="${1:?usage: bin/new-secret dev|test|prod}"
FILE="deploy/k8s/overlays/$ENV/secret.enc.yaml"
if [ -e "$FILE" ] && [ "${FORCE:-}" != 1 ]; then
  echo "$FILE exists. Rotating also needs ALTER ROLE (12.4); FORCE=1 overwrites." >&2; exit 1
fi
PW=$(openssl rand -hex 24)          # hex is safe inside DATABASE_URL
cat <<YAML | sops --encrypt --filename-override "$FILE" /dev/stdin > "$FILE.tmp"
apiVersion: v1
kind: Secret
metadata:
  name: postgres
stringData:
  POSTGRES_USER: todo
  POSTGRES_PASSWORD: $PW
  POSTGRES_DB: todo
  DATABASE_URL: postgres://todo:$PW@postgres:5432/todo
YAML
mv "$FILE.tmp" "$FILE"
echo "wrote $FILE"
