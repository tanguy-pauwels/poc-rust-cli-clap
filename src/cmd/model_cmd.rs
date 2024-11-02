use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ModelCommand {

    #[clap(subcommand)]
    pub command: ModelSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ModelSubcommand {
    /// Generate a new Model based on existing Table in the database.
    /// You can see the complete documentation of the `generate` command here:
    /// https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Generate(GenerateModel),
}


#[derive(Debug, Args)]
pub struct GenerateModel {
    /// Specifies the name of the model to generate.
    /// Optional if the `--all` option is used to generate models for all existing database tables.
    /// If `--all` is false, the [name] argument is required.
    #[clap(verbatim_doc_comment)]
    pub name: Option<String>,

    /// Default: false.
    /// Generates models for all existing database tables.
    /// Set `--all` to true to create models for each table in the current database schema.
    /// If set to false, you must specify a [name] for the model.
    #[arg(short, long)]
    #[clap(verbatim_doc_comment)]
    pub all: bool,
}
