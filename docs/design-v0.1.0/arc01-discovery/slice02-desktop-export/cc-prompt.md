# CC assignment — slice02: desktop-export (VERIFICATION, seats swapped)

You are **CC**, and for this slice you are the **independent verifier**, not the
implementer. CDC was the doer (the Desktop export is mounted in the Cowork
session, so CDC ran the characterization). Your job is to **reproduce** CDC's
`attested` evidence → `reproduced`, exactly as CDC does for your slices. Load the
**collaboration-framework**; this runs under **ledger discipline**.

## What to verify

The ledger is `docs/design-v0.1.0/arc01-discovery/slice02-desktop-export/ledger.md`
(10 rows, D-1…D-10, all `attested`). The findings artifact is
`workbench/2026.06.30-discovery-desktop-export.md`. The data files are in
`workbench/` (reachable by you): `conversations.json`, `projects/*.json`,
`users.json`, `memories.json`.

## Protocol

1. **Re-run each row's Verify command** against the `workbench/` files and confirm
   the captured result. The commands are in the ledger's Verify column and in the
   findings artifact's code blocks. Key numbers to reproduce:
   - D-1: 820 conversations, 9,838 messages, 61 projects.
   - D-4: human 4,939 / assistant 4,899.
   - D-5: `account` present; `model` / `cwd` **absent** (`has(...)|any` → false).
   - D-6: `content` array 100%; block types incl. `thinking`/`tool_use`/`tool_result`.
   - D-7: **0** null-parent roots; **53** branch points.
2. **Flag any mismatch.** Exact reproduction is expected — this is a static export
   (not the live Code corpus), so unlike slice01 the numbers should match exactly,
   with no drift band. A discrepancy is a real finding.
3. **Write `cdc-verification.md`** in the slice dir (canonical filename; note inside
   that the verifier seat is CC, doer was CDC). Give a per-row verdict:
   `reproduced` where you re-ran it, and call out anything you could not reproduce.
4. **Verify the bubble-up** in the findings §10 is honest: Desktop *does* carry
   thinking/tool blocks; threading is a tree with sentinel roots; Projects +
   Memories are distinct elements; `account` is `{uuid}`.
5. On clean verification, state the gate: **slice02 CLOSED**, arc ledger **A-2 → done**.

## Out of scope

Normalized schema design (arc02), dedup/canonical-path decisions (arc02), any code.
Do not re-characterize — verify what's there.
