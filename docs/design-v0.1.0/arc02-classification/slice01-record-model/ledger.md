# arc02 · Slice 01: record model

## Ledger

_CC (implementer) fills Status + Evidence (`attested`) as work lands; CDC verifies
→ `reproduced`. Cargo rows reproduce via local 1.85+ / CI. Verify commands run at
repo root._

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| N-1 | Cargo workspace + `crates/ixy-core` build clean (edition 2024, MSRV 1.85, workspace lints/deps) | `cargo build --workspace`; inspect root `Cargo.toml` | serious | slice-doc | done | attested: `cargo build --workspace` exit 0. Root `Cargo.toml` = virtual workspace (resolver 2, `[workspace.package]` edition 2024 / rust-version 1.85, `[workspace.dependencies]` thiserror 2, `[workspace.lints]`). | first Rust in repo |
| N-2 | `NormalizedMessage` + `Source`/`Role`/`Block`/`Provenance`/`Thread`/`RawRef` defined with corpus-map §3b fields | `cargo doc`/grep public items; type presence | serious | slice-doc | done | attested: all 7 re-exported from `lib.rs` (+ 9 id newtypes + `IdError`); `cargo doc --no-deps` exit 0. Fields per §3b. | the record model |
| N-3 | `Block`/`Role`/`Source` are `#[non_exhaustive]` | `grep -n 'non_exhaustive'` on the enums | correctness | slice-doc | done | attested: `#[non_exhaustive]` on `Source`+`Role` (taxonomy.rs) and `Block` (block.rs). Also on `IdError`. | TD-07 |
| N-4 | `Provenance` is sparse — every source-specific field `Option`; test builds Code-style + Desktop-style leaving the other's fields `None` | `cargo test provenance_sparse` | serious | slice-doc | done | attested: `provenance_sparse` passes — Code shape (model+cwd Some, account None) and Desktop shape (account Some, model+cwd None); anchor `session_or_conversation` mandatory (non-Option). | the sparse-union |
| N-5 | `Thread` represents both null-root (Code) and sentinel-root (Desktop) uniformly | `cargo test thread_roots` (both root shapes → `is_root`) | serious | slice-doc | done | attested: `thread_roots` passes. `Thread::root()` → `is_root && parent_id==None` for both sources; invariant `is_root == parent_id.is_none()` guaranteed by constructors. | root detection is source-agnostic |
| N-6 | `NormalizedMessage` construction requires `RawRef` (non-lossy invariant) | `cargo test raw_ref_required`; no constructor omits it | serious | slice-doc | done | attested: `raw_ref_required` passes. `new()` is the sole constructor; takes `raw_ref: RawRef` by value (non-Option) → compile-enforced. `source()` derived from it. | bedrock recoverable |
| N-7 | IDs are newtypes (no stringly-typed ids) | grep the id fields' types | correctness | slice-doc | done | attested: 8 validating string newtypes (ids.rs); message/thread id fields typed `MessageId`/`Option<MessageId>`; grep finds no bare-`String` id field. | TD-03 |
| N-8 | `Debug` on message/blocks elides/truncates content bodies (redaction) | `cargo test debug_redacts`; assert body text not in `{:?}` | serious | slice-doc | done | attested: two `debug_redacts` tests (block.rs + message.rs) pass — secret body absent from `{:?}`; variant/tool-name/lengths shown. Manual `Debug` on `Block` + `NormalizedMessage`. | API-18 / §10 secrets |
| N-9 | Public items documented (rustdoc; `# Examples` on the record model) | `cargo doc --no-deps`; `#![deny(missing_docs)]` or lint | polish | slice-doc | done | attested: `#![deny(missing_docs)]` + `#![deny(rustdoc::broken_intra_doc_links)]` in lib.rs; `cargo doc --no-deps` exit 0; `# Examples` doctests on `NormalizedMessage` + `Provenance` pass. | DC |
| N-10 | `cargo test` + `clippy -D warnings` + `fmt --check` clean; **no `unsafe`** | run all three; `grep -rn 'unsafe'` = none | serious | slice-doc | done | attested: test exit 0 (7 unit + 2 doctest); `clippy --all-targets -- -D warnings` exit 0 (with clippy::pedantic); `fmt --check` exit 0. 0 lines of `unsafe` code; `unsafe_code = "forbid"` (workspace lint) — the sole `grep 'unsafe'` hit is a doc comment. | quality floor |

## What Worked

- **Delegated the guideline extraction, kept the design.** Three parallel
  subagents distilled the relevant TD/AP/DC/PS/CG patterns from ~300 KB of
  guides while the load-bearing design decisions (invariants-via-types, the
  sparse-anchor, source-derived-from-raw_ref) stayed in the main context. Serial
  on thinking, parallel on lookup.
- **Invariants enforced by types, not runtime checks.** `raw_ref` mandatory
  (non-Option field + sole constructor), sparse provenance (`Option` per source
  field + mandatory anchor), root consistency (`Thread` constructors) — all
  unrepresentable-if-violated rather than validated-at-runtime. The only
  fallible path is newtype construction (empty-id rejection), which is the one
  place a runtime check earns its keep.
- **`source()` derived from `raw_ref`** instead of stored twice — single source
  of truth *and* it dropped `new()` to 7 args, side-stepping
  `clippy::too_many_arguments` without an `#[allow]`.
- **Clippy clean with `pedantic` on the first real run** — writing to the
  guidelines up front (backticked doc identifiers, `#[must_use]` accessors,
  `# Errors` on fallible fns) beat chasing lints afterward.
- **Caught a pipe-masked exit code.** A `... | tail` pipeline hid clippy's real
  status; re-running unpiped confirmed exit 0. Evidence captured via explicit
  `$?`, not pipeline tails.

## Closure

**Closed 2026-06-30.** Verified by: CDC (`cdc-verification.md`) — structure + invariants
reproduced from source; **cargo gates reproduced by operator run (1.85+): build ✓, test ✓
(7 unit + 2 doctest), clippy `--all-targets -D warnings` (pedantic) ✓.** `fmt --check`
taken on CC attestation — the operator paste's final command was typo'd (`--checkk`) and
its output not shown, so fmt was not independently reproduced (formatting-only; trivially
re-runnable).
Rows: 10. Done: 10. Deferred: 0. No-op: 0.
Evidence strength: **N-1…N-10 reproduced** (structure/invariants by CDC; build/test/clippy
by operator run) — the sole exception is `fmt --check`, `attested`. **slice01 CLOSED; B-1 → done.**
