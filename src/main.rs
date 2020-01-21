mod airtable {

    #[derive(Debug)]
    pub(crate) struct Config {
        pub key: String,
        pub base: String,
    }

    impl Config {
        pub(crate) fn from_env() -> Result<Self, &'static str> {
            use std::env;
            match (env::var("AIRTABLE_KEY"), env::var("AIRTABLE_APP")) {
                (Ok(key), Ok(base)) => Ok(Self { key, base }),
                _ => Err("Expected env variables AIRTABLE_KEY, and AIRTABLE_APP to be set"),
            }
        }
    }

    pub(crate) struct FetchCtx {
        config: Config,
        client: reqwest::Client,
    }

    impl FetchCtx {
        pub(crate) fn from_env() -> Result<Self, &'static str> {
            let config = Config::from_env()?;
            Ok(Self {
                config,
                client: reqwest::Client::new(),
            })
        }

        pub(crate) fn id_request(&self, table: &str, id: &str) -> reqwest::RequestBuilder {
            let url = format!(
                "https://api.airtable.com/v0/{base}/{table}/{id}",
                base = self.config.base,
                table = table,
                id = id
            );
            self.client.get(&url).bearer_auth(&self.config.key)
        }
    }

    pub(crate) mod response {

        use serde::Deserialize;

        #[derive(Deserialize, Debug)]
        pub struct One<T> {
            id: String,
            fields: T,
            #[serde(rename = "createdTime")]
            created_time: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Many<T> {
            records: Vec<One<T>>,
        }
    }
}

///
/// Generates type definitions for a specific airtable record type,
/// in a module... the structs are built to be deserializable
///
/// This will end up providing built in types and functions:
///
/// - `module::NAME` - the table name of the Airtable base
/// - `module::Fields` - a struct defining the fields of `NAME`
/// - `module::One` - a struct for a single record with `fields: Fields`
/// - `module::Many` - a struct for many `records` (for a mget), of `One`
/// - `module::get_one()` - a function to get `One` record
///
macro_rules! airtable_type {
    (
        $ns:ident
            - from: $name: expr,
            - fields: [
                $( $json_name:expr => $field_name:ident : $field_type: ty,)*
            ]
    ) => {
        pub(crate) mod $ns {
            #![allow(unused)]

            use serde::*;

            pub(crate) const NAME: &'static str = $name;

            #[derive(Debug, Deserialize)]
            pub(crate) struct Fields {
                $(
                    #[serde(rename = $json_name)]
                    pub $field_name: $field_type,
                )*
            }

            pub(crate) type One = crate::airtable::response::One<Fields>;

            pub(crate) type Many = crate::airtable::response::Many<Fields>;

            pub(crate) async fn get_one(ctx: &crate::airtable::FetchCtx, id: &str) -> Result<One, reqwest::Error> {
                ctx.id_request(NAME, id).send().await?.json::<One>().await
            }
        }
    }
}

airtable_type! {
    invoice
        - from: "Invoice",
        - fields: [
            "ID" => id: u32,
            "Invoice Number" => number: String,
            "Notes" => notes: Option<String>,
            "Date" => date: String,
            "Due Date" => due_date: String,
            "Paid" => paid: Option<bool>,
            "Sent" => sent: Option<bool>,
            "Client" => client_ids: Vec<String>,
            "Invoice Item" => invoice_item_ids: Vec<String>,
        ]
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    //
    // We need to have the config in order to be able to talk
    // to the Airtable API at all.
    let ctx = airtable::FetchCtx::from_env().unwrap();
    let an_invoice = invoice::get_one(&ctx, "recLYHi5nzYLlHseu").await?;
    println!("{:#?}", an_invoice);

    Ok(())
}
