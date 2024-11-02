use clap::{Parser, Subcommand};
use crate::cmd::migrations_cmd::*;
use crate::cmd::controller_cmd::*;
use crate::cmd::project_cmd::*;
use crate::cmd::model_cmd::*;


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
pub struct Arguments {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
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