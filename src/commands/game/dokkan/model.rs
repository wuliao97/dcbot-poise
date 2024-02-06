use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rank {
    pub devided_rank: Vec<Devide>
}

#[derive(Debug, Deserialize)]
pub struct Devide {
    pub details: Vec<Detail>
}

#[derive(Debug, Deserialize)]
pub struct Detail {
    pub rank: i64,
    pub exp: i64,
    pub act: i64
}


#[derive(Debug, Deserialize)]
pub struct EXP {
    pub story: Story
}
#[derive(Debug, Deserialize)]
pub struct Story(Vec<Area>);

#[derive(Debug, Deserialize)]
pub struct Area {
    pub title: String,
    pub area_num: String,
    pub act: i64,
    pub exp: i64,
    pub store: String,
    pub description: String
}


