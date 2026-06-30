# slice01 — code-jsonl — CC closing report

**Seat:** CC (implementation, operator trust domain)
**Date:** 2026-06-30
**Artifact:** `workbench/2026.06.30-discovery-code-jsonl.md`
**Ledger:** 10 rows. **Done: 10 · Deferred: 0 · No-op: 0.** All evidence
`attested` by CC; awaiting CDC `reproduced` pass.

This is the per-row walk. Evidence strength is `attested` throughout (CC ran the
Verify and captured output; CDC reproduces). Section refs point into the findings
artifact above.

---

## Per-row walk

### F-1 — Volumes measured · **done** (correctness)
Snapshot: **43 project dirs / 104 `.jsonl` files / 146 MB.**
Verify: `ls -1d ~/.claude/projects/*/ | wc -l`; `find … -name '*.jsonl' | wc -l`;
`du -sh ~/.claude/projects`.
**Done with caveat:** the store is **live**. Three passes during this slice read
45/115, then 43/104; the ledger's expected 44/114 sits *inside* that drift band.
The number is a moving snapshot, not a fixed total — reported as such (§1). Not
softpedalled: I am calling this `done` on the *measurement*, while flagging that
no single fixed value exists to measure.

### F-2 — Time-span measured · **done** (polish)
Oldest session file `2026-05-11`, newest `2026-06-30` (mtime).
Verify: `find … -exec stat -f '%Sm' -t '%Y-%m-%d' {} \; | sort | sed -n '1p;$p'`.
Caveat: mtime, not record `timestamp` — a file's last-write, not first-message.
Adequate for span; noted (§1).

### F-3 — Union schema + version variation · **done** (serious)
60-key union documented; **`type` is the only key on every record**; schema
drifts across **10+ `version`s** (`2.1.139`…`2.1.195`). Four variation axes named
(version drift; sub-agent/sidechain attribution; the two `user` shapes;
`system`-by-`subtype`). Ran over the **full corpus**, not the recon's `head -50`
sample (cheap at 146 MB), so no sampling caveat. §4.

### F-4 — Record-type distribution · **done** (correctness)
13 types, full-corpus counts: assistant 27,092 · user 16,617 · ai-title 3,310 ·
last-prompt 3,290 · attachment 2,957 · permission-mode 2,945 · mode 2,227 ·
file-history-snapshot 1,600 · system 1,234 · agent-name 496 · queue-operation
188 · pr-link 39 · custom-title 4. Surfaced the **two-tier model** (4 envelope
types vs 9 sidecar). §3.

### F-5 — Provenance fields present · **done** (serious)
Verify: presence-by-type table (§5). **Done with an important caveat the row's
phrasing hides:** the six fields are **not uniformly present**.
- `cwd`, `gitBranch`, `version`, `parentUuid`, `timestamp`, `uuid` → only on the
  4 envelope types (assistant/user/attachment/system).
- Sidecar types carry **`sessionId` only**.
- `file-history-snapshot` carries **no `sessionId`** (keys on `messageId`).
- `model` lives at **`message.model`** (assistant only), not top-level.
This non-uniformity is exactly what arc02's normalizer must absorb — flagged
loudly rather than reported as a clean "all six present."

### F-6 — Content-block structure · **done** (serious)
`message.content` is a **string** (human prompt, 794) **or** a typed-block
**array**. Block counts: tool_result 13,827 · tool_use 13,821 · text 7,477 ·
thinking 2,774; plus mixed-block assistant turns (`thinking+text+tool_use`) and
rare `image+text`. §6a.

### F-7 — Path-encoding lossiness + cwd rule · **done** (correctness)
Worked example: dir `…lykn-lang--worktrees-cdc-build-dir-reorg` → record `cwd`
`/Users/oubiwann/lab/lykn/lang/.worktrees/cdc-build-dir-reorg`. Two distinct
losses proven: the `--` collides `-`(literal)+`/`(separator), **and** the leading
`.` of `.worktrees` is dropped. Unrecoverable from the dir name. Rule recorded:
**read `cwd` from a record; never decode the dir name.** §2.

### F-8 — `history.jsonl` shape · **done** (polish)
4 keys: `{display, pastedContents, timestamp, project}`; 7,382 lines / 2.4 MB.
**Two gotchas captured:** `timestamp` is **epoch-ms** (vs ISO-8601 inside session
files), and `project` is the **un-mangled** path (join key to `cwd`). Prompts
only — no `sessionId`, no replies; complements, does not replace, session files.
§7a.

