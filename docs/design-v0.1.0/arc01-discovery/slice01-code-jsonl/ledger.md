# Slice 01: code-jsonl

## Ledger

_CC fills Status + Evidence (strength `attested`) as work lands. CDC reproduces
to `reproduced`. Verify commands run against `~/.claude/projects` in CC's trust
domain._

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | Volumes measured: project-dir count, `.jsonl` count, total bytes | `ls -1d ~/.claude/projects/*/ \| wc -l`; `find … -name '*.jsonl' \| wc -l`; `du -sh ~/.claude/projects` | correctness | slice-doc | done | attested: snapshot 43 dirs / 104 files / 146 MB. **Caveat:** live store — passes read 45/115 then 43/104; ledger's 44/114 sits inside the drift band. Report §1. | counts are a moving snapshot, not a fixed total |
| F-2 | Time-span measured (oldest / newest session file) | `find … -name '*.jsonl' -exec stat -f '%Sm' -t '%Y-%m-%d' {} \; \| sort \| sed -n '1p;$p'` | polish | slice-doc | done | attested: oldest `2026-05-11`, newest `2026-06-30`. Report §1. | mtime-based |
| F-3 | Union record schema documented + variation across versions noted | `… \| jq -s 'map(keys)\|add\|unique'` over a sample | serious | slice-doc | done | attested: 60-key union; `type` is the only universal key; drift across 10+ `version`s (`2.1.139`…`2.1.195`); 4 variation axes documented. Report §4. | schema drives arc02 |
| F-4 | Record-type distribution documented | `cat … \| jq -r '.type // "NULL"' \| sort \| uniq -c \| sort -rn` | correctness | slice-doc | done | attested: 13 types; assistant 27,092 / user 16,617 … custom-title 4. Two-tier (4 envelope vs 9 sidecar). Report §3. | |
| F-5 | Provenance fields confirmed present: cwd, sessionId, timestamp, model, parentUuid, version | `jq 'has("cwd"),has("sessionId"),…'` sampled per type | serious | slice-doc | done | attested: presence-by-type table. **Caveat:** these 6 are NOT uniform — only assistant/user/attachment/system carry cwd/version/parentUuid; sidecars carry sessionId only; `file-history-snapshot` carries NO sessionId; `model` lives at `message.model` (assistant only). Report §5. | the keys arc02 normalizes on |
| F-6 | Content-block structure documented (text / tool_use / tool_result shapes) | `jq '.message.content \| type, (.[]?.type)'` sampled | serious | slice-doc | done | attested: content is string (human prompt) OR typed-block array; tool_result 13,827 / tool_use 13,821 / text 7,477 / thinking 2,774; mixed-block turns exist. Report §6a. | |
| F-7 | Path-encoding lossiness confirmed; "use record `cwd`" rule recorded | show a dir name containing `--` + the `cwd` from a record inside it | correctness | discovery | done | attested: dir `…lykn-lang--worktrees-cdc-build-dir-reorg` → cwd `/Users/oubiwann/lab/lykn/lang/.worktrees/cdc-build-dir-reorg`; the `.` of `.worktrees` is also dropped. Rule recorded. Report §2. | confirmed unrecoverable from dir name |
| F-8 | `history.jsonl` shape documented | `head -3 ~/.claude/history.jsonl \| jq 'keys'` | polish | slice-doc | done | attested: 4 keys `{display, pastedContents, timestamp(epoch-ms), project}`; 7,382 lines / 2.4 MB; prompts only, no sessionId/replies. Report §7a. | thin global timeline |
| F-9 | (stretch) IndexedDB peek: are Desktop transcripts cached locally? | `strings`/ldb dump of `…/IndexedDB/https_claude.ai_0.indexeddb.leveldb` | polish | discovery | done | attested: leveldb holds only `tipTapEditorState` compose-box drafts; NO transcript/sender/conversation records. **Answer: transcripts are NOT locally cached** → Desktop export (slice02) is required. Report §7b. | question answered NO; not deferred |
| F-10 | Findings written to `workbench/2026.06.30-discovery-code-jsonl.md` with reproducible command blocks + ≥1 redacted example per record type | file exists; contains command blocks + examples | serious | slice-doc | done | attested: file exists; all 13 record types have a redacted example (§7); command blocks throughout; F-1…F-9 cross-referenced. | the artifact CDC reads |

## What Worked

- **`jq` over the whole corpus, not a sample, where it was cheap.** At 146 MB the
  full `find … -print0 | xargs -0 cat | jq` passes ran in seconds, so F-3/F-4/F-6
  used the *entire* corpus rather than the `head -50` sample the recon sketched —
  no sampling caveat needed for the distributions.
- **Per-`(type, key-set)` grouping** (`jq -r 'select(.type) | "\(.type)\t\(keys|join(","))"' | sort | uniq -c`)
  surfaced the schema-drift axes mechanically instead of by eyeball — it is what
  exposed that the same `type` legitimately has multiple key-sets.
- **Chasing the F-1 count discrepancy instead of papering over it** turned a
  "numbers don't match the ledger" annoyance into the live-corpus finding.
- **Running F-9 to a real answer** (strings-dumping the leveldb) rather than
  deferring the stretch row converted a "maybe later" into a load-bearing NO that
  de-risks the slice02 plan.

## Closure

Closed at commits `ca4e950` (ledger close) / `6066b4b` (CDC verification) on 2026-06-30. Verified by: CDC (`cdc-verification.md`, 2026-06-30).
Rows: 10. Done: 10. Deferred: 0. No-op: 0.
Evidence strength: F-1 / F-7 / F-8 / F-10 **reproduced** by CDC (independent: operator inventory + mounted artifact); F-2–F-6 / F-9 **attested** (consistency-checked — full re-run blocked by trust-domain split; needs fresh CC context or operator).
CDC gate: **slice01 CLOSED**; arc ledger A-1 → done.
