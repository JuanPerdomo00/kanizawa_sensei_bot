use teloxide_core::payloads::SendMessageSetters;
use teloxide_core::prelude::{Requester, UserId};
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::Message;
use crate::error::{PermissionsDenied, handle_status};
use crate::prelude::Bot;
use crate::utils::{MessageExt, Timer};

pub async fn unbanning(bot: Bot, msg: Message) -> ResponseResult<()> {
    let user_status = handle_status(&bot, &msg).await;
    if !user_status {
        bot.send_message(msg.chat.id, PermissionsDenied)
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);
        return Ok(())
    }

    // Necessary for unban by id (/unban 12345678)
    let Some(replied) = msg.reply_to_message() else {
        let parsed_id = msg.parse_id().await;
        if parsed_id == 404 {
            bot.send_message(msg.chat.id, "El usuario proporcionado no existe o no es válido")
                .reply_to_message_id(msg.id).await?
                .delete_message_timer(bot, msg.chat.id, msg.id, 10);
            return Ok(())
        }
        bot.unban_chat_member(msg.chat.id, UserId(parsed_id)).await?;
        bot.send_message(msg.chat.id, "✅ Usuario desbaneado")
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);
        return Ok(())
    };

    // Unban for reply to a Join Event (/unban <user> Joined the group)
    let Some(user) = replied.from() else {
        let Some(user) = msg.extract_first_new_member(&msg) else {
            return Ok(())
        };

        bot.unban_chat_member(msg.chat.id, user.id).await?;
        bot.send_message(msg.chat.id, "✅ Usuario desbaneado")
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);

        return Ok(())
    };

    // Unban for reply to a message (/unban <replying message>)
    let user_id = user.id;
    let username = user.username
        .as_ref()
        .map_or_else(String::new, std::string::ToString::to_string);

    bot.unban_chat_member(msg.chat.id, user_id).await?;
    bot.send_message(msg.chat.id, format!("✅ @{username} [<code>{user_id}</code>] ha sido desbaneado"))
        .reply_to_message_id(msg.id).await?.delete_message_timer(bot, msg.chat.id, msg.id, 60);

    Ok(())
}