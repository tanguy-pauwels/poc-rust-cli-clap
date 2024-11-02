

use clap::{Args, Parser, Subcommand};

mod cmd;
mod ops;

//use cmd::video_cmd::*;
//use ops::video_ops::*;

//use cmd::user_cmd::*;
//use ops::user_ops::*;

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
/// Barkeel is a web framework based on Rust.
/// It's allow to generate and manage a web project in web, api and crud mode.
/// More information about Barkeel at https://www.google.com/
struct Arguments {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
enum EntityType {
    Migration(MigrationCommand),
    Controller(ControllerCommand),
    Model(ModelCommand),
    Project(ProjectCommand),
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
