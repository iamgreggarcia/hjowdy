-- edit_chat_name.sql
UPDATE public.chats
SET chat_name = $1
WHERE chat_id = $2;
