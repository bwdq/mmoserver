use crate::character::Character;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Account {
    id: u64,
    name: String,
    password: String,
    characters: Vec<Character>,
}
