use super::Error;

// TODO: Implement Identifier
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: &str) -> Result<Self, Error> {
        Ok(Identifier {
            name: name.to_string()
        })
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
