# arc01 — discovery (arc plan)

## Capability statement

A **complete, reproduced map of the corpus**: every source enumerated, each
source's on-disk / export format and record schema documented, volumes and
time-span measured, and the provenance fields identified — sufficient to design
the normalized record model (arc02) *without further spelunking*. The arc
delivers knowledge, not code: its artifact is the corpus map plus throwaway
recon scripts in `workbench/`.

## Slice breakdown

| Slice | Scope (one line) | Load-bearing for | Status |
|-------|------------------|------------------|--------|
| **slice01 — code-jsonl** | Characterize the Claude Code JSONL corpus (schema, record types, provenance, content blocks, volumes) on this machine. | slice03, arc02 | open set written; ready for CC |
| **slice02 — desktop-export** | Characterize a Claude Desktop account export (archive structure, conversation/message schema, provenance). | slice03, arc02 | blocked: needs an export download |
| **slice03 — corpus-map** | Synthesize 01+02 into one corpus-map doc the arc02 schema is written against. | arc02 | blocked: needs 01 + 02 |

Sizing: each slice is comfortably one-context. slice01 is a CC content pass
against a known directory; slice02 is the same against a downloaded archive;
slice03 is synthesis (CDC-led).

## Dependencies

- **Consumes:** the directory inventories already in `workbench/`
  (`dot-claude-dir-inventory.txt`, `claude-app-supp-dir-inventory.txt`) and the
  structural findings (`2026.06.30-discovery-corpus-structure.md`).
- **Leaves for arc02:** the normalized-schema design keys off slice03's map —
  especially the rule that `cwd` (from records), not the dir name, is the project
  provenance key.

## Arc ledger

_Capability above. Opens here; closes (per-row walk) in
`arc01-discovery/closing-report.md`. Class-(b) composition rows are reproduced at
arc scale, never inherited._

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| A-1 | slice01 (code-jsonl) closed cleanly | ptr: slice01 cdc-verification | correctness | arc-plan | done | slice01/cdc-verification.md — gated closed 2026-06-30 | attested |
| A-2 | slice02 (desktop-export) closed cleanly | ptr: slice02 cdc-verification | correctness | arc-plan | done | slice02/cdc-verification.md — gated closed 2026-06-30 (all 10 rows reproduced) | attested |
| A-3 | slice03 (corpus-map) closed cleanly | ptr: slice03 closing-report | correctness | arc-plan | done | slice03 doer-closed (`slice03-corpus-map/closing-report.md`, 6/6); reproduced at arc gate | gated 2026-06-30 |
| A-4 | corpus map documents BOTH sources' record schemas + content-block shapes | read map; cross-check against slice findings | serious | arc-plan | done | reproduced: `corpus-map.md` §2a/§2b vs slice01/02 findings (subagent gate PASS) | gated 2026-06-30 |
| A-5 | every DoD provenance dimension (account/machine/config-root/model/time/project/session) traces to a concrete field in ≥1 source | walk each dimension → field in the map | serious | arc-plan | done | reproduced: `corpus-map.md` §3a mapping table (subagent gate PASS) | gated 2026-06-30 |
| A-6 | arc02 normalized schema is draftable citing ONLY the corpus map (no re-spelunking) | demonstration: draft schema skeleton from map alone | serious | arc-plan | done | reproduced: `corpus-map.md` §3b NormalizedMessage skeleton (subagent gate PASS) | gated 2026-06-30 |

## Version History

### v1.2 — 2026-06-30
**arc01 CLOSED.** Independent gate PASS WITH NOTES (fresh-context subagent; the one
actionable note — missing per-source volumes — fixed in `corpus-map.md` §1). Operator
(Duncan) signed off. A-1…A-6 → done; project ledger P-1 → done; arc02 (classification)
activated.

### v1.1 — 2026-06-30
slice02 + slice03 closed; arc **proposed-closed pending independent gate**. A-2 done
(CC-verified); A-3 slice03 doer-closed; composition rows A-4/A-5/A-6 reproduced via
`corpus-map.md` §2/§3a/§3b. Arc `closing-report.md` written (composition check +
project bubble-up); 3/3 slices delivered, no silent drops, no arc-plan change. Gate
requested of operator / fresh context.

### v1.0 — 2026-06-30
Initial arc plan. Three slices (code-jsonl, desktop-export, corpus-map). Arc
ledger opened (A-1..A-6). slice01 open set authored; slices 02–03 listed but not
yet detailed (plan late, plan deep).
