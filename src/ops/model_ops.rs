use crate::cmd::model_cmd::*;

pub fn generate_model(args: &GenerateModel) {
    if !args.all && args.name.is_none() {
        println!("{}", "\nParameter [NAME] is required when --all isn't specified \n\
                        \nUsage: barkeel model generate [OPTIONS] [NAME] \n\
                        \ntry: \n barkeel model generate [NAME] : to generate a model by specifing his name \
                        \nor: \n barkeel model generate --all : to generate all the model by existings database's tables \n\
                        \nsee: barkeel model generate --help to see more information"
            .to_string());
    } else {
        println!("Creating model with parameters {:?}", args);
    }
}