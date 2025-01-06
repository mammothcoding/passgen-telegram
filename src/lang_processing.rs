pub mod lang_processing {
    use crate::db_processing::db_processing::get_user_app_lang;
    use std::collections::HashMap;

    pub const EN: [(&str, &str); 10] = [
        ("menu_lcase", "include lowercase letters"),
        ("menu_cap", "include capital letters"),
        ("menu_num", "include numbers"),
        ("menu_ss", "include special symbols"),
        ("menu_conven", "strong & usability password"),
        ("menu_cch", "custom charset. Press to set."),
        ("menu_pass_len1", "password length"),
        ("menu_pass_len2", ". Press to edit."),
        ("menu_btn_gen", "🎲 GENERATE"),
        ("help", "The first position in the password is a capital or small letter.\nThe last position is the symbol.\nExcluded ambiguous characters \"0oOiIlL1\"."),
    ];

    pub const RU: [(&str, &str); 10] = [
        ("menu_lcase", "включая маленькие буквы"),
        ("menu_cap", "включая заглавные буквы"),
        ("menu_num", "включая цифры"),
        ("menu_ss", "включая спец. символы"),
        ("menu_conven", "сильный и удобный пароль"),
        ("menu_cch", "установить свой набор симв."),
        ("menu_pass_len1", "длина пароля"),
        ("menu_pass_len2", ". Установить."),
        ("menu_btn_gen", "🎲 СГЕНЕРИРОВАТЬ"),
        ("help", "Первая позиция в пароле - большая или малая буква.\nПоследняя позиция - символ.\nИсключены двояко читаемые символы \"0oOiIlL1\"."),
    ];

    pub async fn get_lang_map(chat_id: i64) -> HashMap<&'static str, &'static str> {
        let user_lang: String = match get_user_app_lang(chat_id).await {
            Some(lang_id) => lang_id,
            _ => "en".to_string(),
        };

        let lang_name: String = user_lang.clone().to_owned();
        let lang_name_slice: &str = &lang_name[..];

        match lang_name_slice {
            "en" => HashMap::from_iter(EN),
            "ru" => HashMap::from_iter(RU),
            /*"es" => SP,
            "pt" => PT,
            "fr" => FR,
            "de" => DE,*/
            _ => HashMap::from_iter(EN),
        }
    }
}
