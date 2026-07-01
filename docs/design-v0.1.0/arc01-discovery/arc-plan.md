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
| A-3 | slice03 (corpus-map) closed cleanly | ptr: slice03 cdc-verification | correctness | arc-plan | open | | attested |
| A-4 | corpus map documents BOTH sources' record schemas + content-block shapes | read map; cross-check against slice findings | serious | arc-plan | open | | reproduce at arc scale |
| A-5 | every DoD provenance dimension (account/machine/model/time/project/session) traces to a concrete field in ≥1 source | walk each dimension → field in the map | serious | arc-plan | open | | reproduce at arc scale |
| A-6 | arc02 normalized schema is draftable citing ONLY the corpus map (no re-spelunking) | demonstration: draft schema skeleton from map alone | serious | arc-plan | open | | reproduce at arc scale |

## Version History

### v1.0 — 2026-06-30
Initial arc plan. Three slices (code-jsonl, desktop-export, corpus-map). Arc
ledger opened (A-1..A-6). slice01 open set authored; slices 02–03 listed but not
yet detailed (plan late, plan deep).
