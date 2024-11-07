use std::process::exit;

use serenity::all::{CommandInteraction, EditInteractionResponse};
use serenity::{
    all::{
        Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
        EventHandler, GuildId, Interaction,
    },
    async_trait,
    model::gateway::Ready,
};

use crate::get_guild_id;

pub mod base;

pub mod tgverify;

pub const COMMANDS: [&(dyn base::Command + Sync); 1] = [&tgverify::TgVerifyCommand];

pub struct CommandHandler;

#[async_trait]
impl EventHandler for CommandHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let command = interaction.command();
        if command.is_none() {
            return;
        }
        let command_interaction = command.unwrap();
        let name = command_interaction.data.name.clone();
        println!("Received command: {}", name);

        let command = COMMANDS.iter().find(|command| command.name() == name);
        if command.is_none() {
            return;
        }

        if let Err(why) = command_interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Defer(CreateInteractionResponseMessage::default()),
            )
            .await
        {
            println!("Failed to acknowledge command: {:?}", why);
            return;
        }

        let command = command.unwrap();
        command.execute(ctx, command_interaction).await;
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let id = get_guild_id();
        if id == 0 {
            println!("GUILD_ID is invalid.");
            exit(1);
        }

        let guild = GuildId::new(id);
        let mut commands = vec![];
        for command in COMMANDS.iter() {
            commands.push(CommandHandler::create_command(*command));
        }

        if let Err(why) = guild.set_commands(&ctx.http, commands).await {
            println!("Failed to set application commands: {:?}", why);
            exit(1);
        }
        println!("Commands set.");
    }
}

impl CommandHandler {
    fn create_command(command: &dyn base::Command) -> CreateCommand {
        let mut create_command = CreateCommand::new(command.name())
            .description(command.description())
            .dm_permission(false);
        for option in command.options() {
            create_command = create_command.add_option(option);
        }
        create_command.default_member_permissions(command.default_permission())
    }
}

pub async fn send_response(ctx: &Context, interaction: &CommandInteraction, content: &str) {
    println!("Sending response: {}", content);
    if let Err(why) = interaction
        .edit_response(&ctx.http, EditInteractionResponse::new().content(content))
        .await
    {
        println!("Failed to send response: {:?}", why);
    }
}
