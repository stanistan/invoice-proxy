use serde::Deserialize;

/// The shape of an HTTP response from Airtable for an object/entity request.
#[derive(Deserialize, Debug)]
pub struct One<T> {
    pub id: String,
    pub fields: T,
    #[serde(rename = "createdTime")]
    pub created_time: String,
}

/// A list of single entity objects.
pub type List<T> = Vec<One<T>>;

/// The shape of an HTTP response from Airtable for muliple objects.
#[derive(Deserialize, Debug)]
pub struct Many<T> {
    pub records: List<T>,
}
