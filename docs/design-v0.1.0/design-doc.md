# ixy — design doc (v0.1.0): architecture of record

> The SDLC step-3 artifact: **what the pieces are, how they fit, and which
> alternatives were weighed and rejected.** Promoted from `project-plan.md`'s
> provisional inline architecture note once discovery + storage research made the
> architecture real (the deviation the project-plan pre-authorized). The *plan*
> lives in `project-plan.md`; the *design* lives here. Research backing lives in
> `workbench/2026.06.30-research-storage-substrate.md` and
> `workbench/2026.06.30-discovery-corpus-structure.md` (this doc cites them the
> way odm's arc04 cites ODD-0014).
>
> **Status:** provisional-but-hardened. Sections 2–6 are committed design;
> Section 10 (open questions) is deferred to arc02/arc03 by *plan late, plan deep*.

## 1. The thesis

`ixy` recovers **I(X;Y)** from the Duncan↔Claude corpus: the query is Y, the
corpus is X, and the tool maximizes the mutual information between them. One
architectural sentence drives everything below:

> **Files are canonical; every index is derived and rebuildable.**

The corpus (raw + normalized) is the durable, app-independent, Dropbox-synced
source of truth. The lexical, vector, and graph indices are disposable caches
that serve query speed and can be regenerated from the files at any time. This is
the same files-as-database philosophy as `odm`, specialized for a retrieval
workload.

## 2. Layered architecture (ports & adapters)

MCP goes **on top, never at the base** — the same instinct as keeping the
`fs_index` extraction out of the base. Three layers:

| Layer | Crate(s) | Depends on |
|-------|----------|------------|
| **Core domain** | `ixy-core` — store, normalize, ingest, index, query. Zero CLI, zero MCP, zero server deps. | — |
| **Delivery adapters** | `ixy` (thin `clap` CLI over core); `ixy-mcp` (thin MCP server over core) | `ixy-core` |

The core is the hexagon; CLI and MCP are two delivery mechanisms that both just
*call the library*. MCP's overhead therefore exists only when MCP is in use. A
future HTTP/daemon front-end would be a third adapter, not a change to the core.

**Multi-process constraint (load-bearing).** A short-lived CLI invocation and a
long-lived MCP server are *separate OS processes* over the same on-disk store, so
the core coordinates **through the filesystem, not in-process state**. It must
never assume "I am the only process." This is exactly what tantivy/Lance already
assume; the core inherits their model rather than inventing one. No mandatory
daemon (that would reintroduce the always-on overhead we're avoiding).

Aligns with rust-guidelines CLI-01/03 (testable `lib` + thin `main`).

## 3. Storage model — three layers

Canonical = files, but "files" is really **two** layers of truth-and-derivation,
plus the indices:

### 3.1 Raw bedrock (immutable, synced, never deleted)

The original Desktop export archives and Claude Code JSONL, kept **verbatim**.
This is the true canonical layer, because *app-independence only holds if we keep
the originals* — our own normalizer can be lossy or buggy, and if it is, a
normalized-as-canonical store has silently discarded fidelity we cannot recover
once the source app is gone. Bedrock is append-only: never edited, never deleted.

### 3.2 Normalized corpus (derived-but-persisted, synced)

One file **per conversation (Desktop) / per session (Code)** — chosen to keep
file counts Dropbox-friendly (hundreds–few thousand, *not* 100k+ per-message
files). Carries the unified record + provenance. Two properties matter:

- **Sparse-union provenance.** No source carries every dimension. Code has
  `model`/`cwd`/`gitBranch`/`version` but no account; Desktop has `account` +
  human-named conversation but no `model`/`cwd`/`version`. The normalized record
  is a sparse union over `account · machine · config-root · model · timestamp ·
  project(cwd) · session/conversation`.
- **Monotonic / superset-merge (deletion policy *inverts* odm's).** odm-index
  treats "file gone from disk" as "delete the record." A **memory** tool must not:
  if a conversation vanishes from a newer export, ixy **keeps** it — durable
  memory the app might lose is the whole point. A new export is a *superset-merge*
  by `(uuid, updated_at)`, never a mirror. The stat-cache's deletion-detection is
  neutered at the corpus layer.

### 3.3 Derived indices (local per-machine, never synced)

- **tantivy** — lexical / matched-filter (exact phrase, BM25).
- **Lance** — vector / semantic channel (embeddings + ANN).
- **petgraph** — relational structure (in-memory, rebuilt from the corpus;
  persist a snapshot only if load time demands it).

All three are derived, self-healing, and rebuildable from §3.1–3.2. They live in
the machine-local platform data dir (§4), **never** beside the synced corpus.

### 3.4 Ingest change-detection (odm-index, lifted)

ixy's ingest is odm-index's stat-cache pattern, adopted near-verbatim: per-file
`(mtime, size, content_hash, meta_hash, extracted-metadata)` records;
first-run-full / subsequent-delta; **racy-git-correct** (size + a content-hash
fallback for any entry whose `mtime >= cache_timestamp`; never stat-only); atomic
write (temp + rename + fsync); corrupt/missing ⇒ rebuild. Two ixy-specific
refinements:

- **Two-level hashing (file + message).** Canonical files are per-conversation;
  the re-embed unit is the message. File-level hashing gives cheap "did anything
  change"; message-level `content_hash` ensures we re-embed only genuinely new
  messages (an appended message to a live Code session must not re-embed the whole
  conversation). Early-cutoff on `content_hash` guards the dominant cost —
  embeddings.
- **`content_hash` is the authority, harder than usual.** Over Dropbox, mtime and
  inode are unreliable (sync perturbs them — a fact odm-index §2.3/§4 already
  flags for network FS). ixy leans on the content hash as truth and stat only as
  the fast filter. This is what makes the stat-cache survive sync.

## 4. Paths & placement

The asymmetry is the whole point: **data path is user-chosen; index path is
computed and not co-locatable.**

- **Canonical corpus** — a **user-specified** path (their Dropbox folder).
- **Derived indices** — computed under the **machine-local platform data dir**
  (`directories::ProjectDirs` data dir → macOS `~/Library/Application
  Support/oxur-ixy`, Linux `$XDG_DATA_HOME/oxur-ixy`), keyed by a hash of the
  corpus root (`…/oxur-ixy/indices/<corpus-hash>/`) so one machine can index more
  than one corpus and a Dropbox path that differs per machine still resolves. The
  co-locate-with-data option is **never exposed** — the footgun is unrepresentable,
  not merely discouraged.

**DATA_HOME, not CACHE_HOME — deliberately, against the usual rule.** Derived
self-healing caches are textbook `$XDG_CACHE_HOME`. But embeddings are *expensive*
to regenerate (unlike odm's cheap stat-cache), and **macOS purges
`~/Library/Caches` under disk pressure** — so cache-home indices could evaporate
and force a costly re-embed at the worst time. DATA_HOME is correct *because* the
derived layer is expensive. (If ever split: the cheap parts — stat-cache, FTS
segments — may live in cache-home; the embeddings must not.)

Crate: `directories::ProjectDirs` (per-platform native) is the default; `etcetera`
with its XDG strategy only if we want literal `~/.local/share` on macOS too.

## 5. Concurrency model

Concurrent access is the **normal** case (Duncan's CLI + Claude's CLI/MCP during a
live conversation), not an edge. It splits cleanly:

- **Concurrent reads (the 95%)** — all *reads* over immutable/derived data.
  tantivy (immutable segments, snapshot-per-reader) and Lance (versioned manifests)
  natively support many concurrent reader processes. No coordination.
- **Writes (the 5%) — single *local* writer.** Ingest is occasional and serializes
  behind one advisory `flock` in the index dir. Reads never block (atomic commit;
  old segments/versions kept alive until readers release). The engines refuse
  concurrent writers via their own locks.

**Concurrency ⊥ Dropbox (the clean separation).** All concurrent multi-process
traffic hits the *local* index dir on one machine, using real local-filesystem
locks that work. The only synced thing is the canonical files, which are
append-mostly and need no locking — so Dropbox never has to arbitrate a lock,
which is exactly what it is bad at. This is the **third independent argument** for
local-per-machine indices, alongside (a) Dropbox corrupts multi-file index dirs
mid-sync and (b) the embedding model must be pinned locally to stay regenerable.

## 6. Embedding model — a pinned local dependency

"Regenerate the vector index" is only true if the embedding model is **pinned,
available, and deterministic.** A remote API embedder undermines the entire
app-independence thesis (deprecation kills reproducibility; machine A on v1 and
machine B on v2 produce non-aligned vector spaces). Therefore: a **pinned local
embedder** (a small sentence-transformer / gguf we own), its identity + version
**stamped into the index metadata**. Embedding compute happens at ingest, locally,
and the ANN index is rebuildable from stored vectors.

## 7. Sources roster (coverage)

Two source *kinds*, many roots/accounts/machines (detail in the discovery doc):

- **Claude Code (JSONL)** — one schema, many config roots: `~/.claude` (default),
  `~/.claude-bilt`, `~/.claude-banyan` (work profiles, second machine). Adds a
  **config-root/profile** provenance dimension. slice01's cc-prompt is reusable
  against each root.
- **Claude Desktop (export JSON)** — per-account snapshots. Personal account
  characterized (slice02-recon). **Work account = known coverage gap** (admin, not
  owner → cannot export).

## 8. Alternatives considered and rejected

- **SQLite/DuckDB as the Dropbox-synced primary** — rejected. Monolithic mutable
  file + async sync = corruption (WAL sidecars, conflicted copies, no lock
  awareness). Fine only as a *local, rebuildable* query cache if SQL ergonomics are
  ever wanted; never the store of record.
- **Normalized-as-canonical (discard raw)** — rejected. Loses fidelity to
  normalizer bugs; breaks app-independence. Raw bedrock is kept (§3.1).
- **Mandatory daemon owning the store (CLI/MCP as IPC clients)** — rejected.
  Centralizes write-coordination but adds always-on overhead we're avoiding.
  Library-first + multi-process-via-fs instead (§2, §5). Revisit only if
  write-coordination ever gets genuinely painful.
- **No-DB / no-FTS (odm's choice)** — rejected *for ixy*. odm can decline FTS
  because its workload is metadata filtering; ixy's workload *is* prose + semantic
  retrieval, so tantivy + Lance are the product, not deps to avoid (§9).
- **Extract `oxur/fs_index` now** — deferred, not rejected (§9).
- **Sync the derived indices** — rejected for the hot path (§5); rebuild/maintain
  locally per machine.

## 9. Relationship to `odm`

- **Adopt wholesale:** odm-index's stat-cache / racy-correct incremental reconcile
  / atomic-snapshot / self-heal design *is* ixy's ingest change-detector (§3.4).
- **Diverge on one axis, with cause:** odm's deliberate *no-FTS, no-vector* is
  right for metadata-filtering; ixy's retrieval workload requires real lexical +
  vector engines, slotted exactly where odm puts its in-memory maps — as derived,
  rebuildable caches over the file truth. Different derived structures, same
  substrate philosophy.
- **`oxur/fs_index` extraction — deferred by design.** The racy-correct stat-cache
  is an excellent shared-crate candidate (correctness-critical, domain-agnostic),
  and the likely endpoint is a shared crate both odm and ixy depend on. But
  extracting *now* would design the API against one real consumer + one imagined
  one (the premature-abstraction trap), and would destabilize odm's closed, green
  arc04 for zero present benefit. Sequence: build ixy's index at the *same clean
  seam* odm already has (generic `record/build/warm/snapshot/hash` vs. domain
  `maps/adapter`), let ixy reveal the real shared surface (message-vs-file
  granularity, Dropbox stat semantics, embeddings), then extract shaped by **two
  real consumers**. Decide at ixy's arc04; tracked as a candidate, not dropped.

## 10. Open questions (deferred to arc02/arc03)

Parked here so they are disclosed, not dropped; resolved when those arcs open.

- **Chunking for embeddings (arc03).** Messages vary from one line to 50 KB Bash
  stdout to 2,774 thinking blocks. What to chunk, what to skip (raw tool stdout is
  mostly noise), what to keep (human + assistant + thinking). This is a
  retrieval-*quality* decision — it moves I(X;Y).
- **Dedup / identity + branching (arc02).** Superset-merge on `(uuid, updated_at)`;
  Desktop message editing yields `parent_message_uuid` *branches* (trees, not
  lines) — what is the canonical message set and reading order?
- **Secret handling (arc02/arc04).** tool_results carry live Bash stdout →
  potential credentials in the canonical corpus. Encryption at rest?
  Scrub-on-ingest? At minimum, redaction discipline (rust-guidelines API-18 /
  LO-15: `Debug` that never prints secrets).
- **Embedding model choice (arc03).** Which local model; dimensionality vs. quality
  vs. ingest cost.
- **Multi-corpus keying (arc04).** Confirm the `<corpus-hash>` scheme if more than
  one corpus is ever indexed on a machine.

## 11. Sources

- `workbench/2026.06.30-research-storage-substrate.md` — Dropbox sync mechanics,
  file-vs-DB analysis, odm reconciliation (with external primary sources).
- `workbench/2026.06.30-discovery-corpus-structure.md` — corpus roster + schemas.
- `odm` ODD-0014 (`docs/design/06-final/…`) + arc04 `arc-plan.md` — the stat-cache
  design ixy's ingest adopts.
- `docs/design-v0.1.0/arc01-discovery/**` — the discovery slices this doc rests on.

## 12. Version history

### v1.0 — 2026-06-30
Initial architecture of record. Promoted from project-plan's provisional inline
note per the pre-authorized deviation, after discovery (arc01) + the storage
research pass. Commits: the layered core/adapter split, the three-layer storage
model (raw bedrock → normalized → derived indices), local-indices-in-DATA_HOME
with the non-colocatable path asymmetry, the read-heavy/single-local-writer
concurrency model, the pinned-local-embedder requirement, the deletion-monotonicity
inversion vs. odm, and the `fs_index`-defer decision. Open questions parked for
arc02/arc03.
