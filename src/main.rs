

use clap::{Args, Parser, Subcommand};

mod cmd;
mod ops;

use cmd::migrations_cmd::*;
use ops::migration_ops::*;

use cmd::controller_cmd::*;
use ops::controller_ops::*;

use cmd::model_cmd::*;
use ops::model_ops::*;

use cmd::project_cmd::*;
use ops::project_ops::*;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(verbatim_doc_comment)]
/// ______________________________________________________________________________________
///Barkeel is a high-performance, security-focused web framework developed in Rust.
/// It enables the creation of websites, web applications, and APIs with robust CLI tools for project management.
/// .
/// You can run `barkeel <command> --help` to get further information about a command.
/// .
/// More information about Barkeel at https://www.google.com/
/// ______________________________________________________________________________________
struct Arguments {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
enum EntityType {
    /// The Project entity provides commands for creating, building, and running a Barkeel project.
    /// It includes setup options for different modes (web, api, or crud).
    /// This entity is the starting point for initializing and managing the project's core structure.
    /// - See `barkeel project --help` or https://www.google.fr
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Project(ProjectCommand),

    /// The Migrations entity manages database migrations.
    /// With commands to generate, run, revert, and redo migrations.
    /// - See `barkeel migration --help` or https://www.google.fr
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Migration(MigrationCommand),

    /// The Model entity enables the creation of models representing database tables.
    /// It simplifies database interactions by generating necessary structures for CRUD operations.
    /// - See `barkeel model --help` or https://www.google.fr
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Model(ModelCommand),

    /// The Controller entity provides commands to generate controllers.
    /// These handle application logic for specific routes and endpoints.
    /// With generated controllers, you can quickly set up basic CRUD functionality.
    /// - See `barkeel controller --help` or https://www.google.fr
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Controller(ControllerCommand),
}

#[derive(Debug, Args)]
struct DeleteEntity {
    /// The entity's id to update
    pub id: i32,
}


fn main() {
    let args = Arguments::parse();
    match &args.entity_type {
        EntityType::Migration(migration) => {
            match &migration.command{
                MigrationSubcommand::Generate(args) => {generate_migration(args);}
                MigrationSubcommand::Run(..) => {run_migration();}
                MigrationSubcommand::Revert(..) => {revert_migration();}
                MigrationSubcommand::Redo(..) => {redo_migration();}
            }
        }
        EntityType::Controller(controller) => {
            match &controller.command{
                ControllerSubcommand::Generate(args) => {generate_controller(args);}
            }
        }
        EntityType::Model(model) => {
            match &model.command{
                ModelSubcommand::Generate(args) => {generate_model(args);}
            }
        }
        EntityType::Project(project) => {
            match &project.command{
                ProjectSubcommand::Generate(args) => {generate_project(args);}
                ProjectSubcommand::Build(args) => {build_project(args);}
                ProjectSubcommand::Run(args) => {run_project(args);}
            }
        }
    }
}
