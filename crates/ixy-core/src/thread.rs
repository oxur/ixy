//! Source-agnostic threading position of a message.

use crate::ids::{BranchId, MessageId};

/// A message's position in its conversation thread, uniform across sources.
///
/// The two sources encode roots differently — Code uses a null parent, Desktop
/// uses a sentinel parent (corpus-map §3c) — but both collapse to the same
/// representation here: [`Thread::root`] yields `parent_id == None` and
/// `is_root == true`, so downstream root detection is a single source-agnostic
/// check. Desktop edits/regenerations fork a thread into a tree; the optional
/// [`BranchId`] tags which fork a non-root message sits on.
///
/// The invariant `is_root == parent_id.is_none()` is guaranteed by the
/// constructors — the fields cannot be set inconsistently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thread {
    parent_id: Option<MessageId>,
    is_root: bool,
    branch: Option<BranchId>,
}

impl Thread {
    /// A thread root — no parent. Used for both null-root (Code) and
    /// sentinel-root (Desktop); the normalizer maps either onto this.
    #[must_use]
    pub fn root() -> Self {
        Self {
            parent_id: None,
            is_root: true,
            branch: None,
        }
    }

    /// A non-root message with the given parent, on the main line.
    #[must_use]
    pub fn child(parent_id: MessageId) -> Self {
        Self {
            parent_id: Some(parent_id),
            is_root: false,
            branch: None,
        }
    }

    /// A non-root message on a named branch (a Desktop edit/regeneration fork).
    #[must_use]
    pub fn child_on_branch(parent_id: MessageId, branch: BranchId) -> Self {
        Self {
            parent_id: Some(parent_id),
            is_root: false,
            branch: Some(branch),
        }
    }

    /// The parent message id, or [`None`] for a root.
    #[must_use]
    pub fn parent_id(&self) -> Option<&MessageId> {
        self.parent_id.as_ref()
    }

    /// Whether this message is a thread root (source-agnostic).
    #[must_use]
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// The branch this message sits on, if it is on a non-main fork.
    #[must_use]
    pub fn branch(&self) -> Option<&BranchId> {
        self.branch.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_roots() {
        // Code root (source used null parent) and Desktop root (source used a
        // sentinel parent) both normalize to the same root shape.
        let code_root = Thread::root();
        let desktop_root = Thread::root();
        for root in [&code_root, &desktop_root] {
            assert!(root.is_root());
            assert!(root.parent_id().is_none());
        }

        // A child is not a root and carries its parent.
        let parent = MessageId::new("m-parent").unwrap();
        let child = Thread::child(parent.clone());
        assert!(!child.is_root());
        assert_eq!(child.parent_id(), Some(&parent));

        // A branched child records its fork.
        let branched = Thread::child_on_branch(parent, BranchId::new("regen-2").unwrap());
        assert!(!branched.is_root());
        assert!(branched.branch().is_some());
    }
}
