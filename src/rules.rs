pub mod rules {
    use log::debug;

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct Rules {
        pub enab_letters: bool,
        pub enab_u_letters: bool,
        pub enab_num: bool,
        pub enab_spec_symbs: bool,
        pub enab_strong_usab: bool,
        pub custom_charset: String,
        pub pwd_len: u64,
        pub pwd_quantity: u64,
    }

    impl Rules {
        pub fn reconfigure_rules_according_selector(&mut self, action: String) {
            let act: &str = &action[..];

            match act {
                // custom_charset must already have the change applied
                "custom_charset" => {
                    if self.custom_charset.is_empty() && !self.enab_strong_usab {
                        self.enab_strong_usab = true;
                    }
                    if !self.custom_charset.is_empty() {
                        self.enab_strong_usab = false;
                        self.set_simple_rule_field("enab_letters", false);
                        self.set_simple_rule_field("enab_u_letters", false);
                        self.set_simple_rule_field("enab_num", false);
                        self.set_simple_rule_field("enab_spec_symbs", false);
                    }
                }
                "enab_strong_usab" => {
                    if self.enab_strong_usab {
                        self.enab_strong_usab = false;
                        if self.custom_charset.is_empty() {
                            self.set_simple_rule_field("enab_letters", true);
                            self.set_simple_rule_field("enab_u_letters", true);
                            self.set_simple_rule_field("enab_num", true);
                            self.set_simple_rule_field("enab_spec_symbs", true);
                        }
                    } else {
                        self.enab_strong_usab = true;
                        self.set_simple_rule_field("enab_letters", false);
                        self.set_simple_rule_field("enab_u_letters", false);
                        self.set_simple_rule_field("enab_num", false);
                        self.set_simple_rule_field("enab_spec_symbs", false);
                    }
                }
                _ => {
                    if self.get_simple_rule_field(act) {
                        self.set_simple_rule_field(act, false);
                        if !self.enab_letters
                            && !self.enab_u_letters
                            && !self.enab_num
                            && !self.enab_spec_symbs
                        {
                            self.enab_strong_usab = true;
                        }
                    } else {
                        self.set_simple_rule_field(act, true);
                        self.custom_charset = "".to_string();
                        self.enab_strong_usab = false;
                    }
                }
            }

            debug!("📗 Reconfiguring rules by action {act} successfully.");
        }

        pub fn get_simple_rule_field(&self, field: &str) -> bool {
            match field {
                "enab_letters" => self.enab_letters,
                "enab_u_letters" => self.enab_u_letters,
                "enab_num" => self.enab_num,
                "enab_spec_symbs" => self.enab_spec_symbs,
                "enab_strong_usab" => self.enab_strong_usab,
                _ => false,
            }
        }

        pub fn set_simple_rule_field(&mut self, field: &str, value: bool) {
            match field {
                "enab_letters" => self.enab_letters = value,
                "enab_u_letters" => self.enab_u_letters = value,
                "enab_num" => self.enab_num = value,
                "enab_spec_symbs" => self.enab_spec_symbs = value,
                "enab_strong_usab" => self.enab_strong_usab = value,
                _ => {}
            }
        }
    }
}
