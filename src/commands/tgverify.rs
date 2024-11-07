use serenity::{
    all::{CommandInteraction, CommandOptionType, Context, CreateCommandOption, Permissions},
    async_trait,
};

use crate::commands::send_response;

use super::base::Command;

pub struct TgVerifyCommand;

#[async_trait]
impl Command for TgVerifyCommand {
    fn name(&self) -> &str {
        "tgverify"
    }

    fn description(&self) -> &str {
        "Verify your BYOND account."
    }

    fn options(&self) -> Vec<CreateCommandOption> {
        vec![CreateCommandOption::new(
            CommandOptionType::String,
            "token",
            "The token you received from the game server.",
        )
        .required(true)]
    }

    async fn execute(&self, ctx: Context, interaction: CommandInteraction) {
        println!("TODO: Implement tgverify command");
        send_response(&ctx, &interaction, "TODO").await;
    }

    fn default_permission(&self) -> serenity::all::Permissions {
        Permissions::empty()
    }
}
