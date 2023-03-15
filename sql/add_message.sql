INSERT INTO messages (chat_id_relation, role, content)
VALUES ($1, $2, $3) RETURNING id, created_on, role, content, chat_id_relation;
