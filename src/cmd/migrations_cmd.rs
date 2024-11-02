use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct MigrationCommand {

    #[clap(subcommand)]
    pub command: MigrationSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum MigrationSubcommand {
    /// Creates a migration folder named in the format yyyy-mm-dd-timestamp_name
    /// with up.sql and down.sql files.
    /// You can see the complete documentation of the `generate` command here:
    /// https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Generate(GenerateMigration),

    /// Executes all pending migrations, as determined by barkeel's internal schema table.
    /// You can see the complete documentation of the `run` command here:
    /// https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Run(RunMigration),

    /// Executes the down.sql file of the most recent migration to revert changes.
    /// You can see the complete documentation of the `revert` command here:
    /// https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
    Revert(RevertMigration),

    /// Executes the down.sql followed by the up.sql of the most recent migration.
    /// You can see the complete documentation of the `redo` command here:
    /// https://google.com
    /// ______________________________________________________________________________________
    #[clap(verbatim_doc_comment)]
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
