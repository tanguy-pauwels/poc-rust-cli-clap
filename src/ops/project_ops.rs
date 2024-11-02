use crate::cmd::project_cmd::*;

pub fn generate_project(args: &GenerateProject) {
    println!("Generating {} project", args.name);
    println!("Next you could run `barkeel project print-schema`to init schema.rs")
}

pub fn build_project(args: &BuildProject) {
    println!("Building project {:?}", args);
    println!("Next you could run `barkeel project run`")

}

pub fn run_project(args: &RunProject) {
    println!("Building project {:?}", args);
}