use std::env;

use dotenv::dotenv;
use regex::Regex;
use serenity::{
    async_trait,
    model::{
        gateway::{Activity, Ready},
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
        },
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("tz")
                    .description("Append your timezone to your nickname")
                    .create_option(|option| {
                        option
                            .name("timezone")
                            .description("Your current timezone (UTC offset)")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                            .add_string_choice("UTC-8", "-8")
                            .add_string_choice("UTC-7", "-7")
                            .add_string_choice("UTC-6", "-6")
                            .add_string_choice("UTC-5", "-5")
                            .add_string_choice("UTC-4", "-4")
                            .add_string_choice("UTC-3", "-3")
                            .add_string_choice("UTC-2", "-2")
                            .add_string_choice("UTC-1", "-1")
                            .add_string_choice("UTC+0", "+0")
                            .add_string_choice("UTC+1", "+1")
                            .add_string_choice("UTC+2", "+2")
                            .add_string_choice("UTC+3", "+3")
                            .add_string_choice("UTC+4", "+4")
                            .add_string_choice("UTC+5", "+5")
                            .add_string_choice("UTC+6", "+6")
                            .add_string_choice("UTC+7", "+7")
                            .add_string_choice("UTC+8", "+8")
                            .add_string_choice("UTC+9", "+9")
                            .add_string_choice("UTC+10", "+10")
                            .add_string_choice("UTC+11", "+11")
                            .add_string_choice("UTC+12", "+12")
                            .add_string_choice("UTC-12", "-12")
                            .add_string_choice("UTC-11", "-11")
                            .add_string_choice("UTC-10", "-10")
                            .add_string_choice("UTC-9", "-9")
                    })
            })
        })
            .await
            .expect("Could not create application commands");

        ctx.shard.set_activity(Some(Activity::watching("for /tz")));
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "tz" => match &command.guild_id {
                    Some(_) => {
                        let chosen_offset = command
                            .data
                            .options
                            .get(0)
                            .expect("Expected user option")
                            .value
                            .as_ref()
                            .expect("Expected user object")
                            .as_str()
                            .expect("Could not parse option as string");

                        let member = &command.member.as_ref().expect("Member could not be found");

                        let regex = Regex::new(r" \(UTC[+\-]1?[0-9]\)$").unwrap();

                        let current_nick = match member.nick.as_deref() {
                            Some(nick) => nick,
                            None => &member.user.name.as_str(),
                        };

                        let base_nick = regex.replace(&current_nick, "");

                        match member
                            .edit(&ctx.http, |member| member.nickname(format!("{} (UTC{})", base_nick, chosen_offset)))
                            .await {
                            Ok(_) => "Changed your nickname".to_string(),
                            _ => "Uh oh! I couldn't change your nickname. Make sure:\n1. I have permission to `Manage Nicknames`\n2. My role's position is __above__ yours. (I can't change admins' nicknames.)".to_string(),
                        }
                    }
                    None => "Uh oh! I can't change your nickname here.".to_string(),
                },
                _ => "Uh oh! That command is registered but not implemented.".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .content(content)
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
