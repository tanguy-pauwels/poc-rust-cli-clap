use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ProjectCommand {
    #[clap(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubcommand {
    /// Initializes a new project and generates basic boilerplate code.
    /// See `barkeel project generate --help` or https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Generate(GenerateProject),

    /// Compiles the project.
    /// See `barkeel project build --help` or https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Build(BuildProject),

    /// Runs the project.
    /// See `barkeel project run --help` or https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Run(RunProject),
}


#[derive(Debug, Args)]
pub struct GenerateProject {
    /// (required): Name of the project to create.
    pub name: String,

    /// (optional): Defines the project type.
    /// Options: web, api, or crud.
    /// see https://www.google.com to see more information about the <MODE> option
    /// ______________________________________________________________________________________
    #[arg(short, long)]
    #[clap(verbatim_doc_comment)]
    pub mode: String,
}

#[derive(Debug, Args)]
pub struct BuildProject {
    #[arg(short, long)]
    /// (optional): Compile in release mode (default: false).
    pub release: bool,
}

#[derive(Debug, Args)]
pub struct RunProject {
    #[arg(short, long)]
    /// (optional): Specifies the number of workers to use.
    pub worker: i32,

    #[arg(short, long)]
    /// (optional): Enables debug mode (default: false).
    pub debug: bool,
}
