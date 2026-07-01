//! Strongly-typed string identifiers and leaf values for the record model.
//!
//! Every stringly-typed value in the corpus (message uuids, session/conversation
//! ids, account ids, model names, timestamps, bedrock locators) is wrapped in a
//! validating newtype rather than passed around as a bare [`String`] (guideline
//! TD-03, anti-pattern AP-29). Construction rejects empty or whitespace-only
//! input, so an "id" can never silently be blank.

use std::fmt;

/// Error returned when an identifier newtype is constructed from invalid input.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum IdError {
    /// The supplied value was empty or contained only whitespace.
    #[error("{kind} must not be empty or blank")]
    Empty {
        /// The newtype that rejected the value (e.g. `"MessageId"`).
        kind: &'static str,
    },
}

/// Defines a validated, non-empty string newtype with the standard derives,
/// a fallible `new`, `as_str` / `into_inner` accessors, and a [`Display`] impl.
///
/// [`Display`]: std::fmt::Display
macro_rules! string_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        ///
        /// Constructed via [`new`](Self::new), which rejects empty/blank input.
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            /// Creates the identifier from any string-like value.
            ///
            /// # Errors
            ///
            /// Returns [`IdError::Empty`] if `value` is empty or only whitespace.
            pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(IdError::Empty { kind: stringify!($name) });
                }
                Ok(Self(value))
            }

            /// Borrows the identifier as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the newtype, returning the inner [`String`].
            #[must_use]
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

string_newtype! {
    /// Identifier of a single normalized message — the source message uuid
    /// (Code record `uuid`, Desktop `chat_messages[].uuid`).
    MessageId
}

string_newtype! {
    /// Identifier of the containing thread: a Code `sessionId` or a Desktop
    /// `conversation.uuid`. Present for every message in both sources.
    SessionId
}

string_newtype! {
    /// Account identifier — the Desktop export's `account.uuid`, which joins to
    /// the account's `users.json`. Absent for Code records.
    AccountId
}

string_newtype! {
    /// Model identifier — the Code assistant `message.model` string
    /// (e.g. `claude-opus-4-8`). Absent for Desktop records.
    ModelId
}

string_newtype! {
    /// Identifier of the machine/host a record originated on.
    MachineId
}

string_newtype! {
    /// An opaque, source-format timestamp string (ISO-8601 for Code, `created_at`
    /// for Desktop). Parsing into a structured instant is deferred to a later
    /// slice; ISO-8601 UTC sorts correctly lexicographically in the meantime.
    Timestamp
}

string_newtype! {
    /// A branch discriminator for Desktop edit/regeneration forks. Provisional —
    /// the canonical reading-path semantics are an arc02 decision.
    BranchId
}

string_newtype! {
    /// An opaque pointer into a bedrock source record, e.g. `file.jsonl#L42`
    /// (Code) or `conversations.json#<conv-uuid>/<msg-uuid>` (Desktop).
    Locator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_and_blank() {
        assert!(MessageId::new("").is_err());
        assert!(MessageId::new("   ").is_err());
        assert_eq!(
            SessionId::new("\t\n").unwrap_err(),
            IdError::Empty { kind: "SessionId" }
        );
    }

    #[test]
    fn accepts_and_round_trips() {
        let id = MessageId::new("abc-123").unwrap();
        assert_eq!(id.as_str(), "abc-123");
        assert_eq!(id.clone().into_inner(), "abc-123");
        assert_eq!(id.to_string(), "abc-123");
    }
}
