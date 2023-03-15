INSERT INTO chats (app_user) VALUES ($1) RETURNING chat_id, app_user, created_on;
