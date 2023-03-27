INSERT INTO images (chat_id, url, created_on)
VALUES ($1, $2, $3)
RETURNING *;

