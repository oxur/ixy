# arc01 — discovery — closing report (composition + bubble-up to project)

**Assembled by:** CDC (ran/verified the slices). **Status:** **PROPOSED-CLOSED,
pending independent gate** — per LEDGER-DISCIPLINE §B / PROJECT-MANAGEMENT Part V,
the one who performed the composition cannot sign it off. Gate = operator (Duncan)
or a fresh context / subagent. **Date:** 2026-06-30.

## Capability, restated + verdict

> A complete, reproduced map of the corpus: every source enumerated, its schema +
> content-block shapes + volumes + provenance documented — sufficient to design the
> normalized record model (arc02) without further spelunking.

**Verdict: delivered** (pending gate). The corpus map (`corpus-map.md`) documents
both source kinds and demonstrates a normalized record is draftable from it alone.

## Slice walk (3 = arc-plan breakdown; no silent drop)

| Slice | Outcome | Close |
|-------|---------|-------|
| slice01 — code-jsonl | ✅ delivered | CC doer / CDC verified — CLOSED (`slice01/cdc-verification.md`) |
| slice02 — desktop-export | ✅ delivered | CDC doer / CC verified — CLOSED (`slice02/cdc-verification.md`) |
| slice03 — corpus-map | ✅ delivered | CDC doer — synthesis; verified at this gate |

## Composition check (arc ledger A-4/A-5/A-6 — reproduced at arc scale)

- **A-4** — the map documents both sources' record schemas + content-block shapes.
  *Reproduce:* read `corpus-map.md` §2a/§2b against the two findings docs. ✅
- **A-5** — every DoD provenance dimension (account/machine/config-root/model/time/
  project/session-or-conversation) traces to a concrete field in ≥1 source.
  *Reproduce:* `corpus-map.md` §3a table. ✅
- **A-6** — arc02's normalized schema is draftable citing only the map.
  *Reproduce:* `corpus-map.md` §3b — the `NormalizedMessage` skeleton drafted with no
  re-spelunking. ✅

Composition verdict: the three slices **recompose** into the promised capability.
No arc-capability gap.

## Accumulated arc-plan changes

None. The 3-slice breakdown held from v1.0; no slice forced a re-slice or re-sequence.

## Bubble-up to the project

**1. Delivered its capability per `project-plan.md`?** Yes — arc02 is unblocked.

**2. Revealed that the project plan didn't anticipate?** (All absorbed; no new arc.)
- The corpus is a **set of roots** (multi-root/profile provenance dimension) with
  named **coverage gaps** (work Desktop account; second-machine Code roots pending).
- **Desktop is richer than assumed** (thinking/tool blocks) — corrected in discovery.
- **Live, self-mutating Code corpus** + **monotonic-merge** deletion policy.
  All are already reflected in `design-doc.md` (§3, §6, §10) and `corpus-map.md` §4;
  the roadmap (arcs 02–06) holds unchanged.

**3. Silent-drop diff at arc scale:** none — 3 planned slices, 3 delivered.

## Gate

**Independent check: PASS WITH NOTES** (fresh-context subagent, 2026-06-30). The
subagent reproduced A-4/A-5/A-6 against the artifacts (every schema/provenance cell
traced to the findings; the §3b sketch invents no fields; 3/3 slices closed; bubble-up
claims supported). One actionable note — the committed map omitted per-source volume
figures — **fixed** (`corpus-map.md` §1 now carries them). Two cosmetic notes noted,
not blocking: slice03 has no `cdc-verification.md` (by design — its independent check
IS this arc gate); design-doc §7 "slice02-recon" naming is now stale ("slice02").

**Operator sign-off: DONE (Duncan, 2026-06-30).** **arc01 CLOSED.** arc ledger
A-1…A-6 → done; project ledger **P-1 → done**; arc02 (classification) is now active.
