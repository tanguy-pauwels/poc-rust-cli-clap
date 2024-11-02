use crate::cmd::migrations_cmd::*;

pub fn generate_migration(args: &GenerateMigration) {
    println!("Creating migration with parameters {:?}", args);
    println!("Next, you could run migration with the command `barkeel migration run`");
}

pub fn run_migration() {
    println!("Running migration");
    println!("If it was a mistake, you can run `barkeel migration revert` \
    to undo the change on the database");
}

pub fn revert_migration() {
    println!("reverting migration");
    println!("If it was a mistake, you can run `barkeel migration run` \
    to re apply the change on the database");
}

pub fn redo_migration() {
    println!("re doing last migrations");
}
