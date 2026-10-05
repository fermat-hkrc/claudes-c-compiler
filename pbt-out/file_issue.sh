#!/usr/bin/env bash
# 用法: file_issue.sh <title-file> <body-file>  → 输出新 issue URL
set -euo pipefail
TOKEN=$(grep "github.com" ~/.git-credentials | head -1 | sed -E 's#https://[^:]+:([^@]+)@.*#\1#')
TITLE=$(cat "$1")
BODY=$(cat "$2")
python3 - "$TOKEN" "$TITLE" "$BODY" <<'PY'
import json, sys, urllib.request
token, title, body = sys.argv[1], sys.argv[2], sys.argv[3]
req = urllib.request.Request(
    'https://api.github.com/repos/fermat-hkrc/claudes-c-compiler/issues',
    data=json.dumps({'title': title, 'body': body}).encode(),
    headers={'Authorization': f'token {token}', 'Accept': 'application/vnd.github+json'},
    method='POST')
with urllib.request.urlopen(req) as r:
    d = json.load(r)
print(d['html_url'])
PY
