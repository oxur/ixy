# CC assignment — arc02 · slice01: record model

You are **CC** (Claude Code), the implementer (IC seat), for the first Rust in
`ixy`. Load the **collaboration-framework** and the **rust-guidelines** skill
(read `11-anti-patterns.md` first, then `01-core-idioms.md`, `05-type-design.md`,
`12-project-structure.md`, `15-cargo/01-cargo-basics.md`). This runs under
**ledger discipline**; CDC verifies every row.

## The job

Stand up the Cargo workspace and the `ixy-core` crate, and implement the
**normalized record types** — the schema both source normalizers (slice02/03)
will target. **Types + invariants + tests only. No I/O, no normalizer logic, no
persistence** (those are later slices).

The spec is `ledger.md` (rows N-1…N-10) — read it first. Scope + the crate layout
are in `slice-doc.md`. The field shapes come from
`../../arc01-discovery/corpus-map.md` **§3b** (the `NormalizedMessage` sketch) and
**§3a** (provenance dimensions) — do not re-derive them; the arc01 gate already
verified they're complete.

## Crate layout (confirmed)

- Cargo **workspace** at repo root: resolver v2, `edition = "2024"`, `rust-version
  = "1.85"`, `[workspace.lints]` (clippy `-D warnings` posture), `[workspace.dependencies]`.
- Member crate **`crates/ixy-core`** — the domain library; the types live here.
- **Do not** create `oxur_ixy` (umbrella/binary) or `ixy-mcp` yet — deferred to
  their arcs.

## Types to implement (from corpus-map §3b)

```
NormalizedMessage { id, source, provenance, role, blocks, timestamp, thread, raw_ref }
Source   = Code | Desktop                         // #[non_exhaustive]
Role     = Human | Assistant | System | Tool       // #[non_exhaustive]
Block    = Text{..} | Thinking{..} | ToolUse{..} | ToolResult{..} | Meta{..}  // #[non_exhaustive]
Provenance { account:Option, machine:Option, config_root:Option, model:Option,
             project_cwd:Option, session_or_conversation: <id>, timestamp? }   // sparse union
Thread   { parent_id: Option<Id>, is_root: bool, branch: Option<..> }  // null-root & sentinel-root both → is_root
RawRef   { source, locator }                       // pointer to bedrock; mandatory
```

## rust-guidelines to honor (non-negotiable rows)

- **TD-03** newtype ids (no bare `String` ids) — row N-7.
- **TD-07** `#[non_exhaustive]` on `Block`/`Role`/`Source` — row N-3.
- **M-PUBLIC-DEBUG + API-18** — public types implement `Debug`, but message/block
  `Debug` **elides or truncates content bodies** (they may hold secrets; §10). Prove
  with a test that body text does not appear in `{:?}` — row N-8.
- **`RawRef` mandatory** — no `NormalizedMessage` constructor may omit it — row N-6.
- **No `unsafe`**; `thiserror` for any fallible constructor; `?` not `unwrap` off
  invariant paths — row N-10.
- Rustdoc on public items, `# Examples` on the record model — row N-9.

## Reporting (ledger discipline)

1. Fill Evidence at `attested` as each row lands (commit SHA + Verify output).
2. **Per-row walk** in `closing-report.md` — status + evidence for every N-row; no
   prose summaries, no "deviations: none."
3. **Bubble-up to arc02** (Part IV): did slice01 deliver the record model; what did
   implementing the types reveal for slice02/03 (a field the sketch missed? a role
   or block variant the corpus needs that §3b didn't name? a redaction boundary?);
   the silent-drop diff.
4. Name any uncertainty. No silent drops — close with all 10 rows dispositioned.

## Out of scope

The normalizers (slice02 Code, slice03 Desktop), dedup/merge (slice04),
Projects/Memories (slice05), persistence (arc04). Build the types they'll use, not
the logic that uses them.
