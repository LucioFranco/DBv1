use super::error::Error;

// TODO: Implement Identifier
#[derive(Debug, Clone)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: &str) -> Result<Self, Error> {
        if Identifier::check_string(name) {
            Ok(Identifier { name: name.to_string() })
        } else {
            Err(Error::NotValidIdentifier(name.to_owned()))
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn check_string(_val: &str) -> bool {
        // TODO: Implement check_string for Identifier
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert!(Identifier::new("user_v1").is_ok());
    }
}
