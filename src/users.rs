use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub name: String,
    age: u8,
    pub(crate) gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }
    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut data = String::new();

        file.read_to_string(&mut data)?;
        let user = serde_json::from_str(&data)?;

        Ok(user)
    }
    pub fn persit(&self, filename: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(filename)?;
        let data = serde_json::to_string(self)?;
        file.write_all(data.as_bytes())?;
        /*         match File::create(filename) {
            Ok(file) => {
                todo!()
            }
            Err(e) => return Err(e),
        } */

        Ok(data.len())
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("Unknown User".into(), 0, Gender::Unspecified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_file() {
        let user = User::default();
        user.persit("test.json").unwrap();
    }
    #[test]
    fn load_file() {
        let user = User::default();
        let user2 = User::load("test.json").unwrap();
        assert_eq!(user, user2);
    }
}
