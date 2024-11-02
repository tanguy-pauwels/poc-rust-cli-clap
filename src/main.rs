mod args;
mod cmd;
mod ops;

use clap::Parser;
use args::*;

use cmd::migrations_cmd::*;
use ops::migration_ops::*;

use cmd::controller_cmd::*;
use ops::controller_ops::*;

use cmd::model_cmd::*;
use ops::model_ops::*;

use cmd::project_cmd::*;
use ops::project_ops::*;


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
