# slice02 — desktop-export — verification

**Seat note (seats swapped):** verifier = **CC**, doer = **CDC**. The filename is
the canonical `cdc-verification.md`, but for this slice CC is the independent
reviewer (the Desktop export is mounted in CC's trust domain, so CC can re-run the
Verify commands; CDC did the original characterization). This is the mirror of
slice01, where CDC verified CC.

**Date:** 2026-06-30
**Verifies:** CDC closing evidence + ledger (10 rows D-1…D-10, all `done`/`attested`).
**Verdict:** **slice02 gated CLOSED.** All 10 rows independently **reproduced**
against `workbench/{conversations.json, projects/*.json, users.json, memories.json}`.
No silent drops, no spec-softening. One counting-semantics ambiguity surfaced
(D-6) — reconciled, not a data discrepancy.

## Method

Unlike slice01 (a live, self-mutating store with a drift band), this is a **static
export** — so exact reproduction is the bar, and I met it. I re-ran each row's
Verify against the mounted files; I did not re-read CDC's captured output. Every
headline number the cc-prompt called out matched to the digit.

## Per-row verdict

| Row | CDC | CC strength | Reproduced result |
|-----|-----|-------------|-------------------|
| D-1 volumes | done | **reproduced** | 820 convs · 9,838 msgs · 61 projects · 189 MB — exact. |
| D-2 time-span | done | **reproduced** | `2025-08-18T00:33:27Z` → `2026-06-26T05:49:14Z` — exact. |
| D-3 schema | done | **reproduced** | conv 7 keys `{account,chat_messages,created_at,name,summary,updated_at,uuid}`; msg 9 keys `{attachments,content,created_at,files,parent_message_uuid,sender,text,updated_at,uuid}` — exact. |
| D-4 sender dist | done | **reproduced** | human 4,939 / assistant 4,899 — exact. |
| D-5 provenance | done | **reproduced** | `has("account")`=true, `account`=`{uuid}`; `has("model")|any`=false, `has("cwd")|any`=false, `has("version")|any`=false — exact. The sparse-union negative confirmed. |
| D-6 content blocks | done | **reproduced** (w/ note) | content array 100% (9,838/9,838); block types `[text,thinking,token_budget,tool_result,tool_use]` — exact. Attachment/file counts: see note below. |
| D-7 threading | done | **reproduced** | 0 null-parent roots; 53 branch points (parents referenced by >1 child) — exact. Tree-with-sentinel-root confirmed. |
| D-8 projects | done | **reproduced** | 61 projects; keys include `prompt_template` + `docs`; 15 carry docs; doc entry keys `{content,created_at,filename,uuid}` — exact. |
| D-9 users/memories | done | **reproduced** | `users.json` (array, len 1) → `{email_address,full_name,uuid,verified_phone_number}` = PII; `memories.json` (array, len 1) → `{account_uuid,conversations_memory,project_memories}` — exact. |
| D-10 findings artifact | done | **reproduced** | File exists in mounted `workbench/`; §9 carries redacted examples for conversation / human msg / assistant-with-tool+thinking / project; D-1…D-9 cross-referenced. Read directly. |

## The one thing worth flagging — D-6 counting semantics (reconciled)

The ledger D-6 evidence line reads "attachments 287, files 583." My first
reproduction summed per-message array lengths and got **436 / 1065**, which looked
like a mismatch. It is not a data discrepancy — it is two different denominators:

- **436 attachments / 1065 files** = total *items* (`[.chat_messages[]?.attachments|length]|add`).
- **287 / 583** = *messages carrying ≥1* attachment/file (`select((.attachments|length)>0)|length`).

Both are correct. The findings artifact §3 states it **precisely** — "287 messages
carry `attachments`; 583 carry `files`" — so the artifact is not softpedalled; only
the ledger's terser note could be misread. Reproduced under the message-count
reading.

**Arc02 note (not a slice02 defect):** the normalizer will need to decide which
count it means when it reports attachment/file coverage — per-item or per-message.
Recording it here so it isn't rediscovered downstream.

## Bubble-up honesty check (cc-prompt step 4)

Findings §10 verified against my re-derivation — all four claims hold:

1. **Desktop carries `thinking`/`tool_use`/`tool_result` blocks** ✓ — block-type set
   reproduced exactly; the correction to the earlier "plain-text-only" assumption is
   real and load-bearing for arc02's unified block model.
2. **Tree threading + sentinel roots** ✓ — 0 null roots, 53 branch points reproduced.
3. **Projects + Memories are distinct corpus elements** ✓ — both present with the
   documented shapes; two elements beyond conversations for arc02 to model or exclude.
4. **`account` is an object `{uuid}`** ✓ — joins to `users.json` (PII), reproduced.

## Silent-drop check

10 opening rows, 10 dispositioned, all reproduced. No missing rows. The one
"Out of scope" boundary (no re-characterization, no schema design) was respected —
I verified what was there and did not extend it.

## Gate

**slice02 CLOSED.** Arc ledger **A-2 (slice02 closed cleanly) → done.**
Evidence strength: all 10 rows **reproduced** (static export, exact match — stronger
than slice01, which was `reproduced` on 4 and `attested` on 6 due to the
trust-domain reproduction limit; here I *can* reach the data, so all 10 reproduce).
