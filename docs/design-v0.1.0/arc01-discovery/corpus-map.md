# ixy — corpus map (arc01 discovery synthesis)

> The single map **arc02's normalized schema is written against.** Synthesizes the
> two closed, verified discovery slices — slice01 (Code JSONL) and slice02 (Desktop
> export) — into one cross-source model + the provenance mapping + the open
> questions arc02 inherits. Schema/design only (no personal data), so this doc is
> committed (unlike the `workbench/` findings it draws from).
>
> Sources: `slice01-code-jsonl/` (findings `workbench/2026.06.30-discovery-code-jsonl.md`),
> `slice02-desktop-export/` (findings `workbench/2026.06.30-discovery-desktop-export.md`),
> roster in `workbench/2026.06.30-discovery-corpus-structure.md`.

## 1. Sources roster (coverage)

Two source **kinds**, a **set** of roots/accounts/machines:

| Kind | Root/account | Machine | Volume (as characterized) | State |
|------|--------------|---------|---------------------------|-------|
| Code JSONL | `~/.claude` (default) | this machine | ~146 MB; 44 dirs / 114 files (live — drift band 43–45 / 104–115) | characterized (slice01) |
| Code JSONL | `~/.claude-bilt` (work profile) | second machine | pending | **pending** (reuse slice01 cc-prompt) |
| Code JSONL | `~/.claude-banyan` (work profile) | second machine | pending | **pending** |
| Desktop export | personal account `2df6bd1e…` | app-independent | 820 convs / 9,838 msgs / 189 MB; 2025-08-18 → 2026-06-26 | characterized (slice02) |
| Desktop export | work account | — | — | **coverage gap** (admin ≠ owner; cannot export) |

Same schema per kind, so pending roots need enumeration + volumes only, not
re-characterization.

## 2. Per-source schema (condensed)

### 2a. Claude Code JSONL (per-record, session-centric)

- **13 record `type`s**, two tiers: **4 envelope** (`assistant`, `user`,
  `attachment`, `system`) carry full provenance; **9 sidecar** (`ai-title`,
  `last-prompt`, `permission-mode`, `mode`, `file-history-snapshot`, `agent-name`,
  `queue-operation`, `pr-link`, `custom-title`) are session UI-state. Open/union
  schema, drifts across 10+ CLI versions.
- **Content blocks:** `text`, `thinking`, `tool_use`, `tool_result` (in
  `message.content` arrays).
- **Threading:** `uuid` / `parentUuid` linked list; **`logicalParentUuid`** bridges
  compaction boundaries; `sourceToolAssistantUUID` joins tool_result → tool_use.
  Roots: `parentUuid == null`.
- **Provenance:** `cwd` (authoritative — dir name is lossy), `sessionId` (==filename),
  `gitBranch`, `version`, `timestamp` (ISO-8601), `message.model` (assistant only).
  `file-history-snapshot` has **no** `sessionId`.
- **Also:** `history.jsonl` (global prompt timeline; epoch-ms).

### 2b. Claude Desktop export (per-conversation, conversation-centric)

- **conversation** `{uuid, name, summary, account{uuid}, created_at, updated_at,
  chat_messages[]}`; **message** `{uuid, parent_message_uuid, sender, text,
  content[], attachments[], files[], created_at, updated_at}`.
- **Content blocks:** array 100%; `text`, `thinking`, `token_budget`, `tool_use`,
  `tool_result` (**Desktop carries thinking/tool blocks too** — for tool-enabled
  accounts).
- **Threading:** `parent_message_uuid` **trees** (53 branch points from
  edits/regens); roots use a **sentinel** parent (not null).
- **Provenance:** `account{uuid}` (join → `users.json` PII); `sender` ∈ {human,
  assistant}; `created_at`/`updated_at`. **Absent:** `model`, `cwd`, `version`,
  `gitBranch`.
- **Also:** `projects/*.json` (61; `prompt_template` + `docs[]`, 15 with docs);
  `memories.json` (`{account_uuid, conversations_memory, project_memories}`);
  `users.json` (PII).

## 3. Cross-source reconciliation → the normalized model

### 3a. Provenance dimension → source field (A-5)

| Dimension | Code JSONL | Desktop export |
|-----------|------------|----------------|
| account | — | `conversation.account.uuid` |
| machine | (host of the root) | (export is app-independent) |
| config-root/profile | the `~/.claude*` root | — |
| model | `message.model` (assistant) | — |
| timestamp | `timestamp` (ISO-8601) | `created_at`/`updated_at` |
| project | `cwd` (from record) | — (Projects are separate entities) |
| session/conversation | `sessionId` | `conversation.uuid` |

**Sparse union confirmed:** no source fills every cell. Code supplies
model/cwd/version/branch; Desktop supplies account + human-named conversation. The
normalized record must treat every provenance field as optional-by-source.

### 3b. Unified message (sketch — the A-6 draftability demonstration)

Drafted citing **only** this map (no re-spelunking) — evidence arc02's schema is
derivable:

```
NormalizedMessage {
  id            // source msg uuid
  source        // Code | Desktop
  provenance    // sparse: {account?, machine?, config_root?, model?, project_cwd?,
                //          session_or_conversation, timestamp}
  role          // human/user | assistant | system | tool   (sender/type unified)
  blocks[]      // unified: text | thinking | tool_use | tool_result | (token_budget, meta)
  thread        // { parent_id, is_root, branch? }   ← null-parent (Code) OR sentinel (Desktop)
  raw_ref       // pointer back to the bedrock record (never lossy)
}
```

Non-message elements modeled separately: **Project** (`prompt_template` + docs),
**Memory** (account memory), **Code sidecars** (titles, history.jsonl) — index-or-exclude
decided in arc02.

### 3c. Reconciliation notes for arc02

- **Roles:** Code `type` vs Desktop `sender` unify to a role enum; tool_use/tool_result
  are *blocks*, not roles.
- **Threading:** two shapes — Code linked-list (+ compaction bridge), Desktop trees
  (+ sentinel root). Canonical-path selection is an arc02 decision (§4).
- **Root detection differs by source** (null vs sentinel) — must not assume one rule.

## 4. Open questions inherited by arc02/arc03

Carried forward (also in design-doc §10 where architectural):

1. **Dedup / identity / canonical path** (arc02) — superset-merge on `(uuid,
   updated_at)`; Desktop trees ⇒ pick canonical reading path (latest-leaf?).
2. **Monotonic corpus** (arc02/arc04) — a conversation gone from a newer export is
   **kept**, not deleted (deletion policy inverts odm's).
3. **Chunking + what-to-embed** (arc03) — heterogeneous blocks; skip raw tool stdout,
   keep human/assistant/thinking.
4. **Projects + Memories** (arc02) — model or exclude, with rationale.
5. **Secrets/PII** (arc02/arc04) — tool_result stdout + `users.json` → scrub/encrypt posture.
6. **D-6 counting** (arc02) — per-message vs total-items for attachments/files.
7. **Sentinel-root detection** (arc02) — Desktop roots are not null.

## 5. Verdict

Both source kinds are documented to record-schema + content-block + provenance
depth; every DoD provenance dimension traces to a concrete field in ≥1 source
(§3a); and a normalized record is draftable from this map alone (§3b). This
satisfies arc01's capability — arc02 can begin its schema design against this
document without re-opening the raw corpus.
