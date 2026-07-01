# ixy — project plan (design v0.1.0)

> The plan-of-record: the arc roadmap and the project ledger. This is the plan,
> not the design. Architecture is folded inline (see *Architecture note*) rather
> than carried in a separate design doc — see the deviation note below.

## Definition of done

`ixy` is a **local Rust CLI** that:

1. **Ingests** the full Duncan↔Claude corpus from both sources — Claude Code
   JSONL session logs (across machines) and Claude Desktop account exports
   (across accounts).
2. **Normalizes** them into a single, durable, **app-independent** local store
   that Duncan owns and that survives the Claude apps changing or disappearing.
3. **Consolidates** across accounts and machines, with deduplication and
   provenance (account / machine / model / timestamp / project / session).
4. **Retrieves** by three complementary channels: lexical (tantivy, the matched
   filter), semantic (lancedb, the meaning channel), and graph (petgraph, the
   relational structure) — maximizing I(X;Y) between a query and the history.

## Boundaries (explicit non-goals)

- **No GUI.** CLI only.
- **No generation.** ixy retrieves; it does not call a model to answer. (A
  generative layer may come later; it is out of scope for this project.)
- **No cloud.** Fully local; the store is a file tree / embedded DBs Duncan owns.
- **No live capture.** Snapshot + idempotent incremental ingest, not real-time tailing.
- **ixy does not trigger Desktop exports.** Those are manual/async; ixy ingests
  them once present on disk.

### Deviation note (honest, per spec-keeping)

The framework's SDLC places a separate **design doc** (step 3) before the arc
roadmap. We are folding a lightweight architecture note into this plan instead,
because the operator judged the problem space well-known and well-bounded
(ingest → normalize → index → query is a standard shape) and elected to skip
heavy research/design. **Risk accepted:** if discovery (arc01) or analysis
(arc03) surfaces architectural surprises, we promote a real design doc at that
point rather than forcing them into this file.

**Resolved 2026-06-30:** discovery (arc01) + the storage-substrate research pass
made the architecture real (layering, storage model, concurrency, Dropbox/XDG
placement, embedding pinning). A design doc was promoted as pre-authorized —
`design-doc.md`. This note stands as the record of *why* the design doc arrived
after the roadmap rather than before it.

### Architecture note → promoted to a design doc

The architecture is now the design of record in **`design-doc.md`** (this file is
the plan, not the design). In one line: **files are canonical; every index is
derived and rebuildable** — a layered `ixy-core` + thin CLI/MCP adapters over a
three-layer store (raw bedrock → normalized corpus → local-per-machine derived
indices: tantivy / Lance / petgraph). See `design-doc.md` for the pieces, the
concurrency model, the path placement, and the alternatives rejected.

## Arc roadmap (dependency order)

| Arc | Capability (one line) | Depends on |
|-----|-----------------------|------------|
| **arc01 — discovery** | A reproduced map of every corpus source: formats, record schemas, volumes, time-span, provenance fields. | — |
| **arc02 — classification** | A normalized record model reconciling Code + Desktop, with the provenance dimensions and dedup keys; a classifier/normalizer. | arc01 |
| **arc03 — analysis** | A profile of the classified corpus (volume, cross-source dedup, coverage, chunking) that finalizes the storage design. | arc02 |
| **arc04 — consolidated storage** | The durable, app-independent, multi-account/multi-machine store + idempotent incremental ingest. | arc03 |
| **arc05 — indexing** | tantivy + lancedb + petgraph indexes over the store. | arc04 |
| **arc06 — retrieval CLI** | The `ixy` query surface: hybrid search, read, browse, provenance-filtered. | arc05 |

Arcs 04–06 are **pencil** — discovery and analysis are expected to reshape them.
Per *plan late, plan deep*, only the active arc is detailed (see
`arc01-discovery/arc-plan.md`).

## Current status

- **Active:** arc01 — discovery, **proposed-closed pending independent gate**. All
  three slices closed (slice01 CC-verified, slice02 CC-verified, slice03 doer-closed);
  `arc01-discovery/corpus-map.md` delivered; arc `closing-report.md` written.
- **Closed:** none yet (arc01 awaits its gate → then P-1 done, arc02 activates).
- **Not yet planned in detail:** arc02–arc06.

## Project ledger

_Definition of done above. Class-(a) rows: each arc closed + composed. Class-(b)
rows: the DoD demonstrated end-to-end, reproduced at project scale. Opens here;
closes (per-row walk) in the project `closing-report.md`._

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| P-1 | arc01 discovery closed + composed | ptr: arc01 closing-report | correctness | project-plan | open | proposed: `arc01-discovery/closing-report.md`; composition A-4/5/6 reproduced (`corpus-map.md`); independent gate pending | → done at arc01 gate |
| P-2 | arc02 classification closed + composed | ptr: arc02 closing-report | correctness | project-plan | open | | attested |
| P-3 | arc03 analysis closed + composed | ptr: arc03 closing-report | correctness | project-plan | open | | attested |
| P-4 | arc04 storage closed + composed | ptr: arc04 closing-report | correctness | project-plan | open | | attested |
| P-5 | arc05 indexing closed + composed | ptr: arc05 closing-report | correctness | project-plan | open | | attested |
| P-6 | arc06 retrieval closed + composed | ptr: arc06 closing-report | correctness | project-plan | open | | attested |
| P-7 | `ixy ingest` consumes BOTH a Code JSONL tree and a Desktop export into one store | acceptance demo | serious | DoD | open | | reproduce at project scale |
| P-8 | lexical query returns exact-phrase hits with full provenance | acceptance demo | serious | DoD | open | | reproduce at project scale |
| P-9 | semantic query returns meaning-matched hits when exact words differ | acceptance demo | serious | DoD | open | | reproduce at project scale |
| P-10 | multi-account + multi-machine corpora consolidate with dedup | acceptance demo | serious | DoD | open | | reproduce at project scale |
| P-11 | store is app-independent: corpus fully usable with the source apps absent | acceptance demo | serious | DoD | open | | reproduce at project scale |

## Version History

### v1.2 — 2026-06-30
arc01 discovery **proposed-closed** (pending independent gate): all three slices
closed, `corpus-map.md` delivered, arc `closing-report.md` written with the
composition check + project bubble-up. P-1 evidence populated (gate-pending). No
roadmap change — arcs 02–06 hold; the discovery findings were absorbed into
`design-doc.md` + `corpus-map.md`, not into new arcs.

### v1.1 — 2026-06-30
Promoted the architecture to a design doc (`design-doc.md`) per the pre-authorized
deviation, after arc01 discovery + the storage research pass hardened the design.
Replaced the provisional inline architecture note with a pointer; recorded the
deviation as resolved. Roadmap + ledger unchanged (arc01 still active). Surfaced
by: the storage-design conversation (Dropbox sync, odm reconciliation, concurrency,
XDG placement).

### v1.0 — 2026-06-30
Initial roadmap. Six arcs (discovery → classification → analysis → storage →
indexing → retrieval). DoD, boundaries, and the design-doc deviation note set.
Project ledger opened (P-1..P-11). Arc01 active.
