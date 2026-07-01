//! Sparse, source-agnostic provenance for a normalized message.

use std::path::PathBuf;

use crate::ids::{AccountId, MachineId, ModelId, SessionId};

/// Where a message came from, as a **sparse union** over both sources.
///
/// No single source fills every dimension: Code supplies `model` / `project_cwd`
/// / config-root; Desktop supplies `account`; neither supplies all (corpus-map
/// §3a). Every source-specific dimension is therefore an [`Option`]. The one
/// dimension present in *both* sources — the session-or-conversation the message
/// belongs to — is mandatory and anchors the type.
///
/// Build with [`new`](Provenance::new) plus the `with_*` methods, so the
/// mandatory anchor is always supplied and new dimensions can be added without
/// breaking call sites:
///
/// ```
/// use ixy_core::{Provenance, SessionId, AccountId};
///
/// let p = Provenance::new(SessionId::new("conv-1")?)
///     .with_account(AccountId::new("acct-9")?);
/// assert!(p.model().is_none());
/// # Ok::<(), ixy_core::IdError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    session_or_conversation: SessionId,
    account: Option<AccountId>,
    machine: Option<MachineId>,
    config_root: Option<PathBuf>,
    model: Option<ModelId>,
    project_cwd: Option<PathBuf>,
}

impl Provenance {
    /// Creates provenance anchored to the session/conversation the message
    /// belongs to. All source-specific dimensions start as [`None`].
    #[must_use]
    pub fn new(session_or_conversation: SessionId) -> Self {
        Self {
            session_or_conversation,
            account: None,
            machine: None,
            config_root: None,
            model: None,
            project_cwd: None,
        }
    }

    /// Sets the account this message belongs to (Desktop).
    #[must_use]
    pub fn with_account(mut self, account: AccountId) -> Self {
        self.account = Some(account);
        self
    }

    /// Sets the originating machine/host.
    #[must_use]
    pub fn with_machine(mut self, machine: MachineId) -> Self {
        self.machine = Some(machine);
        self
    }

    /// Sets the config root the record was found under (e.g. `~/.claude`).
    #[must_use]
    pub fn with_config_root(mut self, config_root: PathBuf) -> Self {
        self.config_root = Some(config_root);
        self
    }

    /// Sets the model that produced the message (Code assistant turns).
    #[must_use]
    pub fn with_model(mut self, model: ModelId) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the working directory the session ran in (Code `cwd`).
    #[must_use]
    pub fn with_project_cwd(mut self, project_cwd: PathBuf) -> Self {
        self.project_cwd = Some(project_cwd);
        self
    }

    /// The session or conversation the message belongs to (always present).
    #[must_use]
    pub fn session_or_conversation(&self) -> &SessionId {
        &self.session_or_conversation
    }

    /// The account, if known.
    #[must_use]
    pub fn account(&self) -> Option<&AccountId> {
        self.account.as_ref()
    }

    /// The originating machine, if known.
    #[must_use]
    pub fn machine(&self) -> Option<&MachineId> {
        self.machine.as_ref()
    }

    /// The config root, if known.
    #[must_use]
    pub fn config_root(&self) -> Option<&PathBuf> {
        self.config_root.as_ref()
    }

    /// The model, if known.
    #[must_use]
    pub fn model(&self) -> Option<&ModelId> {
        self.model.as_ref()
    }

    /// The project working directory, if known.
    #[must_use]
    pub fn project_cwd(&self) -> Option<&PathBuf> {
        self.project_cwd.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_sparse() {
        let session = SessionId::new("sess-or-conv").unwrap();

        // Code-style: model + project_cwd present; account None.
        let code = Provenance::new(session.clone())
            .with_model(ModelId::new("claude-opus-4-8").unwrap())
            .with_project_cwd(PathBuf::from("/Users/x/lab/ixy"));
        assert!(code.model().is_some());
        assert!(code.project_cwd().is_some());
        assert!(code.account().is_none(), "Code has no account");

        // Desktop-style: account present; model + cwd None.
        let desktop =
            Provenance::new(session.clone()).with_account(AccountId::new("acct-2df6").unwrap());
        assert!(desktop.account().is_some());
        assert!(desktop.model().is_none(), "Desktop has no model");
        assert!(desktop.project_cwd().is_none(), "Desktop has no cwd");

        // The anchor is mandatory and shared by both shapes.
        assert_eq!(code.session_or_conversation(), &session);
        assert_eq!(desktop.session_or_conversation(), &session);
    }
}
