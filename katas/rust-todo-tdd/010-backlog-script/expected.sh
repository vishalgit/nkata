#!/usr/bin/env bash
# Create GitHub issues just in time: an epic before its first story, a story when its card
# appears, a task when you start it. Each lands on the board with its fields and parent link.
#
#   scripts/backlog epic  E1 "Manage my todos" "Add, see, complete, rename and delete todos"
#   scripts/backlog story TODO-7 E1 story 5 "Add a todo"          # body: backlog/stories/TODO-7.md
#   scripts/backlog task  TODO-7 3 domain "Typed ids and the constrained title (3.2–3.11)" ids-and-title
#   TASK_ONLY=1 scripts/backlog task TODO-4 5 infra "Migrations and offline query data in the pipeline (5.15)"
#
# `task` also creates its chapter's milestone if it is new, moves the issue to In progress and
# checks out its branch todo-<n>/<issue>-<slug> from an up-to-date main (unless TASK_ONLY=1).
set -euo pipefail
cd "$(dirname "$0")/.."
REPO="${REPO:-$(gh repo view --json nameWithOwner --jq .nameWithOwner)}"
OWNER="${REPO%%/*}"
CHAPTERS=("Workbench" "Walking skeleton" "Domain types" "Workflows and query" "Persistence" "Identity"
  "Server functions" "UI and 7-1 SCSS" "Slices and list UI" "Authorization" "E2E journeys" "Hardening")

PNUM=$(gh project list --owner "$OWNER" --format json | jq -r '.projects[] | select(.title=="Todo book") | .number')
PID=$(gh project view "$PNUM" --owner "$OWNER" --format json --jq .id)
FIELDS=$(gh project field-list "$PNUM" --owner "$OWNER" --format json)
fid() { jq -r --arg n "$1" '.fields[] | select(.name==$n) | .id' <<<"$FIELDS"; }
opt() { jq -r --arg n "$1" --arg o "$2" '.fields[] | select(.name==$n) | .options[]? | select(.name==$o) | .id' <<<"$FIELDS"; }

# Number of the issue whose title starts with "<key> · ", if any.
find_issue() {
  gh issue list --repo "$REPO" --state all --limit 500 --search "\"$1 ·\" in:title" --json number,title \
    | jq -r --arg p "$1 · " '[.[] | select(.title | startswith($p))][0].number // empty'
}
add_item()   { gh project item-add "$PNUM" --owner "$OWNER" --url "$1" --format json --jq .id; }
set_select() { local o; o=$(opt "$2" "$3")
               if [ -n "$o" ]; then gh project item-edit --project-id "$PID" --id "$1" \
                 --field-id "$(fid "$2")" --single-select-option-id "$o" >/dev/null; fi; }
set_num()    { gh project item-edit --project-id "$PID" --id "$1" --field-id "$(fid "$2")" --number "$3" >/dev/null; }
sub_issue()  { gh api -X POST "repos/$REPO/issues/$1/sub_issues" \
                 -F sub_issue_id="$(gh api "repos/$REPO/issues/$2" --jq .id)" >/dev/null; }

epic() {
  local key=$1 title=$2 goal=$3 url
  if [ -n "$(find_issue "$key")" ]; then echo "$key already exists"; return; fi
  gh label create "epic:$key" --color EDEDED --description "Epic $key" --repo "$REPO" --force >/dev/null
  url=$(gh issue create --repo "$REPO" --title "$key · $title" --body "**Goal:** $goal" --label "type:epic,epic:$key")
  set_select "$(add_item "$url")" Epic "$key"
  echo "$url"
}

story() {
  local key=$1 epic=$2 type=$3 points=$4 title=$5 parent body url item
  if [ -n "$(find_issue "$key")" ]; then echo "$key already exists"; return; fi
  parent=$(find_issue "$epic")
  [ -n "$parent" ] || { echo "no issue for $epic yet: run scripts/backlog epic first" >&2; exit 1; }
  body="backlog/stories/$key.md"
  if [ ! -f "$body" ]; then body=$(mktemp); printf 'Story card: the build guide, %s.\n' "$key" > "$body"; fi
  url=$(gh issue create --repo "$REPO" --title "$key · $title" --body-file "$body" --label "type:$type,epic:$epic")
  sub_issue "$parent" "${url##*/}"
  item=$(add_item "$url"); set_select "$item" Epic "$epic"; set_num "$item" Points "$points"
  echo "$url"
}

task() {
  local story=$1 chapter=$2 layer=$3 title=$4 slug=${5:-} parent epic milestone url n item
  parent=$(find_issue "$story")
  [ -n "$parent" ] || { echo "no issue for $story yet: run scripts/backlog story first" >&2; exit 1; }
  epic=$(gh issue view "$parent" --repo "$REPO" --json labels --jq '.labels[].name | select(startswith("epic:"))')
  epic=${epic#epic:}
  milestone=$(printf 'Ch %02d · %s' "$chapter" "${CHAPTERS[$((chapter - 1))]}")
  if ! gh api "repos/$REPO/milestones?state=all&per_page=100" --jq '.[].title' | grep -qxF "$milestone"; then
    gh api "repos/$REPO/milestones" -f title="$milestone" >/dev/null
  fi
  url=$(gh issue create --repo "$REPO" --title "$story · Ch $chapter · $title" \
    --body "Part of #$parent. Acceptance criteria are on the story." \
    --label "type:task,layer:$layer,epic:$epic" --milestone "$milestone")
  n=${url##*/}
  sub_issue "$parent" "$n"
  item=$(add_item "$url"); set_select "$item" Epic "$epic"; set_num "$item" Chapter "$chapter"
  echo "$url"
  if [ "${TASK_ONLY:-0}" != 1 ]; then
    set_select "$item" Status "In progress"
    git switch main && git pull --ff-only
    gh issue develop "$n" --repo "$REPO" --name "${story,,}/$n-${slug:?a branch slug is needed}" --checkout
  fi
}

cmd=${1:?usage: scripts/backlog epic|story|task ...}; shift
case "$cmd" in
  epic|story|task) "$cmd" "$@" ;;
  *) echo "unknown command: $cmd" >&2; exit 2 ;;
esac
