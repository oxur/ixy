# slice01 — code-jsonl — CDC verification

**Seat:** CDC (independent review, Cowork trust domain)
**Date:** 2026-06-30
**Verifies:** CC closing-report + ledger (10 rows, all `done`/`attested`).
**Verdict:** **slice01 gated CLOSED.** No silent drops, no spec-softening. Three
rows independently **reproduced**; the rest **attested**, consistency-checked,
with the reproduction limit named below.

## Method and its honest limit

The corpus lives under `~/.claude`, which this Cowork session **cannot mount**.
So I cannot re-run the content Verify commands against the live store — the
structural limit of the trust-domain split. My verification therefore has two
tiers:

- **reproduced** — re-derived from an *independent artifact* (the operator's
  16:39 `find` inventory in `workbench/`), which CC did not use. This is genuine
  independent reproduction, not re-reading CC's output.
- **attested (consistency-checked)** — I confirmed the evidence exists in the
  findings artifact, is internally consistent, and that each Verify is a command
  that *would* fail if the criterion were false. Full reproduction of these needs
  a fresh CC context or the operator. This is the known structural limitation
  (LEDGER-DISCIPLINE §"Known structural limitation"), recorded, not hidden.

## Per-row verdict

| Row | CC | CDC strength | Note |
|-----|----|--------------|------|
| F-1 volumes | done | **reproduced** | Inventory = 44 dirs / 114 jsonl; ∈ CC drift band 43–45 / 104–115. Corroborates the live-corpus finding from an independent snapshot. |
| F-2 time-span | done | attested (consistent) | mtime-based; caveat (mtime ≠ first-message) is correct and disclosed. |
| F-3 union schema | done | attested (consistent) | 60-key union, `type` universal, 10+ versions; can't reach corpus to re-run. |
| F-4 type distribution | done | attested (consistent) | 13 types + two-tier model; counts internally consistent (sum ≈ 47.9k). |
| F-5 provenance | done | attested (consistent) | Non-uniformity caveat is a *strengthening*, not softpedalling — flags the real arc02 constraint. |
| F-6 content blocks | done | attested (consistent) | string vs typed-block array; block counts consistent. |
| F-7 path-encoding | done | **reproduced** | Cited dir `…lykn-lang--worktrees-cdc-build-dir-reorg` present in inventory; 3 `--` collision dirs total. Double loss (`/`+`.` → `-`) confirmed plausible. |
| F-8 history.jsonl | done | **reproduced (existence)** | Present in inventory. Shape/keys (epoch-ms `timestamp`, un-mangled `project`) attested — can't reach file contents. |
| F-9 IndexedDB (stretch) | done | attested (consistent) | Answered as a load-bearing **NO** (transcripts not cached) — a real disposition, not a fake no-op. Confirms slice02 necessity. |
| F-10 findings artifact | done | **reproduced** | File exists in mounted `workbench/`; contains command blocks + redacted examples for all 13 types. Read directly. |

## Discipline checks

- **Row count:** 10 opened / 10 dispositioned. ✓ No silent drops.
- **Spec-softening:** none. The two "done with caveat" rows (F-1, F-5) caveat in
  the direction of *more* honesty (no fixed total exists; fields aren't uniform),
  not weaker guarantees. ✓
- **No-op integrity:** F-9 is a `done` with a real answer, not a disguised skip. ✓
- **Bubble-up honesty:** the closing-report's three bubble-up answers are
  accurate and material. ✓

## Bubble-up disposition (the CDC decision Part IV forces)

**Does slice01 require an `arc-plan.md` change?** No change to **arc01's** slice
breakdown or sequencing — discovery's three slices stand, and F-9 *confirms*
(does not alter) slice02's necessity. So arc01 arc-plan is unchanged, which is a
valid recorded outcome.

But three findings are load-bearing **downstream** and are hereby routed forward
(dispositioned, not dropped), to be carried in slice03's corpus map and flagged
for the named arcs:

1. **Live, self-mutating corpus** → **arc04** (ingest must be snapshot-tolerant;
   files appear/grow/vanish mid-scan, incl. the running session's own file).
2. **Non-uniform provenance + 13-type two-tier model** → **arc02** (normalizer
   keys off per-type field presence; `cwd` from records, `model` at
   `message.model`, `file-history-snapshot` has no `sessionId`).
3. **Version-keyed schema drift** → **arc02** (open/union record type, never a
   closed struct).

These will surface again at arc02 planning; recording them here so they are not
re-derived. No `project-plan.md` change forced yet (plan late, plan deep).

## Gate

slice01 **CLOSED**, 2026-06-30. Arc ledger **A-1 → done**. Outstanding for
arc01: slice02 (Desktop export, blocked on download), slice03 (corpus map).
