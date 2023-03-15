SELECT role, content FROM messages WHERE chat_id_relation = ?1 
ORDER BY created_on DESC; 
