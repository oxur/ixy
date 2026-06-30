# slice01 — code-jsonl (slice doc)

## Goal

Produce a **documented, reproduced characterization** of the Claude Code JSONL
corpus on this machine (`~/.claude/projects/**/*.jsonl` plus `history.jsonl`),
sufficient to feed the normalized record model in arc02. This is a CC content
pass in the operator's trust domain; CDC verifies.

## Scope

**In:**

- Record-level **schema**: the union of top-level keys across a sample, and where
  it varies across CC versions.
- **Record-type distribution** (`type`: user / assistant / summary / tool_use /
  tool_result / …).
- **Provenance fields** present per record: `cwd`, `sessionId`, `timestamp`,
  model string, `parentUuid` (threading), `version`.
- **Content-block structure**: how `message.content` represents text vs tool_use
  vs tool_result.
- **Volumes + time-span**: dir/file counts, total bytes, oldest/newest session.
- **Path-encoding gotcha**: confirm dir-name lossiness against a record's `cwd`.
- **`history.jsonl`** shape.

**Out:**

- The Desktop export (slice02).
- The normalized schema design itself (arc02).
- Any ingestion / Rust code (later arcs).
- Deep characterization of other `~/.claude` stores (`tasks/`, `plans/`,
  `file-history/`, `sessions/`) — note their existence/relevance only.

**Stretch (deferrable):** peek the claude.ai IndexedDB LevelDB for cached
Desktop transcripts.

## Verification approach

CC runs the recon commands (see `cc-prompt.md`), capturing **reproducible
command output** into `workbench/2026.06.30-discovery-code-jsonl.md`. CC fills
the ledger Evidence at strength `attested` and walks the ledger row-by-row in
`closing-report.md`. CDC (this Cowork session) reads the findings file from the
mounted `workbench/`, checks each Verify is a command that would fail if the
criterion were false, reproduces what it can from the captured output + the
inventories, and writes `cdc-verification.md`.

## Exit criteria

All `ledger.md` rows reach a final status; the Code-source inputs to the corpus
map (slice03) are complete and reproduced; no silent drops (close row-count =
open row-count).
