use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct MigrationCommand {

    #[clap(subcommand)]
    pub command: MigrationSubcommand,
}

#[derive(Debug, Subcommand)]
/// The Migration entity. Used to manage migrations using Diesel
/// more documentation here: https://www.google.fr
pub enum MigrationSubcommand {
    /// generate a migration folder named based on the name passed in argument
    /// with up.sql and down.sql inside.
    Generate(GenerateMigration),
    /// Runs all pending migrations, as determined by barkeel's internal schema table.
    Run(RunMigration),
    /// Runs the down.sql for the most recent migration.
    Revert(RevertMigration),
    /// Runs the down.sql and then the up.sql for the most recent migration.
    Redo(RedoMigration),
}

#[derive(Debug, Args)]
pub struct GenerateMigration {
    /// The name of the migration. Exemple:
    /// `barkeel migration generate foo` will generate a migration folder
    /// named yyyy-mm-dd-timestamp_<name> with up.sql and down.sql inside.
    pub name: String,
}

#[derive(Debug, Args)]
pub struct RunMigration {}

#[derive(Debug, Args)]
pub struct RevertMigration {}

#[derive(Debug, Args)]
pub struct RedoMigration {}
