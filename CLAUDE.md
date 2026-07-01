# ixy — project instructions for Claude sessions

> `ixy` — recovering **I(X;Y)** from the Duncan↔Claude corpus: a local Rust CLI
> that indexes and searches our entire conversation history across accounts and
> machines. The query is Y, the corpus is X; the tool maximizes the mutual
> information between them.

## Collaboration framework

This project runs under the **collaboration-framework** (peer frame, 9-point
SDLC, ledger discipline, bubble-up/close). Load it at the start of every
substantial session. Before planning or closing anything, read
`docs/PROJECT-MANAGEMENT.md` (from the framework) and `LEDGER-DISCIPLINE.md`.

Seats:

- **CDC** — Cowork/Desktop: planning, architecture, review, QA, prompt-authoring.
- **CC** — Claude Code in the repo: implementation, self-review, tests, evidence.
- **Operator** — Duncan: holds the sources of truth; grants access; gates closes.

## Planning layout (confirmed 2026-06-30, per `docs/PROJECT-MANAGEMENT.md`)

Planning artifacts live under `docs/design-vX.Y.Z/`:

- `project-plan.md` — roadmap + project ledger (DoD).
- `design-doc.md` — architecture of record (the pieces + how they fit; SDLC step 3).
- `arcNN-<slug>/arc-plan.md` — slice breakdown + arc ledger; `closing-report.md` at close.
- `arcNN-<slug>/sliceNN-<slug>/` — open set `{slice-doc, ledger, cc-prompt}.md`;
  close set `{closing-report, cdc-verification}.md`.

Discovery / scratch tooling lives in `workbench/` (throwaway; harden into
`crates/` only what proves worth keeping).

## Code layout

Cargo workspace at repo root; umbrella crate at `crates/oxur_ixy/`.

## Trust-domain split (important)

The corpus lives in protected directories — `~/.claude` and
`~/Library/Application Support/Claude` — that the Cowork session (CDC) **cannot
mount**. Discovery and ingest against those paths are run by **CC** in the
operator's trust domain; CC writes results into `workbench/`, which CDC reads.
That handoff channel is by design, not a workaround.

## Corpus sources

See `workbench/2026.06.30-discovery-corpus-structure.md`. Two sources:

1. **Claude Code** — `~/.claude/projects/<enc-path>/*.jsonl` (local, per-machine).
2. **Claude Desktop** — account **export** JSON (app-independent, per-account, async).

Schema rule learned in discovery: project dir names are a *lossy* path encoding
(real dashes collide with the `/`→`-` separator) — read the authoritative `cwd`
from inside records, never decode the dir name.
