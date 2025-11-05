use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::fs::read_to_string;

struct CroppedString {
    pub content: String,
    pub cut_amount: usize,
}

fn crop_string(input_string: &str, limit: usize) -> CroppedString {
    let mut char_count = 0;

    for (i, _) in input_string.char_indices() {
        // Increment character count
        char_count += 1;

        // Stop when we reach the limit
        if char_count > limit {
            let cut_string = input_string[..i].to_string();
            return CroppedString {
                content: cut_string.clone(),
                cut_amount: input_string.chars().count() - cut_string.chars().count(),
            };
        }
    }

    // If the string is shorter than the limit, return it as is
    CroppedString {
        content: input_string.to_string(),
        cut_amount: 0,
    }
}

pub fn run(_options: &[ResolvedOption]) -> String {
    let mut system_message = read_to_string("system_message.txt").unwrap_or("".to_string());

    if !system_message.is_empty() {
        let cut_system_message = crop_string(&system_message, 1500);

        return format!(
            "{} **and {} more chars**",
            cut_system_message.content, cut_system_message.cut_amount
        );
    }

    system_message = read_to_string("system_message_example.txt").unwrap();
    system_message
}

pub fn register() -> CreateCommand {
    CreateCommand::new("system_prompt").description("Shows the current system prompt")
}
