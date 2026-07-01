//! The closed taxonomies a message is classified by: its [`Source`] and [`Role`].
//!
//! Both are `#[non_exhaustive]` (guideline TD-07): the corpus spans multiple
//! Claude surfaces and the discovery arc may yet surface a role or source the
//! current model does not name, so downstream `match`es must carry a wildcard.

/// Which store a normalized message was recovered from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Source {
    /// Claude Code per-session JSONL (`~/.claude*/projects/**/*.jsonl`).
    Code,
    /// Claude Desktop account export (`conversations.json`).
    Desktop,
}

/// The role of a message's author, unified across sources.
///
/// Code's record `type` and Desktop's `sender` both map onto this enum; tool
/// invocations and their results are *content blocks* (see [`Block`]), not roles.
///
/// [`Block`]: crate::Block
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Role {
    /// A human turn (Code `user` prompt, Desktop `human`).
    Human,
    /// An assistant turn.
    Assistant,
    /// A system message.
    System,
    /// A tool-role turn (tool output surfaced as its own message).
    Tool,
}
