//! Message content blocks — the unified block model across both sources.

use std::fmt;

/// A single content block within a message.
///
/// Both sources represent a message body as an array of typed blocks; this enum
/// is their union (`text`, `thinking`, `tool_use`, `tool_result`, plus a
/// catch-all [`Meta`](Block::Meta) for source-specific kinds such as Desktop's
/// `token_budget`). It is `#[non_exhaustive]` (TD-07) because new block kinds
/// appear across Claude versions.
///
/// # Redaction
///
/// Block bodies may hold secrets or PII — assistant text, extended thinking, and
/// especially `tool_result` output can contain live shell stdout with
/// credentials (design-doc §10). [`Block`]'s [`Debug`] therefore **never prints
/// body content**: it renders the variant, the non-secret discriminants (tool
/// name, meta kind), and body *lengths*, but not the bodies themselves
/// (guideline API-18 / M-PUBLIC-DEBUG).
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Block {
    /// Rendered text (`text`).
    Text {
        /// The text body (redacted in [`Debug`]).
        text: String,
    },
    /// Extended-thinking content (`thinking`).
    Thinking {
        /// The thinking body (redacted in [`Debug`]).
        text: String,
    },
    /// A tool invocation (`tool_use`).
    ToolUse {
        /// The tool name (not secret; shown in [`Debug`]).
        name: String,
        /// The raw tool input payload (redacted in [`Debug`]).
        input: String,
    },
    /// A tool result (`tool_result`) — may carry live command output.
    ToolResult {
        /// The raw tool output (redacted in [`Debug`]).
        output: String,
    },
    /// Any other/opaque block preserved verbatim (e.g. Desktop `token_budget`).
    Meta {
        /// The source block-type discriminant, e.g. `"token_budget"` (shown).
        kind: String,
        /// The raw block payload (redacted in [`Debug`]).
        data: String,
    },
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text { text } => f.debug_struct("Text").field("len", &text.len()).finish(),
            Self::Thinking { text } => f
                .debug_struct("Thinking")
                .field("len", &text.len())
                .finish(),
            Self::ToolUse { name, input } => f
                .debug_struct("ToolUse")
                .field("name", name)
                .field("input_len", &input.len())
                .finish(),
            Self::ToolResult { output } => f
                .debug_struct("ToolResult")
                .field("output_len", &output.len())
                .finish(),
            Self::Meta { kind, data } => f
                .debug_struct("Meta")
                .field("kind", kind)
                .field("data_len", &data.len())
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_redacts() {
        let secret = "SECRET-TOKEN-abc123";

        let text = Block::Text {
            text: secret.into(),
        };
        let rendered = format!("{text:?}");
        assert!(!rendered.contains(secret), "Text body leaked: {rendered}");
        assert!(
            rendered.contains("len"),
            "expected a length field: {rendered}"
        );

        let result = Block::ToolResult {
            output: format!("$ cat .env\n{secret}"),
        };
        let rendered = format!("{result:?}");
        assert!(!rendered.contains(secret), "tool output leaked: {rendered}");

        // Non-secret discriminants stay visible for debugging.
        let call = Block::ToolUse {
            name: "Bash".into(),
            input: secret.into(),
        };
        let rendered = format!("{call:?}");
        assert!(
            rendered.contains("Bash"),
            "tool name should show: {rendered}"
        );
        assert!(!rendered.contains(secret), "tool input leaked: {rendered}");
    }
}