### F-9 — IndexedDB peek (stretch) · **done** (polish) — *not deferred*
Probed the claude.ai leveldb (1.2 MB). It holds only **`tipTapEditorState`
compose-box drafts** in a `keyval-store`; **zero** transcript/sender/conversation
records. **Answer to the row's question: Desktop transcripts are NOT cached
locally.** That is a load-bearing *no* (it confirms slice02's account-export path
is necessary), so I ran the stretch to a real answer rather than deferring it.
The deeper ldb reverse-engineering is a dead end, not undone work. §7b.

### F-10 — Findings artifact written · **done** (serious)
`workbench/2026.06.30-discovery-code-jsonl.md` exists; contains reproducible
command blocks throughout and **≥1 redacted example for each of all 13 record
types** (§7), with F-1…F-9 cross-referenced. The handoff channel to CDC.

---

## Silent-drop diff (scope-as-specified vs scope-as-delivered)

Walked the slice-doc "In" list against delivery:

| Spec item | Delivered? |
|---|---|
| Record-level schema + version variation | ✅ F-3 |
| Record-type distribution | ✅ F-4 |
| Provenance fields per record | ✅ F-5 (with non-uniformity caveat) |
| Content-block structure | ✅ F-6 |
| Volumes + time-span | ✅ F-1/F-2 |
| Path-encoding gotcha | ✅ F-7 |
| `history.jsonl` shape | ✅ F-8 |
| Stretch: IndexedDB peek | ✅ F-9 (answered NO) |
| (Out) Desktop export, arc02 schema, ingest code | correctly excluded |
| (Out) sibling stores — *note relevance only* | ✅ noted, not characterized (§7c) |

**No silent drops.** 10 opening rows, 10 dispositioned. Nothing in the "In" list
went unaddressed; nothing in the "Out" list was pulled in.

---

## Bubble-up to arc01 (PROJECT-MANAGEMENT.md Part IV)

**1. Did this slice deliver its assigned piece of arc01?**
Yes. arc01 (discovery) needed the Claude **Code** source characterized well enough
to feed arc02's normalized record model. The slice delivers the union schema, the
type taxonomy, the provenance-field map (with its non-uniformity), the
content-block shapes, and the path-encoding rule — plus the `history.jsonl` and
IndexedDB answers. That is the Code-source input to the corpus map.

**2. What did it reveal that the arc-plan didn't anticipate?**
- **The corpus is *live and self-mutating*.** Volume counts drift between scans
  within one session (45/115 → 43/104), and the running session writes into the
  very store being scanned. arc02 ingest must be snapshot-tolerant
  (files appear/grow/vanish mid-scan) — this is a new design constraint, not in
  the arc-plan.
- **Provenance is non-uniform across record types**, not a flat field set. The
  normalizer can't assume `{cwd, sessionId, timestamp, model, parentUuid,
  version}` are all present; sidecars carry `sessionId` only and
  `file-history-snapshot` carries *no* `sessionId`. The arc-plan's "the keys
  arc02 normalizes on" framing implied uniformity that doesn't hold.
- **Schema drifts across 10+ CLI versions** — arc02 must treat the record as an
  open/union type keyed on `version`, not a closed struct.
- **Richer type taxonomy than the slice-doc enumerated.** The doc listed
  user/assistant/summary/tool_use/tool_result; reality is 13 top-level `type`s
  where tool_use/tool_result are *content blocks*, not record types, and there
  are 9 sidecar types (ai-title, mode, permission-mode, queue-operation, pr-link,
  …) the plan didn't name.
- **F-9 is a load-bearing NO that de-risks slice02:** Desktop transcripts are not
  locally cached, so the account-export path is *confirmed necessary*, not merely
  assumed.

**3. Silent-drop diff:** none (table above). 10/10 rows dispositioned, "In" scope
fully delivered, "Out" scope held.

---

## Handoff to CDC

Reproduce from the mounted `workbench/` artifact: re-run any Verify command (they
are in the report's code blocks) against `~/.claude/projects` and confirm the
captured output. Expect F-1's counts to differ from mine — that *is* the F-1
finding; confirm yours also fall in the drift band rather than matching exactly.
Then write `cdc-verification.md` taking each row `attested` → `reproduced`.
