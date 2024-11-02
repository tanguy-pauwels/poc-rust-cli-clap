use crate::cmd::controller_cmd::*;

pub fn generate_controller(args: &GenerateController) {
    if !args.all && args.name.is_none() {
        println!("{}", "\nParameter [NAME] is required when --all isn't specified \n\
                        \nUsage: barkeel controler generate [OPTIONS] --mode <MODE> [NAME] \n\
                        \ntry: \n barkeel controller generate [NAME] --mode <web|api|crud> : to generate a controller by specifing his name \
                        \nor: \n barkeel controller generate --all --mode <web|api|crud> : to generate all the controller by existings models \n\
                        \nsee: barkeel controller generate --help to see more information"
            .to_string());
    } else {
        println!("Creating controller with parameters {:?}", args);
    }
}


