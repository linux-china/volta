pub(crate) mod completions;
pub(crate) mod fetch;
pub(crate) mod install;
pub(crate) mod list;
pub(crate) mod man_pages;
pub(crate) mod pin;
pub(crate) mod run;
pub(crate) mod setup;
pub(crate) mod uninstall;
pub(crate) mod r#use;
pub(crate) mod which;

pub(crate) use self::which::Which;
pub(crate) use completions::Completions;
pub(crate) use fetch::Fetch;
pub(crate) use install::Install;
pub(crate) use list::List;
pub(crate) use man_pages::ManPages;
pub(crate) use pin::Pin;
pub(crate) use r#use::Use;
pub(crate) use run::Run;
pub(crate) use setup::Setup;
pub(crate) use uninstall::Uninstall;

use volta_core::error::ExitCode;
use volta_core::session::Session;

/// A Volta command.
pub(crate) trait Command: Sized {
    /// Executes the command. Returns `Ok(code)` if the process should return `code`,
    /// `Err(Error::Volta(e))` if the process should report `e` and return `e.exit_code()`,
    /// and `Err(Error::Tool(code))` if the process should return the exit code of a tool.
    fn run(self, session: &mut Session) -> Result<ExitCode, crate::common::Error>;
}
