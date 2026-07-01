//! A pointer from a normalized message back to its bedrock record.

use crate::ids::Locator;
use crate::taxonomy::Source;

/// A reference back to the original source record a message was normalized from.
///
/// `ixy` treats the raw corpus as canonical bedrock and every normalized record
/// as derived; a [`RawRef`] is the pointer that keeps normalization **non-lossy**
/// — from any [`NormalizedMessage`] you can always find the exact bytes it came
/// from. It is mandatory on every message (there is no way to build one without
/// it).
///
/// [`NormalizedMessage`]: crate::NormalizedMessage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawRef {
    source: Source,
    locator: Locator,
}

impl RawRef {
    /// Creates a reference to a bedrock record in `source` at `locator`.
    #[must_use]
    pub fn new(source: Source, locator: Locator) -> Self {
        Self { source, locator }
    }

    /// The store the bedrock record lives in.
    #[must_use]
    pub fn source(&self) -> Source {
        self.source
    }

    /// The opaque pointer to the record within its source.
    #[must_use]
    pub fn locator(&self) -> &Locator {
        &self.locator
    }
}
