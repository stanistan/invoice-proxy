#[allow(unused)]
pub(crate) fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[macro_export]
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        crate::gen_schema::compose_two($head, compose!($($tail),+))
    };
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
#[macro_export]
macro_rules! gen_airtable_schema {
    (
        @gen $(
            $ns:ident, $ns_name:expr, $name:expr, [ $(
                $json_name:expr, $field_name:ident, $field_type:ty, -> [ $($t:expr,)* ]: $t_field:ty
            )*],
        )*
    ) => {
            $(
                pub mod $ns {
                    #![doc = "This namespace `"]
                    #![doc = $ns_name]
                    #![doc = "` is generated by [`gen_airtable_schema`](../macro.gen_airtable_schema.html)."]
                    #![allow(unused)]

                    use serde::*;
                    use serde_json::Value;
                    use crate::airtable::FetchCtx;
                    use super::*;

                    /// The table name of this airtable table.
                    ///
                    /// This will be used in `get_one`, and other functions
                    /// defined in this namespace, to fetch data.
                    pub const NAME: &'static str = $name;

                    /// This is autogenerated, and corresponds to the `fields`
                    /// internal to a single record.
                    #[derive(Debug, Deserialize)]
                    pub struct Fields {
                        $( #[serde(rename = $json_name)] pub $field_name: $field_type,)*
                    }

                    /// This is autogenerated, and corresponds to what the fully
                    /// hydrated `One` transforms it into.
                    ///
                    /// See: [`map`](./fn.map.html)
                    #[derive(Debug)]
                    pub struct Mapped {
                        $( pub $field_name: $t_field, )*
                    }

                    /// A single record, see
                    /// [`airtable::response::One`](../airtable/response/struct.One.html)
                    pub type One = crate::airtable::response::One<Fields>;

                    /// Many records, see
                    /// [`airtable::response::Many`](../airtable/response/struct.Many.html)
                    pub type Many = crate::airtable::response::Many<Fields>;

                    /// Get a single typed record via the `FetchCtx`.
                    pub async fn get_one<T: AsRef<str>>(ctx: &FetchCtx, id: T) -> Result<One, Error> {
                        ctx.id_request(NAME, id.as_ref()).send().await
                            .map_err(|e| Error::Req(e))?
                            .json::<One>().await
                            .map_err(|e| Error::Req(e))
                    }

                    pub async fn get_many_by_id<T: AsRef<str>>(ctx: &FetchCtx, ids: Vec<T>) -> Result<Vec<One>, Error> {
                        let mut result = Vec::with_capacity(ids.len());
                        for id in ids {
                            result.push(get_one(ctx, id).await?);
                        }
                        Ok(result)
                    }

                    /// Get a signle _dynamic_ JSON record via the `FetchCtx`.
                    pub async fn get_one_dynamic(ctx: &FetchCtx, id: &str) -> Result<Value, Error> {
                        ctx.id_request(NAME, id).send().await
                            .map_err(|e| Error::Req(e))?
                            .json::<Value>().await
                            .map_err(|e| Error::Req(e))
                    }

                    /// Given a typed API response, create the fully hydrated `Mapped` resource.
                    pub async fn map_one(ctx: &FetchCtx, one: One) -> Result<Mapped,  Error> {
                        Ok(Mapped {
                            $($field_name: {
                                let val = one.fields.$field_name;
                                $( let val = $t(ctx, val).await?; )*
                                val
                            }),*
                        })
                    }

                    pub async fn map_many(ctx: &FetchCtx, many: Vec<One>) -> Result<Vec<Mapped>, Error> {
                        let mut result = Vec::with_capacity(many.len());
                        for one in many {
                            result.push(map_one(ctx, one).await?);
                        }
                        Ok(result)
                    }

                }
            )*
    };

    (
        $(
            mod $ns:ident ($name: expr) {
                    $(
                        $k:expr => fn $fn:ident ($ft:ty) -> $t_ft:ty { $($tfs:expr),+ },
                    )*
            }
        )*
    ) => {
        gen_airtable_schema! {
            @gen $(
                $ns, stringify!($ns), $name, [
                    $(
                        $k, $fn, $ft, -> [ $($tfs,)* ]: $t_ft
                     )*
                ],
            )*
        }
    }
}
