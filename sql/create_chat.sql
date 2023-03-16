INSERT INTO chats (app_user, created_on)
VALUES ($1, $2) RETURNING chat_id, app_user, created_on, chat_name;

