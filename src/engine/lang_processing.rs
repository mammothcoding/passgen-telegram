pub mod lang_processing {
    use crate::engine::db_processing::db_processing::get_user_app_lang;
    use std::collections::HashMap;

    pub const EN: [(&str, &str); 22] = [
        ("menu_lcase", "include lowercase letters"),
        ("menu_cap", "include capital letters"),
        ("menu_num", "include numbers"),
        ("menu_ss", "include special symbols"),
        ("menu_conven", "strong & usability password"),
        ("menu_cch", "custom charset. Press to set."),
        ("menu_pass_len1", "password length"),
        ("menu_pass_len2", ". Press to edit."),
        ("menu_pass_qua", "passwords quantity"),
        ("menu_btn_gen", "🎲 GENERATE"),
        ("menu_btn_stat", "📊 statistics"),
        ("menu_stat_btn_regen", "re-generate"),
        ("menu_stat_btn_close", "close"),
        ("dialog_large_cch", "<i>⚠️ A very large custom charset size has been passed. Please enter your character set below again 🔡🔢🔣</i>"),
        ("dialog_wrng_plen", "<i>🚫 Wrong number! Please enter the number again 🔢</i>"),
        ("dialog_unk_cmd", "<i>🚫 Unknown command!</i>"),
        ("dialog_ent_cch", "<i>Please enter your character set below to generate a password 🔡🔢🔣</i>"),
        ("dialog_ent_plen", "<i>Please enter your password length below 🔢</i>"),
        ("dialog_ent_pqua", "<i>Please enter the number of passwords to be generated 🔢</i>"),
        ("dialog_pwd_is", "Password is (click to copy):"),
        ("dialog_max_mess_len", "relax, why do you need so many long passwords?😀
Try reducing the number of passwords or their length.
The allowed message length for Telegram has been exceeded!"),
        ("help", "<b>🔏 Mammothcoding password generator for Telegram.</b>
    A telegram bot-service for generating cryptographically secure passwords/tokens and other sets and sequences.
<i><a href=\"https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs\">CSPRNGs</a> Isaac64Rng and Hc128Rng are used.</i>
<i>🦀 Made with Rust.</i>
<i>🔗 <u><a href=\"https://github.com/mammothcoding/passgen-telegram\">Project github page</a></u> (temporary private)</i>

    <u><b>Usage:</b></u>
🔹 You can choose interface language.
🔹 You can create a regular password, choosing in the rules the presence of <i>small and capital letters, numbers, special characters</i>.
🔹 You can create a <i>strong and usability password</i>:
including the whole standard set, but the first position in the password is a capital or small letter, the last position is the symbol.
Excluded ambiguous characters <i>\"0oOiIlL1\"</i>.
🔸 If this rule is enabled, the other consistency rules of the generating are not taken, except for a rule <i>\"custom charset\"</i>.
🔹 You can create a set from your <i>\"custom charset\"</i> that includes any unicode characters like \"abcABC123⭕➖❎⚫⬛n₼⁂🙂\".
🔸 This set of characters will exclude all other rules except for a rule <i>\"strong & usability password\"</i>.
⚙️ If <i>\"strong & usability password\"</i> on too then you can generate combined <i>strong and usability</i> result with <i>custom charset</i>.
🔹 You can specify the required <i>\"password length\"</i> of not less than 4 and not more than 3900 characters.
🔹 You can specify the required <i>\"passwords quantity\"</i> - not more than 100.
🔹 For security purposes, you can delete the message with the last password you created using the
<b>[🧹 pwd]</b> button that appears after you generate the password.
                                           ☑️/start"),
    ];

    pub const ES: [(&str, &str); 22] = [
        ("menu_lcase", "incluir letras minúsculas"),
        ("menu_cap", "incluir letras mayúsculas"),
        ("menu_num", "incluir números"),
        ("menu_ss", "incluir símbolos especiales"),
        ("menu_conven", "contraseña segura y fácil de usar"),
        ("menu_cch", "personalizado. Pulse para configurarlo."),
        ("menu_pass_len1", "longitud de la contraseña"),
        ("menu_pass_len2", ". Pulse para editar."),
        ("menu_pass_qua", "Cantidad de contraseñas"),
        ("menu_btn_gen", "🎲 GENERAR"),
        ("menu_btn_stat", "📊 estadísticas"),
        ("menu_stat_btn_regen", "regenerado"),
        ("menu_stat_btn_close", "cerca"),
        ("dialog_large_cch", "<i>⚠️ Se ha pasado un tamaño de juego de caracteres personalizado muy grande. Vuelva a introducir su juego de caracteres 🔡🔢🔣</i>"),
        ("dialog_wrng_plen", "<i>🚫 Se ha equivocado de número. Vuelva a introducir el número 🔢</i>"),
        ("dialog_unk_cmd", "<i>🚫 ¡Comando desconocido!</i>"),
        ("dialog_ent_cch", "<i>Introduzca a continuación su juego de caracteres para generar una contraseña 🔡🔢🔣</i>"),
        ("dialog_ent_plen", "<i>Introduzca a continuación la longitud de su contraseña 🔢</i>"),
        ("dialog_ent_pqua", "<i>Introduzca el número de contraseñas que desea generar 🔢</i>"),
        ("dialog_pwd_is", "La contraseña es (haga clic para copiar):"),
        ("dialog_max_mess_len", "relájate, ¿por qué necesitas tantas contraseñas largas? 😀 .
Intenta reducir el número de contraseñas o su longitud.
¡Longitud de mensaje excedida para Telegram!"),
        ("help", "<b>🔏 Mammothcoding generador de contraseñas para Telegram.</b>
    Un servicio bot de telegramas para generar contraseñas/tokens criptográficamente seguros y otros conjuntos y secuencias.
<i><a href=\"https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs\">CSPRNGs</a> Se utilizan Isaac64Rng y Hc128Rng.</i>
<i>🦀 Fabricado con Rust.</i>
<i>🔗 <u><a href=\"https://github.com/mammothcoding/passgen-telegram\">Página del proyecto en github</a></u> (privado temporal)</i>

    <u><b>Utilización:</b></u>
🔹 Puedes elegir el idioma de la interfaz.
🔹 Puedes crear una contraseña normal, eligiendo en las reglas la presencia de <i>letras minúsculas y mayúsculas, números, caracteres especiales</i>.
🔹 Puede crear una <i>contraseña segura y fácil de usar</i>:
incluyendo todo el conjunto estándar, pero la primera posición en la contraseña es una letra mayúscula o minúscula, la última posición es el símbolo.
Se excluyen los caracteres ambiguos <i>\"0oOiIlL1\"</i>.
🔸 Si esta regla está activada, las otras reglas de consistencia de la generación no se toman, a excepción de una regla <i>\"charset personalizado\"</i>.
🔹 Puedes crear un conjunto a partir de tu <i>\"charset personalizado\"</i> que incluya cualquier carácter unicode como \"abcABC123⭕➖❎⚫⬛n₼⁂🙂\".
🔸 Este conjunto de caracteres excluirá todas las demás reglas excepto una regla <i>\"contraseña segura y fácil de usar\"</i>.
⚙️ Si la contraseña de fortaleza y usabilidad también está activada, puede generar un resultado combinado de fortaleza y usabilidad con un conjunto de <i>\"charset personalizado\"</i>.
🔹 Puede especificar la longitud de contraseña requerida de no menos de 4 y no más de 3900 caracteres.
🔹 Puede especificar el <i>\"Cantidad de contraseñas\"</i> requerido: no más de 100.
🔹 Por motivos de seguridad, puedes eliminar el mensaje con la última contraseña que creaste utilizando el botón
<b>[🧹 pwd]</b> que aparece después de generar la contraseña.
                                           ☑️/start"),
    ];

    pub const PT: [(&str, &str); 22] = [
        ("menu_lcase", "incluir letras minúsculas"),
        ("menu_cap", "incluir letras maiúsculas"),
        ("menu_num", "incluir números"),
        ("menu_ss", "incluir símbolos especiais"),
        ("menu_conven", "palavra-passe forte e de fácil utilização"),
        ("menu_cch", "conjunto de caracteres personalizado. Prima para definir."),
        ("menu_pass_len1", "comprimento da palavra-passe"),
        ("menu_pass_len2", ". Prima para editar."),
        ("menu_pass_qua", "quantidade de palavras-passe"),
        ("menu_btn_gen", "🎲 GERAR"),
        ("menu_btn_stat", "📊 estatísticas"),
        ("menu_stat_btn_regen", "regenerado"),
        ("menu_stat_btn_close", "fechar"),
        ("dialog_large_cch", "<i>⚠️ Foi passado um tamanho de conjunto de caracteres personalizado muito grande. Por favor, introduza novamente o seu conjunto de caracteres abaixo 🔡🔢🔣</i>"),
        ("dialog_wrng_plen", "<i>🚫 Número errado! Por favor, introduza o número novamente 🔢</i>"),
        ("dialog_unk_cmd", "<i>🚫 Comando desconhecido!</i>"),
        ("dialog_ent_cch", "<i>Introduza o seu conjunto de caracteres abaixo para gerar uma palavra-passe 🔡🔢🔣</i>"),
        ("dialog_ent_plen", "<i>Introduza o comprimento da sua palavra-passe abaixo 🔢</i>"),
        ("dialog_ent_pqua", "<i>Introduza o número de palavras-passe a gerar 🔢</i>"),
        ("dialog_pwd_is", "A palavra-passe é (clique para copiar):"),
        ("dialog_max_mess_len", "relaxa, porque é que precisas de tantas palavras-passe longas?
Tenta reduzir o número de palavras-passe ou o seu comprimento.
Comprimento da mensagem excedido para o Telegram!"),
        ("help", "<b>🔏 Mammothcoding gerador de palavras-passe para o Telegram.</b>
    Um serviço de bot de telegramas para gerar palavras-passe/tokens criptograficamente seguros e outros conjuntos e sequências.
<i><a href=\"https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs\">CSPRNGs</a> São utilizados Isaac64Rng e Hc128Rng.</i>
<i>🦀 Fabricado com Rust.</i>
<i>🔗 <u><a href=\"https://github.com/mammothcoding/passgen-telegram\">Página do projeto no github</a></u> (privado temporário)</i>

    <u><b>Utilização:</b></u>
🔹 Pode escolher o idioma da interface.
🔹 Pode criar uma palavra-passe normal, escolhendo nas regras a presença de <i>letras pequenas e maiúsculas, números, caracteres especiais</i>.
🔹 Pode criar uma <i>palavra-passe forte e fácil de utilizar</i>:
incluindo todo o conjunto padrão, mas a primeira posição na palavra-passe é uma letra maiúscula ou minúscula, a última posição é o símbolo.
Caracteres ambíguos excluídos <i>\"0oOiIlL1\"</i>.
🔸 Se esta regra estiver activada, as outras regras de consistência do gerador não são utilizadas, exceto a regra <i>\"conjunto de caracteres personalizado\"</i>.
🔹 Você pode criar um conjunto a partir do seu <i>\"conjunto de caracteres personalizado\"</i> que inclui quaisquer caracteres unicode como \"abcABC123⭕➖❎⚫⬛n₼⁂🙂\".
🔸 Este conjunto de caracteres excluirá todas as outras regras, exceto a regra <i>\"palavra-passe forte e de fácil utilização\"</i>.
⚙️ Se <i>\"palavra-passe forte e de fácil utilização\"</i> também estiver activada, pode gerar um resultado combinado forte e de fácil utilização com <i>conjunto de caracteres personalizados</i>.
🔹 Pode especificar o <i>\"comprimento da palavra-passe\"</i> necessário, que não pode ser inferior a 4 nem superior a 3900 caracteres.
🔹 Pode especificar o <i>\"quantidade de palavras-passe\"</i> necessário - não mais de 100.
🔹 Por motivos de segurança, pode eliminar a mensagem com a última palavra-passe criada através do botão
<b>[🧹 pwd]</b> que aparece após ter gerado a palavra-passe.
                                           ☑️/start"),
    ];

    pub const FR: [(&str, &str); 22] = [
        ("menu_lcase", "inclure les lettres minuscules"),
        ("menu_cap", "inclure des lettres majuscules"),
        ("menu_num", "inclure des chiffres"),
        ("menu_ss", "inclure des symboles spéciaux"),
        ("menu_conven", "mot de passe fort et facile à utiliser"),
        ("menu_cch", "le jeu de caractères personnalisé. Appuyez sur pour définir."),
        ("menu_pass_len1", "longueur du mot de passe"),
        ("menu_pass_len2", ". Appuyez sur pour modifier."),
        ("menu_pass_qua", "mots de passe quantité"),
        ("menu_btn_gen", "🎲 GÉNÉRER"),
        ("menu_btn_stat", "📊 statistiques"),
        ("menu_stat_btn_regen", "régénérer"),
        ("menu_stat_btn_close", "fermer"),
        ("dialog_large_cch", "<i>⚠️ Une taille de jeu de caractères personnalisée très importante a été transmise. Veuillez saisir à nouveau votre jeu de caractères ci-dessous 🔡🔢🔣</i>"),
        ("dialog_wrng_plen", "<i>🚫 Mauvais numéro ! Veuillez saisir à nouveau le numéro 🔢</i>"),
        ("dialog_unk_cmd", "<i>🚫 Commande inconnue !</i>"),
        ("dialog_ent_cch", "<i>Veuillez saisir votre jeu de caractères ci-dessous pour générer un mot de passe 🔡🔢🔣</i>"),
        ("dialog_ent_plen", "<i>Veuillez saisir la longueur de votre mot de passe ci-dessous 🔢</i>"),
        ("dialog_ent_pqua", "<i>Veuillez saisir le nombre de mots de passe à générer 🔢</i>"),
        ("dialog_pwd_is", "Le mot de passe est (cliquez pour copier) :"),
        ("dialog_max_mess_len", "relax, pourquoi as-tu besoin d'autant de mots de passe longs ? 😀
Essayez de réduire le nombre de mots de passe ou leur longueur.
Longueur du message dépassée pour Telegram !"),
        ("help", "<b>🔏 Mammothcoding générateur de mot de passe pour Telegram.</b>
    Un service de télégrammes pour générer des mots de passe/tokens et d'autres ensembles et séquences cryptographiquement sécurisés.
<i><a href=\"https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs\">CSPRNGs</a> Isaac64Rng et Hc128Rng sont utilisés.</i>
<i>🦀 Fabriqué avec de la Rust.</i>
<i>🔗 <u><a href=\"https://github.com/mammothcoding/passgen-telegram\">Page github du projet</a></u> (temporaire privé)</i>

    <u><b>Utilisation:</b></u>
🔹 Vous pouvez choisir la langue de l'interface.
🔹 Vous pouvez créer un mot de passe ordinaire, en choisissant dans les règles la présence de <i>lettres minuscules et majuscules, de chiffres, de caractères spéciaux</i>.
🔹 Vous pouvez créer un <i>mot de passe fort et facile à utiliser</i> :
y compris l'ensemble des caractères standard, mais la première position du mot de passe est une lettre majuscule ou minuscule, la dernière position est le symbole.
Les caractères ambigus <i>\"0oOiIlL1\"</i> sont exclus.
🔸 Si cette règle est activée, les autres règles de cohérence du générateur ne sont pas prises en compte, à l'exception d'une règle <i>\"le jeu de caractères personnalisé\"</i>.
🔹 Vous pouvez créer un ensemble à partir de votre <i>\"le jeu de caractères personnalisé\"</i> qui inclut tous les caractères unicode comme \"abcABC123⭕➖❎⚫⬛n₼⁂🙂\".
🔸 Cet ensemble de caractères exclura toutes les autres règles à l'exception d'une règle <i>\"mot de passe fort et facile à utiliser\"</i>.
⚙️ Si l'option <i>\"mot de passe fort et facile à utiliser\"</i> est également activée, vous pouvez générer un résultat combiné <i>\"mot de passe fort et facile à utiliser\"</i> avec <i>\"le jeu de caractères personnalisé\"</i>.
🔹 Vous pouvez spécifier la <i>\"longueur du mot de passe\"</i> requise, qui ne doit pas être inférieure à 4 ni supérieure à 3900 caractères.
🔹 Vous pouvez spécifier le <i>\"mots de passe quantité\"</i> requis - pas plus de 100.
🔹 Pour des raisons de sécurité, vous pouvez supprimer le message contenant le dernier mot de passe que vous avez créé à l'aide du bouton
<b>[🧹 pwd]</b> qui apparaît après avoir généré le mot de passe.
                                           ☑️/start"),
    ];

    pub const DE: [(&str, &str); 22] = [
        ("menu_lcase", "Kleinbuchstaben enthalten"),
        ("menu_cap", "Großbuchstaben enthalten"),
        ("menu_num", "Zahlen einbeziehen"),
        ("menu_ss", "besondere Symbole enthalten"),
        ("menu_conven", "starkes & benutzerfreundliches Passwort"),
        ("menu_cch", "benutzerdefinierten Zeichensatz. Drücken Sie zum Einstellen."),
        ("menu_pass_len1", "Passwort-Länge"),
        ("menu_pass_len2", ". Drücken Sie zum Bearbeiten."),
        ("menu_pass_qua", "Passwörter Menge"),
        ("menu_btn_gen", "🎲 GENERIEREN"),
        ("menu_btn_stat", "📊 Statistik"),
        ("menu_stat_btn_regen", "regenerieren"),
        ("menu_stat_btn_close", "schließen"),
        ("dialog_large_cch", "<i>⚠️ Eine sehr große benutzerdefinierte Zeichensatzgröße wurde überschritten. Bitte geben Sie Ihren Zeichensatz unten erneut ein 🔡🔢🔣</i>"),
        ("dialog_wrng_plen", "<i>🚫 Falsche Nummer! Bitte geben Sie die Nummer erneut ein 🔢</i>"),
        ("dialog_unk_cmd", "<i>🚫 Unbekannter Befehl!</i>"),
        ("dialog_ent_cch", "<i>Bitte geben Sie unten Ihren Zeichensatz ein, um ein Passwort zu generieren 🔡🔢🔣</i>"),
        ("dialog_ent_plen", "<i>Bitte geben Sie unten die Länge Ihres Passworts ein 🔢</i>"),
        ("dialog_ent_pqua", "<i>Bitte geben Sie die Anzahl der zu generierenden Passwörter ein 🔢</i>"),
        ("dialog_pwd_is", "Das Passwort lautet (zum Kopieren anklicken):"),
        ("dialog_max_mess_len", "Relax, warum brauchen Sie so viele lange Passwörter? 😀
Versuchen Sie, die Anzahl der Passwörter oder ihre Länge zu reduzieren.
Nachrichtenlänge für Telegram überschritten!"),
        ("help", "<b>🔏 Mammothcoding passwort-Generator für Telegram.</b>
    Ein Telegramm-Botdienst zur Erzeugung von kryptographisch sicheren Passwörtern/Tokens und anderen Mengen und Sequenzen.
<i><a href=\"https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs\">CSPRNGs</a> Isaac64Rng und Hc128Rng werden verwendet.</i>
<i>🦀 Hergestellt mit Rust.</i>
<i>🔗 <u><a href=\"https://github.com/mammothcoding/passgen-telegram\">Projekt github Seite</a></u> (vorübergehend privat)</i>

    <u><b>Verwendung:</b></u>
🔹 Sie können die Sprache der Schnittstelle wählen.
🔹 Sie können ein normales Passwort erstellen, indem Sie in den Regeln das Vorhandensein von Klein- und Großbuchstaben, Zahlen und Sonderzeichen festlegen.
🔹 Sie können ein <i>\"starkes & benutzerfreundliches Passwort\"</i> erstellen:
einschließlich des gesamten Standardsatzes, aber die erste Position im Passwort ist ein Groß- oder Kleinbuchstabe, die letzte Position ist das Symbol.
Ausgenommen mehrdeutige Zeichen <i>\"0oOiIlL1\"</i>.
🔸 Wenn diese Regel aktiviert ist, werden die anderen Konsistenzregeln der Generierung nicht berücksichtigt, mit Ausnahme der Regel <i>\"benutzerdefinierten Zeichensatz\"</i>.
🔹 Sie können einen Satz aus Ihrem <i>\"benutzerdefinierten Zeichensatz\"</i> erstellen, der alle Unicode-Zeichen wie \"abcABC123⭕➖❎⚫⬛n₼⁂🙂\" enthält.
🔸 Dieser Zeichensatz schließt alle anderen Regeln aus, mit Ausnahme der Regel <i>\"starkes & benutzerfreundliches Passwort\"</i>.
⚙️ Wenn auch <i>\"starkes & benutzerfreundliches Passwort\"</i> aktiviert ist, können Sie mit <i>\"benutzerdefinierten Zeichensatz\"</i> ein kombiniertes <i>starkes & benutzerfreundliches</i> Ergebnis erzeugen.
🔹 Sie können die erforderliche <i>\"Passwort-Länge\"</i> von nicht weniger als 4 und nicht mehr als 3900 Zeichen angeben.
🔹 Sie können die gewünschte <i>\"Passwörter Menge\"</i> angeben - nicht mehr als 100.
🔹 Aus Sicherheitsgründen können Sie die Nachricht mit Ihrem zuletzt erstellten Passwort über die Schaltfläche
<b>[🧹 pwd]</b> löschen, die nach der Passwortgenerierung angezeigt wird.
                                           ☑️/start"),
    ];

    pub const RU: [(&str, &str); 22] = [
        ("menu_lcase", "включая маленькие буквы"),
        ("menu_cap", "включая заглавные буквы"),
        ("menu_num", "включая цифры"),
        ("menu_ss", "включая спец. символы"),
        ("menu_conven", "сильный и удобный пароль"),
        ("menu_cch", "установить свой набор симв."),
        ("menu_pass_len1", "длина пароля"),
        ("menu_pass_len2", ". Установить."),
        ("menu_pass_qua", "количество паролей"),
        ("menu_btn_gen", "🎲 СГЕНЕРИРОВАТЬ"),
        ("menu_btn_stat", "📊 статистика"),
        ("menu_stat_btn_regen", "перегенерировать"),
        ("menu_stat_btn_close", "закрыть"),
        ("dialog_large_cch", "<i>⚠️ Превышен размер пользовательского набора символов. Пожалуйста, введите набор символов поменьше еще раз 🔡🔢🔣</i>"),
        ("dialog_wrng_plen", "<i>🚫 Неправильное число! Пожалуйста, введите число еще раз 🔢.</i>"),
        ("dialog_unk_cmd", "<i>🚫 Неизвестная команда!</i>"),
        ("dialog_ent_cch", "<i>Пожалуйста, введите свой пользовательский набор символов 🔡🔢🔣</i>"),
        ("dialog_ent_plen", "<i>Пожалуйста, введите длину пароля 🔢</i>"),
        ("dialog_ent_pqua", "<i>Пожалуйста, введите количество генерируемых паролей 🔢</i>"),
        ("dialog_pwd_is", "Пароль (нажмите, чтобы скопировать):"),
        ("dialog_max_mess_len", "расслабься, зачем тебе столько длинных паролей?😀
Попробуй уменьшить количество паролей или их длину.
Превышена допустимая длина сообщения для Telegram!"),
        ("help", "<b>🔏 Mammothcoding генератор паролей для Telegram.</b>
    Телеграм-бот-сервис для создания криптографически защищенных паролей/токенов и других наборов и последовательностей.
<i>В генераторе используются <a href=\"https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs\">CSPRNGs</a> Isaac64Rng and Hc128Rng.</i>
<i>🦀 Создано на языке Rust.</i>
<i>🔗 <u><a href=\"https://github.com/mammothcoding/passgen-telegram\">Гитхаб проекта</a></u> (временно приватный)</i>

    <u><b>Использование:</b></u>
🔹 Вы можете выбрать язык интерфейса.
🔹 Вы можете создать обычный пароль, регулируя правила наличия <i>малых и заглавных букв, цифр, специальных символов</i>.
🔹 Вы можете создать <i>\"сильный и удобный пароль\"</i>:
включающий весь стандартный набор, но первая позиция в пароле - заглавная или строчная буква, последняя позиция - символ.
Исключены неоднозначные символы <i>\"0oOiIlL1\"</i>.
🔸 Если это правило включено, то другие правила не учитываются, за исключением правила <i>\"свой набор символов\"</i>.
🔹 Вы можете указать собственный набор символов для генератора <i>\"свой набор символов\"</i>, включающий любые символы юникода, например \"abcABC123⭕➖❎⚫⬛n₼⁂🙂\"
🔸 Этот набор символов исключит все остальные правила, кроме правила <i>\"сильный и удобный пароль\"</i>.
⚙️ Если правило <i>\"сильный и удобный пароль\"</i> тоже включено, то вы можете сгенерировать комбинированный результат <i>\"сильный и удобный пароль\"</i> с <i>пользовательским набором символов</i>.
🔹 Вы можете указать необходимую <i>\"длину пароля\"</i> - не менее 4 и не более 3900 символов.
🔹 Вы можете указать необходимое <i>\"количество паролей\"</i> - не более 100.
🔹 В целях безопасности вы можете удалить сообщение с последним созданным паролем с помощью кнопки
<b>[🧹 pwd]</b>, которая появляется после генерирования пароля.
                                           ☑️/start"),
    ];

    pub async fn get_lang_map(chat_id: i64, bot_id: i64) -> HashMap<&'static str, &'static str> {
        let user_lang: String = match get_user_app_lang(chat_id, bot_id).await {
            Some(lang_id) => lang_id,
            _ => "en".to_string(),
        };

        let lang_name: String = user_lang.clone().to_owned();
        let lang_name_slice: &str = &lang_name[..];

        match lang_name_slice {
            "en" => HashMap::from_iter(EN),
            "es" => HashMap::from_iter(ES),
            "pt" => HashMap::from_iter(PT),
            "fr" => HashMap::from_iter(FR),
            "de" => HashMap::from_iter(DE),
            "ru" => HashMap::from_iter(RU),
            _ => HashMap::from_iter(EN),
        }
    }

    pub fn obtain_user_lang_code(user_lang_code: &str) -> &str {
        match user_lang_code {
            "es" => user_lang_code,
            "pt" => user_lang_code,
            "fr" => user_lang_code,
            "de" => user_lang_code,
            "ru" => user_lang_code,
            _ => "en",
        }
    }
}
