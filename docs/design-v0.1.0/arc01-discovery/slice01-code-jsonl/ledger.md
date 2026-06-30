# Slice 01: code-jsonl

## Ledger

_CC fills Status + Evidence (strength `attested`) as work lands. CDC reproduces
to `reproduced`. Verify commands run against `~/.claude/projects` in CC's trust
domain._

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | Volumes measured: project-dir count, `.jsonl` count, total bytes | `ls -1d ~/.claude/projects/*/ \| wc -l`; `find … -name '*.jsonl' \| wc -l`; `du -sh ~/.claude/projects` | correctness | slice-doc | open | | inventory says 44 dirs / 114 files — confirm |
| F-2 | Time-span measured (oldest / newest session file) | `find … -name '*.jsonl' -exec stat -f '%Sm' -t '%Y-%m-%d' {} \; \| sort \| sed -n '1p;$p'` | polish | slice-doc | open | | |
| F-3 | Union record schema documented + variation across versions noted | `… \| jq -s 'map(keys)\|add\|unique'` over a sample | serious | slice-doc | open | | schema drives arc02 |
| F-4 | Record-type distribution documented | `cat … \| jq -r '.type // "NULL"' \| sort \| uniq -c \| sort -rn` | correctness | slice-doc | open | | |
| F-5 | Provenance fields confirmed present: cwd, sessionId, timestamp, model, parentUuid, version | `jq 'has("cwd"),has("sessionId"),…'` sampled per type | serious | slice-doc | open | | the keys arc02 normalizes on |
| F-6 | Content-block structure documented (text / tool_use / tool_result shapes) | `jq '.message.content \| type, (.[]?.type)'` sampled | serious | slice-doc | open | | |
| F-7 | Path-encoding lossiness confirmed; "use record `cwd`" rule recorded | show a dir name containing `--` + the `cwd` from a record inside it | correctness | discovery | open | | e.g. `…lykn-lang--worktrees…` |
| F-8 | `history.jsonl` shape documented | `head -3 ~/.claude/history.jsonl \| jq 'keys'` | polish | slice-doc | open | | thin global timeline |
| F-9 | (stretch) IndexedDB peek: are Desktop transcripts cached locally? | `strings`/ldb dump of `…/IndexedDB/https_claude.ai_0.indexeddb.leveldb` | polish | discovery | open | | deferrable; low priority |
| F-10 | Findings written to `workbench/2026.06.30-discovery-code-jsonl.md` with reproducible command blocks + ≥1 redacted example per record type | file exists; contains command blocks + examples | serious | slice-doc | open | | the artifact CDC reads |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

Closed at commit <SHA> on <date>. Verified by: <name/session>.
Rows: 10. Done: <n>. Deferred: <n>. No-op: <n>.
