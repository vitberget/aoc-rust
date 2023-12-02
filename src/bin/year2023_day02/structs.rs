#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub picks: Vec<Pick>
}

#[derive(Debug, PartialEq)]
pub struct Pick {
    pub red: u32,
    pub green: u32,
    pub blue: u32
}
