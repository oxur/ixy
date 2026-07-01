# Slice 02: desktop-export

## Ledger

_Doer = CDC (filled Evidence to `attested`, having run the Verify against the
mounted export). Verifier = CC (re-runs → `reproduced`). Files live in
`workbench/`: `conversations.json`, `projects/*.json`, `users.json`, `memories.json`._

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| D-1 | Volumes measured: conversations, messages, projects, bytes | `jq 'length'`; `jq '[.[].chat_messages\|length]\|add'`; `ls projects/*.json\|wc -l`; `du -sh` | correctness | slice-doc | done | attested: 820 convs / 9,838 msgs / 61 projects / 189 MB. §1. | |
| D-2 | Time-span measured (created_at min/max) | `jq -r '.[].created_at' \| sort \| sed -n '1p;$p'` | polish | slice-doc | done | attested: 2025-08-18 → 2026-06-26. §1. | ~10 months |
| D-3 | Conversation + message schema documented | `jq '.[0]\|keys'`; `jq '[…chat_messages[]?\|keys]\|add\|unique'` | serious | slice-doc | done | attested: conv 7 keys; msg 9 keys (`text` flattened + `content` block-array). §2. | drives arc02 |
| D-4 | Sender distribution documented | `jq -r '.[].chat_messages[]?.sender' \| sort \| uniq -c` | correctness | slice-doc | done | attested: human 4,939 / assistant 4,899. §1. | |
| D-5 | Provenance present/absent confirmed: account YES; model/cwd/version NO | `jq '.[0]\|has("account")'`; `jq '[…\|has("model")]\|any'`; `…has("cwd")` | serious | slice-doc | done | attested: `account`={uuid} present; `model`=false, `cwd`=false (schema also lacks version/gitBranch). §4. | the sparse-union axis |
| D-6 | Content-block structure documented (incl. thinking/tool blocks) | `jq -r '.[].chat_messages[]?.content\|type'\|sort\|uniq -c`; `jq '[…content[]?.type]\|unique'` | serious | slice-doc | done | attested: content array 100% (9,838); block types text/thinking/token_budget/tool_result/tool_use; attachments 287, files 583. §3. | **corrects earlier "plain-text-only"** |
| D-7 | Threading + branching characterized | `jq '[…\|select(.parent_message_uuid==null)]\|length'`; parent-referenced-by->1 count | serious | slice-doc | done | attested: 0 null-parent roots (→ sentinel root, not null); 53 branch points (→ trees, not lines). §5. | arc02 canonical-path |
| D-8 | Projects element characterized | `jq 'keys'` on a project; docs shape; count with docs | serious | slice-doc | done | attested: 61 projects; keys incl. prompt_template + docs[{uuid,filename,content,created_at}]; 15 carry docs. §6. | new corpus element |
| D-9 | users.json (PII) + memories.json shape documented | `jq '.[0]\|keys' users.json`; `jq` memories | serious | discovery | done | attested: users {email,full_name,uuid,verified_phone_number} = PII; memories {account_uuid, conversations_memory, project_memories}. §7–8. | PII → hygiene/§10 |
| D-10 | Findings written to `workbench/2026.06.30-discovery-desktop-export.md` with commands + redacted examples | file exists; command blocks + ≥1 redacted example per element | serious | slice-doc | done | attested: file exists; §9 has redacted conv/message/project examples; D-1…D-9 cross-referenced. | the artifact CC verifies |

## What Worked

- **Full-corpus `jq` (not a sample)** — 189 MB ran in seconds, so distributions
  (D-4, D-6) are exact, no sampling caveat.
- **Testing for field *absence*** (`has("model")|any`) turned "Desktop has no model"
  from assumption into a checked negative — and simultaneously caught the
  *positive* surprise that content carries thinking/tool blocks.
- **Counting branch parents** surfaced tree-threading (53 branch points) that a
  linear-thread assumption would have silently mismodeled in arc02.

## Closure

Closed at commit <pending> on 2026-06-30. Verifier: CC (seats swapped — CDC was doer; `cdc-verification.md`).
Rows: 10. Done: 10. Deferred: 0. No-op: 0.
Evidence strength: **all 10 rows `reproduced`** by CC against the mounted export
(static export — exact match, no drift band). D-6 attachment/file counts reconciled
(287/583 = messages-carrying vs 436/1065 = total items; findings §3 phrasing exact).
CC gate: **slice02 CLOSED**; arc ledger A-2 → done.
