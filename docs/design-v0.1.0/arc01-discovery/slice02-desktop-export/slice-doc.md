# slice02 — desktop-export (slice doc)

## Goal

Produce a documented, reproduced characterization of a **Claude Desktop account
export** (personal account), sufficient to feed the normalized record model
(arc02) alongside slice01's Code characterization. Output: the findings artifact
`workbench/2026.06.30-discovery-desktop-export.md` and a closed ledger.

## Seats (swapped vs slice01 — noted per LEDGER-DISCIPLINE)

The export is mounted in `workbench/`, so unlike slice01 the Cowork session has
direct access. Therefore for this slice:

- **Doer = CDC** (characterizes; fills the ledger to `attested`).
- **Verifier = CC** (re-runs the Verify commands in the operator trust domain →
  `reproduced`; writes the verification).

The canonical close-set filenames are kept: `closing-report.md` = the doer's
per-row walk + bubble-up (CDC here); `cdc-verification.md` = the independent
verifier's check (CC here — the filename is canonical, the seat is noted inside).

## Scope

**In:** conversation + message schema; content-block structure (incl. thinking /
tool_use / tool_result); provenance fields present *and absent* (account vs
model/cwd/version); sender distribution; volumes + time-span; threading + branching;
the Projects element (`projects/*.json`); the Memory element (`memories.json`);
the PII in `users.json`.

**Out:** the normalized schema design itself (arc02); dedup/identity/canonical-path
decisions (arc02); any ingest code; the *work* account (known coverage gap — cannot
export); other accounts (not yet exported).

## Verification approach

CDC ran the `jq` characterization against the mounted export and captured
reproducible commands + outputs into the findings artifact, filling the ledger to
`attested`. CC (verifier) re-runs the same commands against the same files (they
live in `workbench/`, reachable by CC) and confirms the numbers + schema →
`reproduced`, then writes `cdc-verification.md`. Any mismatch is a finding.

## Exit criteria

All ledger rows reach a final status; the Desktop-source inputs to the corpus map
(slice03) are complete and reproduced; no silent drops (close row-count = open
row-count). Bubble-up findings (thinking/tool blocks in Desktop; tree threading;
Projects + Memories elements) recorded for arc02.
