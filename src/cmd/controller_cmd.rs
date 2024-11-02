use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ControllerCommand {

    #[clap(subcommand)]
    pub command: ControllerSubcommand,
}

#[derive(Debug, Subcommand)]

pub enum ControllerSubcommand {
    /// Generate a new Controller based on existing Model in the project.
    /// You can see the complete documentation of the `generate` command here:
    /// https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Generate(GenerateController),
}


#[derive(Debug, Args)]
pub struct GenerateController {
    /// Specifies the name of the controller to generate.
    /// Optional if the `--all` option is used to generate controller for all existing Models.
    /// If `--all` is false, the [name] argument is required.
    #[clap(verbatim_doc_comment)]
    pub name: Option<String>,

    /// Default: false.
    /// Generates controller for all existing Models.
    /// Set `--all` to create controller for each Model in the project.
    /// If set to false, you must specify a [name] for the model.
    #[arg(short, long)]
    #[clap(verbatim_doc_comment)]
    pub all: bool,

    /// Specifies the controller generation mode: `web`, `api`, or `crud`.
    /// Choose the mode that best matches the type of functionality you need for the controller:
    /// - `web`: for web controllers handling HTML requests and responses
    /// - `api`: for API controllers designed to manage JSON or XML data
    /// - `crud`: for CRUD operations on resources within the application
    ///
    /// For more information on each mode, refer to the documentation at:
    /// https://www.google.com
    #[arg(short, long)]
    #[clap(verbatim_doc_comment)]
    pub mode: String,
}
