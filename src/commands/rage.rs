use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "It's okay to be angry sometimes! Just don't make it 9 to 5 :)".to_string()
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("rage").description("Why u mad :(")
}
