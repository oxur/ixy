# arc02 · slice01 — record model — CDC verification

**Seat:** CDC (independent verifier). **Date:** 2026-06-30. **Verifies:** CC's
closing ledger (N-1…N-10, all `attested`) against the actual `ixy-core` source.
**Verdict:** **structural + invariant rows REPRODUCED** by reading the code; the
**cargo exit-code rows are attested, pending CI/operator reproduction** (no Rust
toolchain in the Cowork sandbox — the honest limit, same shape as odm's "cargo
rows via CI"). No fabrication, no spec-softening; two CC caveats verified honest.

## Method and its limit

The Cowork sandbox has **no `cargo`/`rustc`**, so I cannot re-run
build/test/clippy/fmt here. My reproduction is therefore two-tier:

- **reproduced (by reading the actual source):** every row whose truth is visible
  in the code — types, `#[non_exhaustive]`, newtypes, the sparse/dual-root/non-lossy
  invariants (which are *type-enforced*, so reading the constructor shapes proves
  them), and the `Debug` redaction (readable in the impls + the tests' assertions).
- **attested, pending CI:** the four cargo *exit codes* (N-1 build, N-10
  test/clippy/fmt) — CC ran them (exit 0); reproduction needs a 1.85+ toolchain
  (CI or operator). The `unsafe_code = "forbid"` *guarantee* in N-10 I reproduce
  structurally (it's in the workspace manifest + zero `unsafe` blocks in any module).

## Per-row verdict

| Row | CDC | Note |
|-----|-----|------|
| N-1 workspace builds | **reproduced (struct)** / build exit-0 attested | virtual workspace, resolver 2, edition 2024, rust-version 1.85, `[workspace.lints]` forbid-unsafe + clippy all/pedantic, thiserror 2 — all present in root `Cargo.toml`. |
| N-2 types defined | **reproduced** | all 7 + 8 id newtypes + `IdError` re-exported in `lib.rs`; fields match corpus-map §3b. |
| N-3 non_exhaustive | **reproduced** | `#[non_exhaustive]` on `Source`, `Role` (taxonomy.rs), `Block` (block.rs), `IdError` (ids.rs). |
| N-4 sparse provenance | **reproduced** | `session_or_conversation: SessionId` mandatory (non-Option anchor); all 5 source-specific fields `Option`; builder `with_*`; `provenance_sparse` test logic sound (Code vs Desktop shapes). |
| N-5 dual-root thread | **reproduced** | `Thread::root/child/child_on_branch`; invariant `is_root == parent_id.is_none()` enforced by constructors (private fields); source-agnostic. |
| N-6 RawRef mandatory | **reproduced (strong)** | `new()` is the *sole* constructor, takes `raw_ref: RawRef` **by value** (non-Option), fields private → non-lossy is **compile-enforced**, not runtime. `source()` derived from `raw_ref` (single-sourced). |
| N-7 newtype ids | **reproduced** | 8 validating string newtypes via a private `string_newtype!` macro; empty/blank rejected → `IdError::Empty`; no bare-`String` id field. |
| N-8 Debug redaction | **reproduced** | `Block::Debug` prints variant + non-secret discriminants (tool name, meta kind) + body **lengths**, never bodies; `NormalizedMessage::Debug` delegates to it + shows `block_count`; `debug_redacts` tests assert `SECRET-*` absent. Verified no field bypasses redaction. |
| N-9 docs | **reproduced** | `#![deny(missing_docs)]` + broken-intra-doc-links; `# Examples` doctests on `NormalizedMessage` + `Provenance`; doc exit-0 attested. |
| N-10 clippy/fmt/test/unsafe | **reproduced (unsafe)** / cargo exit-0 attested | `unsafe_code = "forbid"` + zero `unsafe` blocks (read every module) = the real guarantee, reproduced. clippy-pedantic/fmt/test exit-0 = CC-attested, pending CI. |

## CC's two caveats — both honest, guarantees real

1. **N-6 is a positive test + the constructor shape, not a `compile_fail`.** Correct
   and well-called: a `compile_fail` doctest passes on *any* compile error (vacuous
   proof). The real guarantee here is stronger than a test — the sole-constructor +
   non-Option-by-value + private-fields shape makes "message without `RawRef`"
   *unrepresentable*. Verified by reading `message.rs`.
2. **N-10's literal `grep 'unsafe'` hits a doc/lint line, not code.** Correct —
   0 `unsafe` blocks; `unsafe_code = "forbid"` is the enforcement. Not softpedalled.

## What worked (Safety-II)

- **Invariants by construction, not runtime checks** — non-lossy, sparse-anchor,
  root-consistency all unrepresentable-if-violated. The *one* runtime check
  (empty-id rejection) is the one place it earns its keep, and it's the sole reason
  `thiserror` is pulled in — no gratuitous dependency.
- **`source()` derived from `raw_ref`** — single source of truth *and* it kept
  `new()` at 7 args, dodging `clippy::too_many_arguments` without an `#[allow]`.
- **CC delegated guideline *lookup* to subagents, kept the *design* in-context** —
  exactly the serial-on-thinking / parallel-on-lookup line.
- **Caught a pipe-masked exit code** (a `| tail` hid clippy's status) and re-ran
  unpiped — the kind of verification honesty the ledger exists to reward.

## Bubble-up verification + the load-bearing finding

CC's bubble-up is honest. The load-bearing item — **`Timestamp` is unresolved as a
type** — is real and I'm dispositioning it here:

- `NormalizedMessage` carries **one** opaque `timestamp` (string newtype, no
  parsing — correctly out of scope for slice01, and CC documented the deferral
  inline in `ids.rs`).
- But **Desktop has `created_at` *and* `updated_at`**, and **slice04 dedup/merge
  keys on `updated_at`** (corpus-map §4). One opaque timestamp is insufficient by
  the time slice04 lands.

**Disposition: deferred to slice03/slice04 (tracked, not dropped).** No arc02
slice-breakdown change — the finding is a record-model *refinement*, not a
re-slice. **Re-entry condition:** before **slice04**, the record must (a) carry
`updated_at` (the merge key), and (b) support a "newer-than" comparison
(lexicographic ISO-8601 UTC is acceptable if both normalizers emit it; otherwise a
structured instant + a time dependency). **slice03** (Desktop normalizer) and
**slice04** (dedup/merge) slice-docs must resolve this. Recorded in the arc02
arc-plan version history.

## Gate

**slice01 proposed-closed.** Structural + invariant rows reproduced by CDC; the
cargo exit-code rows (N-1 build, N-10 test/clippy/fmt) need a **1.85+ toolchain run
(CI or operator)** to flip `attested → reproduced` and finalize. On that run: arc
ledger **B-1 → done**. No defects found; the deferred `Timestamp` refinement is
tracked for slice03/04.
