//! `ixy-core` — the normalized record model for the `ixy` conversation corpus.
//!
//! `ixy` recovers **I(X;Y)** between a query and a personal LLM conversation
//! history that spans two very different stores: Claude Code's per-session JSONL
//! and Claude Desktop's per-account export. This crate is the keystone that lets
//! everything downstream (classification, analysis, storage, indexing) build on
//! **one** record shape instead of two source schemas.
//!
//! The model is designed against the arc01 discovery synthesis
//! (`docs/design-v0.1.0/arc01-discovery/corpus-map.md`). This crate is **types,
//! invariants, and tests only** — the normalizers that populate these types, and
//! any persistence, live in later slices.
//!
//! # The model
//!
//! [`NormalizedMessage`] is the flagship type. It composes:
//!
//! - [`MessageId`] and the other validating id newtypes ([`SessionId`],
//!   [`AccountId`], [`ModelId`], [`MachineId`], [`Timestamp`], [`BranchId`],
//!   [`Locator`]) — no bare-string ids (TD-03).
//! - [`Source`] and [`Role`] — closed, `#[non_exhaustive]` taxonomies.
//! - [`Block`] — the unified content-block enum, whose [`Debug`] redacts bodies.
//! - [`Provenance`] — a **sparse union**: every source-specific dimension is
//!   optional, anchored by the always-present session/conversation id.
//! - [`Thread`] — source-agnostic threading; null-root (Code) and sentinel-root
//!   (Desktop) both collapse to one root shape.
//! - [`RawRef`] — a mandatory, non-lossy pointer back to the bedrock record.
//!
//! # Invariants
//!
//! - **Non-lossy:** a [`NormalizedMessage`] cannot be built without a [`RawRef`].
//! - **Redaction:** [`Debug`] on messages and blocks never prints body content
//!   (message text, thinking, tool output may hold secrets/PII — design-doc §10).
//! - **Sparse provenance:** source-specific fields are `Option`, never faked.
//! - **No `unsafe`:** the crate forbids it (see the workspace lints).

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::missing_crate_level_docs)]
// Natural module/type pairings (`raw_ref::RawRef`, `message::NormalizedMessage`)
// read better than contorted names; the public API is flat via the re-exports
// below, so the repetition never reaches callers.
#![allow(clippy::module_name_repetitions)]

mod block;
mod ids;
mod message;
mod provenance;
mod raw_ref;
mod taxonomy;
mod thread;

pub use block::Block;
pub use ids::{
    AccountId, BranchId, IdError, Locator, MachineId, MessageId, ModelId, SessionId, Timestamp,
};
pub use message::NormalizedMessage;
pub use provenance::Provenance;
pub use raw_ref::RawRef;
pub use taxonomy::{Role, Source};
pub use thread::Thread;
