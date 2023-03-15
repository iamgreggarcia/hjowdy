INSERT INTO chats (app_user, created_on)
VALUES ($1, $2, 'New chat ' || currval('chat_id')) RETURNING chat_id, app_user, created_on;

