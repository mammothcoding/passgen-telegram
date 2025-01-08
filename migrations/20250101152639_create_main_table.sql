CREATE TABLE IF NOT EXISTS main (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    chat_id int8,
    bot_id  int8,
    user_data jsonb not null,
    app_lang varchar not null default 'en',
    pgen_rules jsonb not null default '{
      "enab_letters": false,
      "enab_u_letters": false,
      "enab_num": false,
      "enab_spec_symbs": false,
      "custom_charset": "",
      "enab_strong_usab": true,
      "pwd_len": 8,
      "pwd_quantity": 1
    }'::jsonb,
    gen_count int8 default 0,
    last_menu_mess_id int4,
    last_gen_mess_id int4,
    dialog_context varchar,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

comment on column main.gen_count is 'Total number of generated passwords.';
