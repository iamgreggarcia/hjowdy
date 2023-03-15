SELECT id, created_on, role, content, chat_id_relation
FROM messages
WHERE chat_id_relation = $1
ORDER BY created_on ASC;
