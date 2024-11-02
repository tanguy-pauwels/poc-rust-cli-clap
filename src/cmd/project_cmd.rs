use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ProjectCommand {

    #[clap(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Debug, Subcommand)]
/// The Project entity. Used to create, build or run the Project.
/// more documentation here: https://www.google.fr
pub enum ProjectSubcommand {
    /// Generate a new project based on the mode specified.
    /// You can see the complete documentation of the project's command here: https://google.com
    Generate(GenerateProject),
    /// Build the project
    /// You can see the complete documentation of the project's command here: https://google.com
    Build(BuildProject),
    Run(RunProject),
}


#[derive(Debug, Args)]
pub struct GenerateProject {
    /// The name of the Project to generate.
    pub name: String,

    #[arg(short, long)]
    /// Project mode to generate: <MODE> <web|api|crud>
    /// see https://www.google.com to see more information about the <MODE> option
    pub mode: String,
}

#[derive(Debug, Args)]
pub struct BuildProject {
    #[arg(short, long)]
    /// Debug Mode <true|false> Default: false
    pub debug: bool,
}

#[derive(Debug, Args)]
pub struct RunProject {
    #[arg(short, long)]
    /// number of worker to run
    pub worker: i32,
}
