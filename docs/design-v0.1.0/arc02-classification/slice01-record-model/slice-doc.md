# arc02 · slice01 — record model (slice doc)

## Goal

Define the **normalized record types** in `ixy-core` that both source normalizers
(slice02 Code, slice03 Desktop) target and everything downstream (arc03–05) builds
on. Types + invariants + tests only — **no I/O, no normalizer logic, no
persistence.** This is the first Rust in the repo, so it also stands up the Cargo
workspace.

Designed against `../../arc01-discovery/corpus-map.md` §3b (the `NormalizedMessage`
sketch) and `../../design-doc.md`. **rust-guidelines** governs.

## Crate layout (the one thing to confirm before CC starts)

Reconciling `design-doc.md` §2 (crates `ixy-core` / `ixy` / `ixy-mcp`) with
`CLAUDE.md` (umbrella at `crates/oxur_ixy`), following the odm precedent
(umbrella + member crates, added as arcs need them):

- **Now (slice01):** Cargo **workspace** at repo root + member crate
  **`crates/ixy-core`** (the domain library; where these types live).
- **Deferred:** umbrella `crates/oxur_ixy` (package `oxur-ixy`, binary `ixy`) →
  the CLI arc (arc06); `crates/ixy-mcp` → its adapter. No empty stubs before their arc.

## Scope

**In:**
- Workspace `Cargo.toml` (resolver v2, edition 2024, MSRV 1.85, `[workspace.lints]`,
  `[workspace.dependencies]`) + `crates/ixy-core`.
- Types (per corpus-map §3b), all public, documented:
  - `NormalizedMessage { id, source, provenance, role, blocks, thread, timestamp, raw_ref }`
  - `Source` (Code | Desktop), `Role` (Human | Assistant | System | Tool)
  - `Block` (Text | Thinking | ToolUse | ToolResult | Meta) — carries content
  - `Provenance` — **sparse**: every source-specific field `Option` (account,
    machine, config_root, model, project_cwd, session_or_conversation, …)
  - `Thread { parent_id: Option<…>, is_root, branch? }` — represents **both**
    null-root (Code) and sentinel-root (Desktop) uniformly
  - `RawRef` — pointer back to bedrock (source + locator), so normalization is
    provably non-lossy
  - Newtype IDs (no stringly-typed ids)

**Out:** the normalizers (slice02/03); dedup/merge (slice04); Projects/Memories
(slice05); persistence (arc04); Desktop canonical-path selection (slice03).

## Design invariants (rust-guidelines)

- **Enums `#[non_exhaustive]`** where the corpus may add variants (`Block`, `Role`,
  `Source`) — TD-07.
- **Newtypes for ids** — TD-03; no bare `String` ids.
- **`Debug` redacts content bodies** — message text / block bodies / tool output may
  contain secrets (design-doc §10). Public types derive/implement `Debug`
  (M-PUBLIC-DEBUG) but elide/truncate bodies rather than dumping them (API-18).
- **`RawRef` is mandatory** on `NormalizedMessage` — construction cannot omit it
  (non-lossy invariant).
- Errors (if any constructors are fallible) via `thiserror` (EH); no `unwrap` on
  non-invariant paths; no `unsafe`.

## Verification approach

CC implements in `ixy-core`; CDC verifies every ledger row. Cargo rows
(`fmt` / `clippy -D warnings` / `test`) run locally (1.85+) and/or CI. The
non-lossy + sparse-provenance + dual-root invariants are pinned by unit tests, not
just types.

## Exit criteria

All ledger rows final; `cargo build`/`test`/`clippy -D warnings`/`fmt --check`
clean; the types express a `NormalizedMessage` constructible from *both* a
Code-shaped and a Desktop-shaped set of inputs (proven by tests), each with the
other source's provenance fields `None`.
