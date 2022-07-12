use crate::error::Error;

/// Represent an application or a subcommand.
pub trait Run {
    /// Run the application or subcommand.
    fn run(self) -> Result<(), Error>;
}
