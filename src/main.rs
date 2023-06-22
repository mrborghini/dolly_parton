use dotenv::dotenv;
use rand::Rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::ops::RangeInclusive;

fn random_number(startrange: usize, maxrange: usize) -> usize {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(RangeInclusive::new(startrange, maxrange));
    return num;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{} {}", msg.author.name, msg.content);

        let splitcommand: Vec<&str> = msg.content.split_whitespace().collect();

        match splitcommand[0] {
            "!dolly" => {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "I don't know I just spawned here")
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!cal" => {
                if splitcommand[1] == "yes" {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Yes!").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                if splitcommand[1] == "no" {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "NO!").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                if splitcommand[1] == "maybe" {
                    if let Err(why) = msg.channel_id.say(&ctx.http, ":thinking:").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }

            "!rage" => {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "It's okay to be angry sometimes! Just don't make it 9 to 5 :)",
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!daddy" => {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "https://youtu.be/tT2CI3UK6jA")
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }

            "!fuckyoudolly" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, ":rage:").await {
                    println!("Error sending message: {:?}", why);
                }
            }

            "!compliment" => {
                let compliments = ["You have a contagious smile!", "Your kindness is a breath of fresh air.", "You're a ray of sunshine on a cloudy day.", "Your creativity knows no bounds.", "You bring out the best in people around you.", "Your positive energy is infectious.", "You have a heart of gold.", "You're a true friend who always knows how to make someone feel special.", "Your positive attitude is inspiring.", "You have an incredible sense of style.", "Your intelligence shines in everything you do.", "You have a heart full of compassion and empathy.", "Your dedication and hard work are truly admirable.", "You have a fantastic sense of humor that always brings joy to others.", "Your perseverance in the face of challenges is remarkable.", "You have a beautiful soul that radiates kindness.", "You are a great listener and always make others feel heard.", "Your passion for life is contagious.", "You have an amazing talent for [insert talent here].", "Your optimism and positivity light up any room you enter.", "You have a remarkable ability to make people feel valued and appreciated.", "Your generosity knows no bounds.", "You have a gift for making the ordinary extraordinary.", "Your presence alone brightens up any room you walk into.", "Your smile has the power to turn someone's day around.", "Your genuine and caring nature makes you a true gem.", "Your intelligence and quick thinking impress everyone around you.", "You have an impeccable taste in [insert interest or hobby here].", "Your determination and perseverance are truly inspiring.", "You have a natural ability to make people feel comfortable and at ease.", "Your confidence is contagious and empowering.", "Your thoughtfulness and attention to detail never go unnoticed.", "You have a beautiful voice that captivates anyone who hears it.", "Your wisdom and insightful perspective are highly valued by those who know you.", "You have an incredible work ethic that sets you apart from the rest.", "Your positive outlook on life is incredibly refreshing.", "You have a unique and captivating personality that draws others in.", "Your kindness towards strangers is a testament to your compassionate nature.", "You possess a remarkable ability to find beauty in the simplest things.", "Your sense of adventure and willingness to try new things is admirable.", "You have an innate ability to bring people together and foster strong connections.", "Your empathy and understanding make you an amazing friend and confidant.", "You have a magnetic aura that attracts positivity and success.", "Your smile is contagious and brightens the day for everyone around you.", "Your determination and perseverance are truly inspiring.", "You have a genuine and kind-hearted soul that touches people's lives.", "You have an extraordinary ability to make people feel seen and heard.", "Your generosity knows no bounds, and you always go the extra mile to help others.", "You have an incredible eye for detail and a knack for perfection.", "Your infectious laughter brings joy to everyone in your presence.", "You have an impeccable sense of timing and know how to make every moment special.", "Your courage to embrace vulnerability is truly inspiring.", "You have a remarkable ability to find beauty and joy in the simplest things.", "Your commitment to personal growth is admirable and sets an example for others.", "You have an incredible intuition that guides you in making wise decisions.", "Your ability to find solutions in difficult situations is remarkable", "You have a magnetic personality that draws people towards you.", "Your authenticity and genuineness are truly refreshing.", "You have an exceptional ability to communicate and connect with others.", "Your presence alone has a calming effect on those around you.", "You have a natural talent for making people feel valued and appreciated.", "Your zest for life and thirst for knowledge are contagious.", "You have an extraordinary ability to turn setbacks into comebacks.", "Your resilience in the face of adversity is truly remarkable.", "You have a genuine curiosity that fuels your continuous learning and growth.", "Your ability to adapt and thrive in any situation is commendable."];

                if splitcommand.len() == 1 {
                    if let Err(why) = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            format!(
                                "{} {}",
                                msg.author,
                                compliments[random_number(0, compliments.len() - 1)]
                            ),
                        )
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    }
                } else {
                    if let Err(why) = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            format!(
                                "{} {}",
                                splitcommand[1],
                                compliments[random_number(0, compliments.len() - 1)]
                            ),
                        )
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }

            "!silly" => {
                let sillymessages = [
                "Look at me I'm silly :man_with_veil_tone1:", 
                "https://media.tenor.com/8lBGroihh-QAAAAd/you-rage.gif", 
                "https://media.discordapp.net/attachments/925855860557746267/1081638870610886707/caption-3-1-1.gif?width=344&height=467",
                "Why couldn't Einstein build a wall? Well he only had Ein stein"
                ];

                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        sillymessages[random_number(0, sillymessages.len() - 1)],
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }

            "!valgun" => {
                let valguns = [
                    "Classic", "Shorty", "Frenzy", "Ghost", "Sherrif", "Stinger", "Spectre",
                    "Bucky", "Judge", "Bulldog", "Guardian", "Phantom", "Vandal", "Marshal",
                    "Operator", "Ares", "Odin",
                ];

                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, valguns[random_number(0, valguns.len() - 1)])
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }

            "!gosleep" => {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "<@446305232365027328> GO TO SLEEP")
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            _ => {}
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} HAS AWAKENEND!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("WAKE UP!");
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
