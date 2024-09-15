use crate::messages::MessageHandler;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(
    _options: &[ResolvedOption],
    ai_dolly_handler: &Box<dyn MessageHandler + Send + Sync>,
) -> String {
    if ai_dolly_handler.clean_up() {
        return "Successfully cleared conversation".to_string();
    }

    return "Could not clear conversation".to_string();
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("clearconversation").description("Clears AI conversation")
}