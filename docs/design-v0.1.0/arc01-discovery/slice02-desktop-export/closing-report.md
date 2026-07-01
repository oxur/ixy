# slice02 — desktop-export — closing report (DOER = CDC)

**Seat:** CDC (doer — the export was mounted in the Cowork session; seats swapped
vs the normal CC-doer arrangement). **Verifier:** CC (`cdc-verification.md`).
**Date:** 2026-06-30. **Artifact:** `workbench/2026.06.30-discovery-desktop-export.md`.
**Ledger:** 10 rows. **Done: 10 · Deferred: 0 · No-op: 0.** All rows independently
**reproduced** by CC against the mounted export (static export — exact match).

This is the doer's per-row walk + bubble-up. Evidence detail lives in the ledger
and the findings artifact (§ refs).

## Per-row walk

- **D-1 volumes** · done — 820 convs / 9,838 msgs / 61 projects / 189 MB. §1.
- **D-2 time-span** · done — 2025-08-18 → 2026-06-26. §1.
- **D-3 schema** · done — conv 7 keys, msg 9 keys (`text` flattened + `content` array). §2.
- **D-4 sender** · done — human 4,939 / assistant 4,899. §1.
- **D-5 provenance** · done — `account`={uuid} present; `model`/`cwd`/`version` absent. §4.
- **D-6 content blocks** · done — array 100%; `text/thinking/token_budget/tool_result/tool_use`. §3.
  **Precision note (from CC's verify):** the ledger's "attachments 287, files 583" counts
  *messages carrying ≥1*; total *items* are 436 / 1,065. Both correct, different
  denominators; findings §3 states the message-count reading exactly. Recorded as
  reproduced under the message-count reading; the per-message-vs-total choice is an
  **arc02 note** so the normalizer doesn't rediscover it.
- **D-7 threading/branching** · done — 0 null-parent roots (sentinel root); 53 branch points (trees). §5.
- **D-8 Projects** · done — 61 projects; `prompt_template` + `docs[]`; 15 with docs. §6.
- **D-9 users/memories** · done — users PII `{email, full_name, uuid, phone}`; memories `{account_uuid, conversations_memory, project_memories}`. §7–8.
- **D-10 findings artifact** · done — exists with commands + redacted examples. §9.

## Silent-drop diff (scope-as-specified vs delivered)

Every "In" item (schema, content blocks, provenance present/absent, sender, volumes,
time-span, threading/branching, Projects, Memories, PII, findings artifact) is
delivered. "Out" items (arc02 schema, dedup/canonical-path, ingest code, work account,
other accounts) held. **No silent drops.** 10 opened / 10 dispositioned.

## Bubble-up to arc01 (Part IV)

**1. Did this slice deliver its assigned piece of arc01?** Yes — the Claude
**Desktop** source is characterized well enough to feed the corpus map (slice03) and
arc02's normalized model, alongside slice01's Code characterization.

**2. What did it reveal the arc-plan didn't anticipate?** (All arc02 inputs — none
change arc01's slice breakdown.)
- **Desktop carries `thinking`/`tool_use`/`tool_result` blocks** for tool-enabled
  accounts — the normalized schema unifies on *block type* across both sources, not
  "text-only vs structured." (This corrects an earlier *chat-only* characterization
  that Desktop was plain text; grep-verified it never reached a committed doc, so no
  plan-change is required — purely a recorded refinement.)
- **Tree threading + sentinel roots** — root-detection is not `parent == null`, and
  53 branch points mean conversations are trees; arc02 must choose a canonical path.
- **Projects + Memories are two corpus elements beyond conversations** — arc02 models
  or explicitly excludes each, with rationale.
- **`account` is an object `{uuid}`** joining to `users.json` (PII).
- **D-6 counting choice** (per-message vs total items) — arc02 normalizer note.

**3. Silent-drop diff:** none (table above).

## Arc-plan-change decision

**No arc01 arc-plan change.** The 3-slice breakdown stands; slice03 (corpus map) is
next and consumes this. All findings above are routed forward to **arc02** (and are
already captured in the design-doc §10 open questions where architectural). A-2 is
`done` (CC-gated).
