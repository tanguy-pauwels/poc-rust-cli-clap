use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ControllerCommand {

    #[clap(subcommand)]
    pub command: ControllerSubcommand,
}

#[derive(Debug, Subcommand)]
#[clap(verbatim_doc_comment)] // Pour forcer a prendre en compte les sautes de lignes
/// The Controller entity. Used to manage controller based on the existing Models
/// more documentation here: https://www.google.fr
pub enum ControllerSubcommand {
    /// Generate a new controller based on existing model.
    /// You can see the complete documentation of the cntroller's command here: https://google.com
    Generate(GenerateController),
}


#[derive(Debug, Args)]
pub struct GenerateController {
    /// The name of the controller to generate, Optionnal if you use the option --all
    /// to generate all controller based on existing models
    pub name: Option<String>,

    #[arg(short, long)]
    /// Option to generate controller based on all existing models.
    /// if all=false: need to specify the [NAME] argument.
    /// Default is false.
    #[clap(verbatim_doc_comment)]
    pub all: bool,

    #[arg(short, long)]
    /// Controller mode to generate, web, api, crud
    /// see https://www.google.com to see more information about the <MODE> option
    pub mode: String,
}
