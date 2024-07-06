use anyhow::Result;
use poise::serenity_prelude as serenity;

use crate::Context;

/// Toggle the verification requirement for a member
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    required_bot_permissions = "MANAGE_ROLES",
    name_localized("ja", "認証スキップ"),
    description_localized("ja", "メンバーの認証の要求を切り替える")
)]
pub async fn bypass_verify(
    ctx: Context<'_>,

    #[name_localized("ja", "メンバー")]
    #[description = "The member to toggle the requirement for"]
    #[description_localized("ja", "認証の要求を切り替えるメンバー")]
    member: serenity::User,
) -> Result<()> {
    if let Some(guild_id) = ctx.guild_id() {
        ctx.defer_ephemeral().await?;

        let guild_member = guild_id.member(&ctx, member.id).await?;

        if (guild_member.flags & serenity::GuildMemberFlags::BYPASSES_VERIFICATION)
            == serenity::GuildMemberFlags::empty()
        {
            let disable_verification = serenity::builder::EditMember::new()
                .flags(guild_member.flags | serenity::GuildMemberFlags::BYPASSES_VERIFICATION);

            guild_id
                .edit_member(&ctx, member.id, disable_verification)
                .await?;

            if ctx.locale().unwrap_or("en").starts_with("ja") {
                ctx.say(
                    "メンバーの認証を無効化しました。\n\
                    認証を有効化するにはコマンドをもう一度使用してください。",
                )
                .await?;
            } else {
                ctx.say(
                    "The verification requirement for the member has been disabled.\n\
                    Use the command again to enable the requirement.",
                )
                .await?;
            }
        } else {
            let enable_verification = serenity::builder::EditMember::new()
                .flags(guild_member.flags & !serenity::GuildMemberFlags::BYPASSES_VERIFICATION);

            guild_id
                .edit_member(&ctx, member.id, enable_verification)
                .await?;

            if ctx.locale().unwrap_or("en").starts_with("ja") {
                ctx.say(
                    "メンバーの認証を有効化しました。\n\
                    認証を無効化するにはコマンドをもう一度使用してください。",
                )
                .await?;
            } else {
                ctx.say(
                    "The verification requirement for the member has been enabled.\n\
                    Use the command again to disable the requirement.",
                )
                .await?;
            }
        }
    };

    Ok(())
}
