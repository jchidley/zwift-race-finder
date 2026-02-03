#!/bin/bash
# Refresh ZwiftPower stats via Playwright browser session state (no stored session ID)

set -euo pipefail

TOOLS_DIR="${HOME}/tools/agent-skills/skills/browser-tools"
STATE_FILE="${ZRF_ZP_STATE_FILE:-${XDG_CACHE_HOME:-$HOME/.cache}/zwift-race-finder/zwiftpower-auth.json}"
CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/zwift-race-finder"
CACHE_FILE="${CACHE_DIR}/user_stats.json"
PROFILE_ID="${1:-${ZWIFTPOWER_PROFILE_ID:-}}"

if [[ -z "$PROFILE_ID" ]]; then
  echo "ZWIFTPOWER_PROFILE_ID is not set" >&2
  exit 1
fi

mkdir -p "$CACHE_DIR"
chmod 700 "$CACHE_DIR"

if [[ ! -f "$STATE_FILE" ]]; then
  echo "No Playwright auth state found."
  echo "Opening browser to ZwiftPower. Please log in, then press Enter here."

  node "${TOOLS_DIR}/browser-start.js" --profile >/dev/null
  node "${TOOLS_DIR}/browser-nav.js" "https://zwiftpower.com/profile.php?z=${PROFILE_ID}" --new --load >/dev/null

  read -r -p "Press Enter after you are logged in: " _

  node "${TOOLS_DIR}/browser-auth.js" --save "$STATE_FILE" >/dev/null
  chmod 600 "$STATE_FILE"
fi

node "${TOOLS_DIR}/browser-auth.js" --load "$STATE_FILE" "https://zwiftpower.com/profile.php?z=${PROFILE_ID}" >/dev/null

stats_json=$(
  node "${TOOLS_DIR}/browser-eval.js" '
(() => {
  const text = document.body.innerText || "";
  const scoreMatch = text.match(/Zwift Racing Score\s+(\d+)/);
  const catMatch = text.match(/Category(?:\s*\(.*?\))?\s+([A-E]\+?)/);
  const title = document.title || "";
  const nameMatch = title.match(/^ZwiftPower\s*-\s*(.+)$/);

  const score = scoreMatch ? parseInt(scoreMatch[1], 10) : null;
  const category = catMatch ? catMatch[1] : null;
  const username = nameMatch ? nameMatch[1].trim() : "ZwiftPower";

  return JSON.stringify({ score, category, username });
})()
' 2>/dev/null
)

python - <<PY
import json, os, sys
from datetime import datetime, timezone

raw = ${stats_json!r}
data = json.loads(raw)

score = data.get("score")
if score is None:
    print("Failed to parse Zwift Racing Score from page", file=sys.stderr)
    sys.exit(1)

category = data.get("category") or None
username = data.get("username") or "ZwiftPower"

payload = {
    "stats": {
        "zwift_score": int(score),
        "category": category if category else "",
        "username": username,
    },
    "cached_at": datetime.now(timezone.utc).isoformat(),
}

cache_file = "${CACHE_FILE}"
with open(cache_file, "w", encoding="utf-8") as f:
    json.dump(payload, f, indent=2)

print(f"Wrote {cache_file}")
PY
