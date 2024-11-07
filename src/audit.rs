use serenity::{
    all::{Context, EventHandler, GuildId, GuildMemberUpdateEvent, Member, User},
    async_trait,
};

pub struct AuditHandler;

#[async_trait]
#[allow(unused_variables)]
impl EventHandler for AuditHandler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {}

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data_if_available: Option<Member>,
    ) {
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        old_if_available: Option<Member>,
        new: Option<Member>,
        event: GuildMemberUpdateEvent,
    ) {
    }
}
