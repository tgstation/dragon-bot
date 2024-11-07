use serenity::{
    all::{CommandInteraction, Context, CreateCommandOption, Permissions},
    async_trait,
};

#[async_trait]
pub trait Command {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn options(&self) -> Vec<CreateCommandOption>;
    fn default_permission(&self) -> Permissions;
    async fn execute(&self, ctx: Context, interaction: CommandInteraction);
}
