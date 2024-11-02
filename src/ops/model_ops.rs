use colored::Colorize;
use crate::cmd::model_cmd::*;

pub fn generate_model(args: &GenerateModel) {
    if !args.all && args.name.is_none() {
        println!(
            "{}",
            format!(
                "\n{} Missing required parameter {}.\n\
            \nWhen the {} option is not specified, you must provide a model name.\n\
            \n{}:\n\
            \t barkeel model generate [OPTIONS] [NAME]\n\
            \nExamples:\n\
            \t {} barkeel model generate my_model \n\
            \t   (Generates a model with the specified name)\n\
            \n\t {} barkeel model generate --all \n\
            \t   (Generates models for all existing models)\n\
            \nFor more information, try:\n\
            \t  barkeel model generate {}\n",
                "Error:".red().bold(),
                "[NAME]".yellow().bold(),
                "--all".cyan().bold(),
                "Usage".blue().bold(),
                "Example:".green().bold(),
                "Example:".green().bold(),
                "--help".cyan().bold()
            ));
    } else {
        println!("Creating model with parameters {:?}", args);
    }
}

//pub fn generate_model(args: &GenerateModel) {
//    println!("Creating model with parameters {:?}", args);
//}