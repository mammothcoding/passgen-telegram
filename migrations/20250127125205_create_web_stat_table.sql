CREATE TABLE IF NOT EXISTS web_stat (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    chat_id int8,
    bot_id  int8,
    token varchar,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create unique index idx_chat_id_bot_id on web_stat (chat_id, bot_id);
