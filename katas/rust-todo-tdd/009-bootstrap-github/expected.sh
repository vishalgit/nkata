#!/usr/bin/env bash
# One-time GitHub setup: repository settings, the fixed labels, the project and its fields.
# Epics, stories, tasks, epic labels and milestones are created later, just in time, by scripts/backlog.
set -euo pipefail
REPO="${REPO:-$(gh repo view --json nameWithOwner --jq .nameWithOwner)}"
OWNER="${REPO%%/*}"

# --- repository settings: squash only, PR title as commit, no auto-merge, delete merged branches
gh repo edit "$REPO" --enable-auto-merge=false --delete-branch-on-merge \
  --enable-squash-merge --enable-merge-commit=false --enable-rebase-merge=false
gh api -X PATCH "repos/$REPO" -f squash_merge_commit_title=PR_TITLE \
  -f squash_merge_commit_message=PR_BODY >/dev/null

# --- the labels every issue and PR draws on
label() { gh label create "$1" --color "$2" --description "$3" --repo "$REPO" --force >/dev/null; }
label type:epic    5319E7 "A group of stories with one goal"
label type:story   1D76DB "User-visible capability"
label type:enabler 0E8A16 "Developer or operator capability"
label type:task    C5DEF5 "One book step, one PR"
label type:bug     D73A4A "Behaviour contradicts an acceptance criterion"
label type:chore   BFBFBF "Upkeep with no behaviour change"
label type:spike   FBCA04 "Time-boxed investigation"
label breaking     B60205 "Changes a migration, URL or API contract"
label dependencies 0366D6 "Dependency updates"
for l in domain persistence http ui e2e infra observability; do label "layer:$l" F9D0C4 "Works in the $l layer"; done

# --- the project and its fields (empty: items arrive with each issue)
PNUM=$(gh project create --owner "$OWNER" --title "Todo book" --format json --jq .number)
gh project link "$PNUM" --owner "$OWNER" --repo "$REPO"
gh project field-create "$PNUM" --owner "$OWNER" --name Epic --data-type SINGLE_SELECT \
  --single-select-options "E0,E1,E2,E3,E4,E5,E6" >/dev/null
gh project field-create "$PNUM" --owner "$OWNER" --name Points  --data-type NUMBER >/dev/null
gh project field-create "$PNUM" --owner "$OWNER" --name Chapter --data-type NUMBER >/dev/null

echo "Done. Finish in the web UI: Status options, Iteration field, views and workflows."
