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
    /// classify according to the logistic and decision classifier
    Klebseq {
        /// path to the file
        filepath: String,
        /// predict to be made
        predictfile: String,
        /// threads for the analysis
        thread: String,
    },
    /// classify according to the Random forest
    RandomSeq {
        /// path to the file
        pathfileinput: String,
        /// predict to be made
        predictfileinput: String,
        /// threads for the analysis
        threads: String,
        /// trees
        trees: String,
        /// max depth
        depth: String,
        // samplesplit
        samplesplitinput: String,
    },
    /// classify according to the KNN classifier
    KNNClassify {
        /// path to the file
        pathfileinput: String,
        /// predict file
        predictfileinput: String,
        /// number of threads
        threads: String,
    },
}
