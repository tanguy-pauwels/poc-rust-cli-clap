use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ModelCommand {

    #[clap(subcommand)]
    pub command: ModelSubcommand,
}

#[derive(Debug, Subcommand)]
/// The Model entity. Used to manage models based on the database's tables
/// more documentation here: https://www.google.fr
pub enum ModelSubcommand {
    /// Generate a new Model based on existing Table in the database.
    /// You can see the complete documentation of the models's command here: https://google.com
    Generate(GenerateModel),
}


#[derive(Debug, Args)]
pub struct GenerateModel {
    /// The name of the Model to generate, Optionnal if you use the option --all
    /// to generate all model based on existing DB Tables
    pub name: Option<String>,

    #[arg(short, long)]
    /// Option to generate Model based on all existing DB Table.
    /// if all=false: need to specify the [NAME] argument.
    /// Default is false.
    pub all: bool,
}
