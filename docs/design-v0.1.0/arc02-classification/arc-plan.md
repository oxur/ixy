# arc02 — classification (arc plan)

> **The first implementation arc** — real Rust lands in `ixy-core`. CC implements,
> CDC plans + verifies; **rust-guidelines** applies (types-first, `thiserror`,
> `#[non_exhaustive]`, redacted `Debug` on any secret/PII-bearing field). Planned
> against `../arc01-discovery/corpus-map.md` (the schema source of record) and
> `../design-doc.md`. **Status:** active, slice breakdown at one-line altitude;
> per-slice open sets written when each slice becomes active (*plan late, plan deep*).

## Capability

A single **normalized record model** that reconciles Claude Code + Claude Desktop
into one uniform representation, plus a **classifier/normalizer** that transforms a
raw record from *either* source into it — so everything downstream (arc03 analysis,
arc04 storage, arc05 indices) builds on one record, not two source schemas. Includes
the sparse-union **provenance** model, the **identity + dedup** keys, and the
**monotonic-merge** semantics (a conversation gone from a newer export is kept, not
deleted). Output: Rust types + normalizers in `ixy-core`, with `raw_ref` back to
bedrock so normalization is never lossy.

## Slice breakdown (dependency-ordered, one-line scope)

| Slice | Scope | Load-bearing for | Open questions (resolve in slice-doc) |
|-------|-------|------------------|----------------------------------------|
| **slice01 — record model** | `NormalizedMessage` + `Block` (text/thinking/tool_use/tool_result/meta) + `Role` + `Provenance` (sparse) + `Thread` types in `ixy-core`; invariants + tests. | 02–05 | secret/PII redaction in `Debug` (API-18); `#[non_exhaustive]` on the block/role enums |
| **slice02 — Code normalizer** | Code JSONL record → `NormalizedMessage`; disposition all 13 types (message vs metadata vs drop); threading (`uuid`/`parentUuid` + `logicalParentUuid`). | slice04, arc03 | which sidecar types become metadata vs dropped |
| **slice03 — Desktop normalizer** | Desktop conversation/message → `NormalizedMessage`; `sender`→role, content-array→blocks, **sentinel-root** detection, tree/branch handling. | slice04, arc03 | **canonical reading path** for branch trees (latest-leaf? keep all branches?) |
| **slice04 — identity + dedup + monotonic merge** | id scheme (source + uuid, no cross-source collision); superset-merge on `(uuid, updated_at)`; monotonic-across-exports (deletion inversion vs odm). | arc03, arc04 | represent source in the id; merge conflict resolution |
| **slice05 — non-message elements** | Projects, Memories, Code sidecars (`history.jsonl`, titles) — model as normalized entities or exclude, with rationale. | arc03 | index-or-exclude each, with rationale |

Sizing: each is one-context (types; per-source normalizer; dedup logic; element
modeling). slice02/03 are symmetric (one per source) and share slice01's types.

## Dependencies

- **Consumes:** `arc01-discovery/corpus-map.md` (the normalized-record sketch §3b, the
  provenance table §3a, the per-source schemas §2, the open questions §4).
- **Leaves for later:** arc03 (analysis) gets a real normalized corpus to profile;
  arc04 (storage) gets the record model to persist; arc05 gets uniform records to index.

## Arc Ledger

> Opens now with class-(b) composition rows from the capability; accrues class-(a)
> slice-closed + class-(c) bubble-up rows as slices close. Class-(b) rows are
> **reproduced at arc scale** — an end-to-end normalize demonstration on real sample
> records, never inherited from the slices. Closes in `arc02-classification/closing-report.md`.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| B-1 | slice01 (record model) closed | ptr: slice01 cdc-verification | correctness | arc-plan | open | | attested |
| B-2 | slice02 (Code normalizer) closed | ptr: slice02 cdc-verification | correctness | arc-plan | open | | attested |
| B-3 | slice03 (Desktop normalizer) closed | ptr: slice03 cdc-verification | correctness | arc-plan | open | | attested |
| B-4 | slice04 (identity/dedup/merge) closed | ptr: slice04 cdc-verification | correctness | arc-plan | open | | attested |
| B-5 | slice05 (non-message elements) closed | ptr: slice05 cdc-verification | correctness | arc-plan | open | | attested |
| B-6 | **Compose:** a real Code record AND a real Desktop message both normalize to the same `NormalizedMessage` type end-to-end | arc-scale demo: normalize sample records from each source | serious | arc-plan | open | | reproduce at arc scale |
| B-7 | **Compose:** provenance sparse-union is faithful per source (Code fills model/cwd/version; Desktop fills account; each leaves the other's fields `None`) | arc-scale demo: inspect normalized provenance from each source | serious | arc-plan | open | | reproduce at arc scale |
| B-8 | **Compose:** dedup/monotonic-merge — an overlapping re-export yields a superset-merge (no dupes, no deletions) | arc-scale demo: normalize two overlapping exports; assert merge | serious | arc-plan | open | | reproduce at arc scale |
| B-9 | **Compose:** normalization is non-lossy — every record carries a `raw_ref`; bedrock is recoverable | arc-scale demo: round-trip a sample record to its raw bedrock | serious | arc-plan | open | | reproduce at arc scale |
| B-10 | **Compose:** the full corpus-map taxonomy is dispositioned (all 13 Code types, all Desktop block types, sentinel roots, branches) | arc-scale: coverage check vs corpus-map §2 | serious | arc-plan | open | | reproduce at arc scale |
| B-11 | bubble-up findings dispositioned | ptr: arc-plan change-log | correctness | bubble-up | open | | accrues as slices close |

## Method

Ledger per slice; CC implements in Rust (rust-guidelines), CDC verifies every row;
cargo rows (`fmt`/`clippy -D warnings`/`test`) via CI or local 1.85+. Five-iteration
cap. Slice closes bubble up here (Part IV); the arc closes with its own
`closing-report.md` + composition check + independent gate (Part V).

## Version History

### v1.0 — 2026-06-30
Initial arc plan, drafted from `corpus-map.md` on arc02 activation. Five slices
(record model → Code normalizer → Desktop normalizer → identity/dedup/merge →
non-message elements); arc ledger opened (B-1…B-11). Breakdown at one-line altitude
per *plan late, plan deep*. **Proposed — pending operator confirmation of the
decomposition + which slice to start.**
