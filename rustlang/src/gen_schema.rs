#[macro_export]
macro_rules! build_route {
    ($ctx:expr, [ ]) => {
        unimplemented!("Missing any defined endpoints")
    };
    ($ctx:expr, [ $name:expr ]) => {
        $name($ctx.clone())
    };
    ($ctx:expr, [ $name:expr, $($ns:expr),+ ]) => {
        build_route!($ctx, $name($ctx.clone()), [ $($ns),+ ])
    };
    ($ctx:expr, $r:expr, [ ]) => {
        $r
    };
    ($ctx:expr, $r:expr, [ $name:expr ]) => {
        $r.or($name($ctx.clone()))
    };
    ($ctx:expr, $r:expr, [ $name:expr, $($ns:expr),+ ]) => {
        build_route!($ctx, $r.or($name($ctx.clone())), [ $($ns),+ ])
    };
}

///
/// Generates type definitions for a specific airtable record type,
/// in a module... the structs are built to be deserializable.
///
#[macro_export]
macro_rules! gen_airtable_schema {
    (
        @endpoints_route $ns:ident { $($_:tt),* }
    ) => {
        $ns::endpoints::route
    };
    (
        @endpoints $ns_name:expr; $(
            $name:ident($arg_type:ty) -> $out_type:ty {
                $($t:expr),*
            }
        )*
    ) => {
        //
        // Generated endpoints using the `endpoints` part
        // of the grammar/macro.
        //
        // Each endpoint that's defined goes into its own module, with the actual endpoint
        // functionality in `::run`, and the defined `warp` filter as `::route`.
        //
        // The main module will have a `route` that composes all of the ones internal.
        // :cool:
        pub mod endpoints {

            use super::*;
            use crate::ctx::{Ctx};
            use warp::{Filter, Reply, Rejection};

            pub fn route(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
                crate::build_route!(ctx, [ $($name::route),* ])
            }

            $(pub mod $name {

                use super::*;
                use crate::ctx::with_ctx;

                pub fn route(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
                    use warp::Filter;
                    warp::path($ns_name)
                        .and(warp::path::param::<$arg_type>())
                        .and(warp::get())
                        .and(with_ctx(ctx))
                        .and_then(run)
                }

                pub async fn run(arg: $arg_type, ctx: Ctx) -> Result<impl Reply, Rejection> {
                    async fn f(ctx: &mut FetchCtx, arg: $arg_type) -> Result<$out_type, Error> {
                        compose!(ctx, arg, [ $($t),* ])
                    }
                    let mut c = ctx.lock().await;
                    match f(&mut c, arg).await {
                        Ok(val) => Ok(warp::reply::json(&val)),
                        Err(_) => Err(warp::reject::not_found())
                    }
                }
            })*

        }
    };

    (
        @gen $(
            $ns:ident, $ns_name:expr, $name:expr, $mapped_name:ident [ $(
                $json_name:expr, $field_name:ident, $field_type:ty, -> [ $($t:expr,)* ]: $t_field:ty
            )*], { $(mod $($mod_tokens:tt)*)? } { $(endpoints $($endpoints_tokens:tt)*)? }
        )*
    ) => {

        use crate::airtable::{FetchCtx, Table};
        use crate::network::response::One;
        use crate::compose;
        use serde::{Serialize, Deserialize};

        $(
            pub mod $ns {

                use super::*;

                #[allow(unused)]
                #[derive(Debug, Deserialize)]
                pub struct Fields {
                    $( #[serde(rename = $json_name)] pub $field_name: $field_type,)*
                }

                #[allow(unused)]
                #[derive(Debug, Serialize)]
                pub struct Mapped {
                    $( pub $field_name: $t_field, )*
                }

                $($($mod_tokens)*)?

                $(gen_airtable_schema!(@endpoints $ns_name; $($endpoints_tokens)*);)?
            }

            pub type $mapped_name = $ns::Mapped;

            impl Table for $mapped_name {
                const NAME: &'static str = $name;
                type Fields = $ns::Fields;
            }

            #[allow(unused)]
            impl $mapped_name {

                #[inline(always)]
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


        )*  // end for each

        // a generated route for all of the endpoints created by the schema, yea i know, it cray.
        pub fn route(ctx: crate::ctx::Ctx) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            use warp::Filter;
            crate::build_route!(ctx, crate::ctx::ctx_cache::route(ctx.clone()), [
                $( $(gen_airtable_schema!(@endpoints_route $ns { $($endpoints_tokens),* }))? )*
            ])
        }

    };

    // This is the main user entrypoint into this macro, it forwards to the `@gen`,
    // stringifying the `ns` names so that they can be used as strings.
    (
        $(
            $ns:ident ($name: expr)
                as $mapped_name:ident {
                    $(
                        $k:expr => fn $fn:ident ($ft:ty) -> $t_ft:ty { $($tfs:expr),+ },
                    )*
                }
                $(, mod { $($mod_tokens:tt)* })?
                $(, endpoints { $($endpoints_tokens:tt)* })?
            ;
        )*
    ) => {
        gen_airtable_schema! {
            @gen $(
                $ns, stringify!($ns), $name, $mapped_name [
                    $(
                        $k, $fn, $ft, -> [ $($tfs,)* ]: $t_ft
                     )*
                ],
                { $(mod $($mod_tokens)*)? }
                { $(endpoints $($endpoints_tokens)*)? }
            )*
        }
    }
}
