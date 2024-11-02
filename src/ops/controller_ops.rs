use crate::cmd::controller_cmd::*;
use colored::Colorize;

pub fn generate_controller(args: &GenerateController) {
    if !args.all && args.name.is_none() {
        println!(
        "{}",
        format!(
            "\n{} Missing required parameter {}.\n\
            \nWhen the {} option is not specified, you must provide a controller name.\n\
            \n{}:\n\
            \t barkeel controller generate [OPTIONS] --mode <MODE> [NAME]\n\
            \nExamples:\n\
            \t {} barkeel controller generate my_controller --mode <web|api|crud>\n\
            \t   (Generates a controller with the specified name)\n\
            \n\t {} barkeel controller generate --all --mode <web|api|crud>\n\
            \t   (Generates controllers for all existing models)\n\
            \nFor more information, try:\n\
            \t barkeel controller generate {}\n",
            "Error:".red().bold(),
            "[NAME]".yellow().bold(),
            "--all".cyan().bold(),
            "Usage".blue().bold(),
            "Example:".green().bold(),
            "Example:".green().bold(),
            "--help".cyan().bold()
        ));
    } else {
        println!("Creating controller with parameters {:?}", args);
    }
}


