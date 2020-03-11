use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct One<T> {
    pub id: String,
    pub fields: T,
    #[serde(rename = "createdTime")]
    pub created_time: String,
}

#[derive(Deserialize, Debug)]
pub struct Many<T> {
    pub records: Vec<One<T>>,
}

