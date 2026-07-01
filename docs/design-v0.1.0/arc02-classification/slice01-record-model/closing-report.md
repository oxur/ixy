# arc02 · slice01 — record model — CC closing report

**Seat:** CC (implementer). **Date:** 2026-06-30.
**Crate:** `crates/ixy-core` (first Rust in the repo).
**Ledger:** 10 rows (N-1…N-10). **Done: 10 · Deferred: 0 · No-op: 0.**
All evidence `attested` by CC; awaiting CDC/CI `reproduced`.

Gate summary (real exit codes, captured unpiped): `cargo build --workspace` 0 ·
`cargo test --workspace` 0 (7 unit + 2 doctest) · `cargo clippy --all-targets --
-D warnings` 0 (with `clippy::pedantic`) · `cargo fmt --check` 0 ·
`cargo doc --no-deps` 0 · `unsafe` code lines 0.

---

## Per-row walk

### N-1 — workspace + `ixy-core` build clean · **done**
Virtual workspace at repo root: `resolver = "2"`, `[workspace.package]`
(edition 2024, rust-version 1.85, license `Apache-2.0 OR MIT`),
`[workspace.dependencies] thiserror = "2"`, `[workspace.lints]` (rust
`unsafe_code = "forbid"`; clippy `all` + `pedantic` at warn). Member
`crates/ixy-core` inherits via `.workspace = true` and `[lints] workspace = true`.
`cargo build --workspace` → exit 0.

### N-2 — the record model defined per corpus-map §3b · **done**
`NormalizedMessage` + `Source`, `Role`, `Block`, `Provenance`, `Thread`, `RawRef`
all defined and re-exported flat from `lib.rs` (plus the id newtypes + `IdError`).
Fields follow §3b. `cargo doc --no-deps` → exit 0.
**One deliberate deviation from the sketch** (see bubble-up): `NormalizedMessage`
does *not* store a separate `source` field — it's derived from `raw_ref.source()`
so the two can't disagree.

### N-3 — `Block`/`Role`/`Source` `#[non_exhaustive]` · **done**
`#[non_exhaustive]` on `Source` and `Role` (taxonomy.rs) and `Block` (block.rs);
also applied to `IdError`. Verified by grep.

### N-4 — `Provenance` is sparse · **done**
Every source-specific dimension is `Option`; the always-present
`session_or_conversation` is the mandatory non-Option anchor. `provenance_sparse`
test builds a Code shape (model+cwd `Some`, account `None`) and a Desktop shape
(account `Some`, model+cwd `None`) — passes.

### N-5 — `Thread` unifies both root shapes · **done**
`Thread::root()` yields `is_root == true && parent_id == None` for both null-root
(Code) and sentinel-root (Desktop); the normalizer maps either onto it. The
invariant `is_root == parent_id.is_none()` is guaranteed by the constructors
(fields are private; there is no inconsistent way to build one). `thread_roots`
passes.

### N-6 — construction requires `RawRef` · **done**
`NormalizedMessage::new` is the **sole** constructor and takes `raw_ref: RawRef`
by value (non-Option) → a message cannot exist without its bedrock pointer,
enforced at compile time. `raw_ref_required` passes.
**Caveat (honest):** the *proof* here is a positive runtime test plus the
type/constructor shape, not a `compile_fail` test. A `compile_fail` doctest was
considered and rejected — it passes on *any* compile error (e.g. an undefined
local), so it would be a vacuous proof of *this* invariant. The compile-time
guarantee is real (single constructor, mandatory param); the test witnesses it.

### N-7 — newtype ids · **done**
8 validating string newtypes (`MessageId`, `SessionId`, `AccountId`, `ModelId`,
`MachineId`, `Timestamp`, `BranchId`, `Locator`), each rejecting empty/blank
input. No bare-`String` id fields (grep clean). `Timestamp` is newtyped-string by
design — see bubble-up (structured-time parsing is deferred).

### N-8 — redacting `Debug` · **done**
Manual `Debug` on `Block` (prints variant + tool-name/meta-kind + body *lengths*,
never bodies) and on `NormalizedMessage` (safe fields + block count + redacted
blocks). Two `debug_redacts` tests (block.rs, message.rs) assert a planted secret
is absent from `{:?}` while structure remains — both pass.

### N-9 — documentation · **done**
`#![deny(missing_docs)]` + `#![deny(rustdoc::broken_intra_doc_links)]` +
`#![warn(rustdoc::missing_crate_level_docs)]` in lib.rs; crate-level `//!`
overview; `# Examples` doctests on `NormalizedMessage` and `Provenance` (both run
green, use `?` per DC-09). `cargo doc --no-deps` → exit 0.

