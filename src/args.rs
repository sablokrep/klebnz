use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "klebnz",
    version = "1.0",
    about = "Machine learning classifier for Kleb
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// estimation sheet
    Klebseq {
        /// path to the file
        filepath: String,
        /// predict to be made
        predictfile: String,
        /// threads for the analysis
        thread: String,
    },
}
