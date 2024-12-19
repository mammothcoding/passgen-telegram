CREATE TABLE IF NOT EXISTS main (
    id int8 PRIMARY KEY,
    last_menu_mess_id int4,
    last_gen_mess_id int4,
    is_bot bool,
    first_name varchar,
    last_name varchar,
    username varchar,
    language_code varchar,
    pgen_rules jsonb not null default '{
      "enab_letters": false,
      "enab_u_letters": false,
      "enab_num": false,
      "enab_spec_symbs": false,
      "custom_charset": null,
      "enab_strong_usab": true,
      "pwd_len": 8
    }'::jsonb,
    gen_count int8 default 0,
    updated_at timestamptz not null default now()
);

comment on column main.id is 'ChatId or UserId of requester from Telegram.';
