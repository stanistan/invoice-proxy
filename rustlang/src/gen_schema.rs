///
/// Generates type definitions for a specific airtable record type,
/// in a module... the structs are built to be deserializable.
///
#[macro_export]
macro_rules! gen_airtable_schema {

    (
        @gen $(
            $ns:ident, $ns_name:expr, $name:expr, $mapped_name:ident [ $(
                $json_name:expr, $field_name:ident, $field_type:ty, -> [ $($t:expr,)* ]: $t_field:ty
            )*], $({ $($tokens:tt)* })?
        )*
    ) => {

        use crate::airtable::{FetchCtx, response::One, Table};
        use crate::compose;

        $(
            pub mod $ns {
                #![doc = "This namespace `"]
                #![doc = $ns_name]
                #![doc = "` is generated by [`gen_airtable_schema`](../../macro.gen_airtable_schema.html)."]
                #![allow(unused)]

                use serde::*;
                use serde_json::Value;
                use super::*;

                /// This is autogenerated, and corresponds to the `fields`
                /// internal to a single record.
                #[derive(Debug, Deserialize)]
                pub struct Fields {
                    $( #[serde(rename = $json_name)] pub $field_name: $field_type,)*
                }

                #[derive(Debug, Serialize)]
                pub struct Mapped {
                    $( pub $field_name: $t_field, )*
                }

                // dump in arbitrary tokens, like any functions that were added to this module
                // after stuff has been defined.
                $($($tokens)*)?
            }

        #[allow(unused)]
        #[doc = "Generated alias from `"]
        #[doc = $ns_name]
        #[doc = ":: Mapped`."]
        pub type $mapped_name = $ns::Mapped;

        impl Table for $mapped_name {
            const NAME: &'static str = $name;
            type Fields = $ns::Fields;
        }

        #[allow(unused)]
        impl $mapped_name {

            fn new_ids_params(ids: Vec<String>) -> Param<Self> {
                Param::new_id(ids)
            }

            pub async fn create_one(ctx: &mut FetchCtx, one: One<$ns::Fields>) -> Result<Self,  Error> {
                Ok(Self {
                    $($field_name: compose!(ctx, one.fields.$field_name, [ $($t),* ])?),*
                })
            }

            pub async fn create_many(ctx: &mut FetchCtx, many: Vec<One<$ns::Fields>>) -> Result<Vec<Self>, Error> {
                let mut result = Vec::with_capacity(many.len());
                for one in many {
                    result.push(Self::create_one(ctx, one).await?);
                }
                Ok(result)
            }

            pub async fn fetch_and_create_first(ctx: &mut FetchCtx, ids: Vec<String>) -> Result<Self, Error> {
                let params = Self::new_ids_params(ids);
                compose!(ctx, params, [ one, Self::create_one ])
            }

            pub async fn fetch_and_create_many(ctx: &mut FetchCtx, ids: Vec<String>) -> Result<Vec<Self>, Error> {
                let params = Self::new_ids_params(ids);
                compose!(ctx, params, [ many, Self::create_many ])
            }

        }

        )*
    };

    (
        $(
            mod $ns:ident ($name: expr) as $mapped_name:ident {
                    $(
                        $k:expr => fn $fn:ident ($ft:ty) -> $t_ft:ty { $($tfs:expr),+ },
                    )*
            } $({ $($tokens:tt)* })?
        )*
    ) => {
        gen_airtable_schema! {
            @gen $(
                $ns, stringify!($ns), $name, $mapped_name [
                    $(
                        $k, $fn, $ft, -> [ $($tfs,)* ]: $t_ft
                     )*
                ],
                $({ $($tokens)* })?
            )*
        }
    }
}