### N-10 — quality floor · **done**
`cargo test` exit 0 (7 unit + 2 doctest); `clippy --all-targets -- -D warnings`
exit 0 **with `clippy::pedantic`**; `cargo fmt --check` exit 0. Zero lines of
`unsafe` code; `unsafe_code = "forbid"` at the workspace level (stronger than a
grep). **Caveat:** the literal `grep -rn 'unsafe'` returns **one** hit — a doc
comment in `lib.rs` stating the crate forbids unsafe. Actual unsafe code: 0
lines; the `forbid` lint is the compile-time guarantee. Two `#[allow]`s exist,
both documented: crate-level `clippy::module_name_repetitions` (natural
module/type names; API is flat via re-exports) — no others.

---

## Silent-drop diff (scope-as-specified vs delivered)

| slice-doc "In" item | Delivered? |
|---|---|
| Workspace `Cargo.toml` (resolver 2, ed 2024, MSRV 1.85, lints, deps) + `ixy-core` | ✅ N-1 |
| `NormalizedMessage` + Source/Role/Block/Provenance/Thread/RawRef | ✅ N-2 |
| Sparse `Provenance` | ✅ N-4 |
| Uniform `Thread` (null + sentinel roots) | ✅ N-5 |
| Mandatory `RawRef` | ✅ N-6 |
| Newtype ids | ✅ N-7 |
| `#[non_exhaustive]` enums | ✅ N-3 |
| Redacting `Debug` | ✅ N-8 |
| Docs + examples | ✅ N-9 |
| Quality floor (test/clippy/fmt/no-unsafe) | ✅ N-10 |
| (Out) normalizers, dedup/merge, Projects/Memories, persistence | correctly excluded |

**No silent drops.** 10 opening rows, 10 dispositioned. Nothing in "In" skipped;
nothing from "Out" pulled in.

---

## Bubble-up to arc02 (did it deliver; what did implementing reveal; drop diff)

**1. Did slice01 deliver the record model?**
Yes. `ixy-core` exports the full normalized model the two normalizers (slice02
Code, slice03 Desktop) will target, with the non-lossy, sparse-provenance,
dual-root, and redaction invariants pinned by types + tests. arc02's next slices
can `use ixy_core::…` and construct these without touching this crate.

**2. What did implementing the types reveal that §3b didn't fully name?**
Concrete findings for the normalizer slices and the arc02 schema:

- **`source` is redundant with `raw_ref.source`.** The §3b sketch lists both a
  top-level `source` and `raw_ref{source,…}`. I stored it once (in `RawRef`) and
  derived `NormalizedMessage::source()`. If arc02 ever needs a source that
  differs from the bedrock's, this must change — but I can't think of a case, and
  single-sourcing prevents drift. **Flagging for confirmation.**
- **Timestamp is unresolved as a *type*.** §3b shows a bare `timestamp`, but
  Desktop messages carry **both `created_at` and `updated_at`**, Code has record
  ISO-8601 *and* `history.jsonl` epoch-ms, and Desktop `updated_at` is what edit
  branching keys off. I modeled `Timestamp` as a validated **string newtype**
  (no parsing, no `chrono` dep — both out of scope this slice) and put **one**
  timestamp on the message. **arc02 must decide:** (a) structured instant +
  `chrono`/`jiff` dependency? (b) does the model need created-vs-updated as two
  fields (dedup/merge on `updated_at` per corpus-map §4 suggests yes)?
- **`#[non_exhaustive]` vs sibling-crate construction.** For the enums, sibling
  normalizer crates can still *construct* existing variants (`Block::Text{…}`,
  `Source::Code`) — non_exhaustive only forces a wildcard in their `match`es.
  Confirmed by the Rust reference; worth a live check in slice02 when the first
  external constructor lands. `Provenance`/`Thread`/`RawRef` use constructors, so
  no issue there.
- **Tool `input`/`output` and `Meta.data` are raw `String`.** No `serde_json`
  dependency was added (types-only slice). If arc02 wants structured tool-call
  payloads (e.g. to index tool names/args), a `serde_json::Value` or typed
  representation is a deliberate dependency decision then, not now.
- **Redaction boundary drawn at *bodies only*.** `Debug` redacts message/thinking/
  tool bodies but **shows** provenance paths (`project_cwd`, `config_root` —
  which contain the home-dir username) and ids. Per design-doc §10 the secret
  concern is tool stdout / message content, not paths; but if paths count as PII
  for the at-rest posture (arc04), that boundary may need to move.

**3. Silent-drop diff:** none (table above). 10/10 rows; "In" fully delivered,
"Out" held.

---

## Handoff

Nothing committed yet — left for the operator's gate (per the pattern this
session has followed). To reproduce: from repo root on a 1.85+ toolchain, run
`cargo build --workspace && cargo test --workspace && cargo clippy --all-targets
-- -D warnings && cargo fmt --check`; all exit 0. CDC then takes each row
`attested → reproduced` and writes `cdc-verification.md`.
