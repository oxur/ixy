//! The flagship type: a single message normalized from either source.

use std::fmt;

use crate::block::Block;
use crate::ids::{MessageId, Timestamp};
use crate::provenance::Provenance;
use crate::raw_ref::RawRef;
use crate::taxonomy::{Role, Source};
use crate::thread::Thread;

/// A single message, normalized from Claude Code or Claude Desktop into one
/// uniform shape that everything downstream builds on.
///
/// The type both source normalizers (arc02 slice02/03) target. Its
/// [`source`](Self::source) is derived from its [`raw_ref`](Self::raw_ref) rather
/// than stored twice, so the source is single-sourced and cannot disagree with
/// the bedrock pointer.
///
/// # Redaction
///
/// Message content lives in [`Block`]s, whose [`Debug`] redacts bodies; this
/// type's [`Debug`] therefore never prints secret content either (guideline
/// API-18). See [`Block`] for the redaction contract.
///
/// # Examples
///
/// ```
/// use ixy_core::{
///     Block, MessageId, NormalizedMessage, Provenance, RawRef, Role,
///     SessionId, Source, Thread, Timestamp, Locator, ModelId,
/// };
///
/// let msg = NormalizedMessage::new(
///     MessageId::new("m-1")?,
///     Provenance::new(SessionId::new("sess-1")?)
///         .with_model(ModelId::new("claude-opus-4-8")?),
///     Role::Assistant,
///     vec![Block::Text { text: "hello".into() }],
///     Thread::root(),
///     Timestamp::new("2026-06-30T00:00:00Z")?,
///     RawRef::new(Source::Code, Locator::new("session.jsonl#L1")?),
/// );
///
/// assert_eq!(msg.source(), Source::Code); // derived from raw_ref
/// assert!(msg.thread().is_root());
/// # Ok::<(), ixy_core::IdError>(())
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct NormalizedMessage {
    id: MessageId,
    provenance: Provenance,
    role: Role,
    blocks: Vec<Block>,
    thread: Thread,
    timestamp: Timestamp,
    raw_ref: RawRef,
}

impl NormalizedMessage {
    /// Assembles a normalized message from its parts.
    ///
    /// `raw_ref` is mandatory — this is the only constructor, and it takes a
    /// [`RawRef`] by value, so a message can never exist without a pointer back
    /// to its bedrock record (the non-lossy invariant). The message's
    /// [`Source`] is taken from `raw_ref`.
    #[must_use]
    pub fn new(
        id: MessageId,
        provenance: Provenance,
        role: Role,
        blocks: Vec<Block>,
        thread: Thread,
        timestamp: Timestamp,
        raw_ref: RawRef,
    ) -> Self {
        Self {
            id,
            provenance,
            role,
            blocks,
            thread,
            timestamp,
            raw_ref,
        }
    }

    /// The message's own identifier.
    #[must_use]
    pub fn id(&self) -> &MessageId {
        &self.id
    }

    /// Which store the message was normalized from (derived from the raw ref).
    #[must_use]
    pub fn source(&self) -> Source {
        self.raw_ref.source()
    }

    /// The message's provenance (sparse across sources).
    #[must_use]
    pub fn provenance(&self) -> &Provenance {
        &self.provenance
    }

    /// The author role.
    #[must_use]
    pub fn role(&self) -> Role {
        self.role
    }

    /// The content blocks.
    #[must_use]
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// The threading position.
    #[must_use]
    pub fn thread(&self) -> &Thread {
        &self.thread
    }

    /// The message timestamp.
    #[must_use]
    pub fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    /// The mandatory pointer back to the bedrock record.
    #[must_use]
    pub fn raw_ref(&self) -> &RawRef {
        &self.raw_ref
    }
}

impl fmt::Debug for NormalizedMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Redaction: every field here is safe to show — the only secret-bearing
        // field is `blocks`, and `Block`'s own `Debug` redacts bodies.
        f.debug_struct("NormalizedMessage")
            .field("id", &self.id)
            .field("source", &self.source())
            .field("role", &self.role)
            .field("timestamp", &self.timestamp)
            .field("thread", &self.thread)
            .field("provenance", &self.provenance)
            .field("raw_ref", &self.raw_ref)
            .field("block_count", &self.blocks.len())
            .field("blocks", &self.blocks)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{Locator, SessionId};

    fn sample() -> NormalizedMessage {
        NormalizedMessage::new(
            MessageId::new("m-1").unwrap(),
            Provenance::new(SessionId::new("sess-1").unwrap()),
            Role::Human,
            vec![Block::Text {
                text: "SECRET-BODY-xyz".into(),
            }],
            Thread::root(),
            Timestamp::new("2026-06-30T00:00:00Z").unwrap(),
            RawRef::new(Source::Code, Locator::new("session.jsonl#L1").unwrap()),
        )
    }

    #[test]
    fn raw_ref_required() {
        // `new` is the sole constructor and demands a `RawRef` by value; a
        // message therefore always carries its bedrock pointer.
        let msg = sample();
        assert_eq!(msg.raw_ref().source(), Source::Code);
        assert_eq!(msg.raw_ref().locator().as_str(), "session.jsonl#L1");
        // Source is single-sourced from the raw ref.
        assert_eq!(msg.source(), msg.raw_ref().source());
    }

    #[test]
    fn debug_redacts() {
        let rendered = format!("{:?}", sample());
        assert!(
            !rendered.contains("SECRET-BODY-xyz"),
            "body leaked: {rendered}"
        );
        // Non-secret structure is still present.
        assert!(rendered.contains("NormalizedMessage"));
        assert!(rendered.contains("block_count"));
    }
}
