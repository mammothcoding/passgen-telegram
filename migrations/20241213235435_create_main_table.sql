CREATE TABLE IF NOT EXISTS main (
    id int8 PRIMARY KEY,
    last_menu_mess_id int4,
    last_gen_mess_id int4,
    is_bot bool,
    first_name varchar,
    last_name varchar,
    username varchar,
    language_code varchar,
    updated_at timestamptz not null default now()
);

comment on column main.id is 'ChatId or UserId of requester from Telegram.';
