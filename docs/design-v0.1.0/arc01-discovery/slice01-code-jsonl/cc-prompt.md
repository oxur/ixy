# CC assignment — slice01: code-jsonl

You are **CC** (Claude Code) in the IC implementation seat for the `ixy` project.
Read `CLAUDE.md` and load the **collaboration-framework**; this slice runs under
**ledger discipline** (`LEDGER-DISCIPLINE.md`). CDC (a Cowork session) will
independently verify your close.

## Why you and not CDC

The corpus lives under `~/.claude` and `~/Library/Application Support/Claude` —
protected directories the Cowork session **cannot mount**. You have full
filesystem access in the operator's trust domain. You are the only party that can
run this pass. Your output file in `workbench/` is the handoff channel back to CDC.

## The job

Characterize the **Claude Code JSONL corpus** on this machine
(`~/.claude/projects/**/*.jsonl` + `~/.claude/history.jsonl`) and write your
findings to:

```
workbench/2026.06.30-discovery-code-jsonl.md
```

Work against `ledger.md` in this directory — it is the spec of "done." Read it
first. Each row's **Verify** column is the command to run. Fill Status +
Evidence (strength `attested`) as you go; do not batch all evidence to the end.

## Recon commands (starting point — adapt as the data demands)

```bash
# F-1 volumes
echo "project dirs: $(ls -1d ~/.claude/projects/*/ 2>/dev/null | wc -l)"
echo "session files: $(find ~/.claude/projects -name '*.jsonl' | wc -l)"
du -sh ~/.claude/projects

# F-2 time-span
find ~/.claude/projects -name '*.jsonl' -exec stat -f '%Sm' -t '%Y-%m-%d' {} \; | sort | sed -n '1p;$p'

# F-3 union schema across a sample (jsonl = one JSON object per line)
find ~/.claude/projects -name '*.jsonl' | head -50 \
  | xargs -I{} sh -c 'head -20 "{}"' \
  | jq -s 'map(keys) | add | unique'

# F-4 record-type distribution (sample if the corpus is multi-GB)
find ~/.claude/projects -name '*.jsonl' -print0 | xargs -0 cat \
  | jq -r '.type // "NULL"' | sort | uniq -c | sort -rn

# F-5 provenance fields — presence by type (adapt key paths to real schema)
find ~/.claude/projects -name '*.jsonl' | head -5 | xargs cat \
  | jq -c '{type, cwd, sessionId, timestamp, model:(.message.model // .model), parentUuid, version}' | head

# F-6 content-block structure
find ~/.claude/projects -name '*.jsonl' | head -5 | xargs cat \
  | jq -c 'select(.message.content) | (.message.content | if type=="array" then map(.type) else type end)' | sort | uniq -c

# F-7 path-encoding lossiness: find a dir with "--", show a record's cwd
ls -1 ~/.claude/projects | grep -- '--' | head -1
# then: cat one of its *.jsonl | jq -r '.cwd' | head -1

# F-8 history.jsonl
head -3 ~/.claude/history.jsonl | jq 'keys'

# F-9 (stretch, low priority) IndexedDB peek
strings "$HOME/Library/Application Support/Claude/IndexedDB/https_claude.ai_0.indexeddb.leveldb/"*.ldb 2>/dev/null | grep -iE 'human|assistant|conversation' | head
```

## Reporting rules (ledger discipline)

1. **Per-row walk** in your `closing-report.md` — for every F-row, state final
   status (`done`/`deferred`/`no-op`) + evidence (the command + its output). No
   prose summaries, no "deviations: none."
2. **Name uncertainty.** "done with caveat X" beats a confident "done" that was
   softpedalled.
3. **Redaction is yours to judge.** You report to the operator first; include ≥1
   *redacted* example per record type (structure visible, private content elided).
4. **No silent drops.** Close with all 10 rows dispositioned; F-9 may be
   `deferred` (low-priority stretch) with a re-entry condition.
5. **Bubble-up to the arc.** End with the three bubble-up questions from
   `PROJECT-MANAGEMENT.md` Part IV: did this slice deliver its assigned piece of
   arc01; what did it reveal the arc-plan didn't anticipate (e.g. unexpected
   record types, schema drift, extra provenance fields); the silent-drop diff.

## Out of scope

Desktop export (slice02), the normalized schema design (arc02), any Rust/ingest
code, deep dives into `tasks/` / `plans/` / `file-history/` (note relevance only).
