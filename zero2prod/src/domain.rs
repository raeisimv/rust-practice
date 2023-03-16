use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn parse(name: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("SubscriberName is empty".into());
        }
        if name.graphemes(true).count() > 256 {
            return Err("SubscriberName is too large".into());
        }

        let forbidden_chars = ['/', '\\', '{', '}', '(', ')', ';', '<', '>', '"'];
        if name.chars().any(|x| forbidden_chars.contains(&x)) {
            return Err("SubscriberName contains forbidden chars".into());
        }

        Ok(Self(name))
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_ok, assert_err};
    use crate::domain::SubscriberName;

    #[test]
    fn a_256_graphemes_long_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_forbidden_chars_are_rejected() {
        let forbidden_chars = ['/', '\\', '{', '}', '(', ')', ';', '<', '>', '"'];
        for ch in forbidden_chars {
            assert_err!(SubscriberName::parse(ch.to_string()));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Morteza Raeisi Vanani".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}