pub trait Table {
    const NAME: &'static str;
    type Fields: serde::de::DeserializeOwned;
}

#[macro_export]
macro_rules! __gen_inner {

    (
        @main $($name:ident ($table:expr) -> $out:ident { $($inner:tt)* })*
    ) => {
        pub mod gen {
            use super::*;
            use crate::airtable::FetchCtx;
            use crate::compose;
            use crate::error::Error;
            use crate::gen_schema::Table;
            use crate::network::request::*;
            use crate::network::response::One;
            use crate::transform::*;
            use serde::{Serialize, Deserialize};
            $(__gen_inner!{@table $name, stringify!($name), ($table) -> $out { $($inner)* }})*

            /// Generated `warp::Filter` for all endpoints created by the schema.
            pub fn route(ctx: crate::ctx::Ctx) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
                use warp::Filter;
                let ctx_cache = crate::ctx::ctx_cache::route(ctx.clone());
                crate::build_route!(ctx, ctx_cache, [ ])
            }
        }
    };

    (
        @table $mod_name:ident, $mod_str_name:expr, ($table:expr) -> $type:ident {
            $(fields { $($fields:tt)* })?
            $(module { $($module:tt)* })?
            $(endpoints { $($endpoints:tt)* })?
        }
    ) => {
        pub mod $mod_name {
            #![allow(unused)]
            use super::*;

            // generate the fields and structs for mapping/transformation
            $(__gen_inner!{@fields $table, [ $($fields)* ]})?

            // insert any module that's been done there, inlined
            $($($module)*)?

            // generate an endpoints module
            pub mod endpoints {
                #![allow(unused)]
                use super::*;
                use crate::ctx::Ctx;
                use warp::{Filter, Reply, Rejection};
                __gen_inner!{@endpoints $mod_str_name, [ $($($endpoints)*)? ]}
            }
        }

        /// Generated type alias for `Mapped` in the module.
        type $type = $mod_name::Mapped;
    };
    ( @choose_field_type $type1:ty | $type2:ty) => { $type1 };
    ( @choose_field_type | $type:ty) => { $type };
    ( @choose_field_type $type:ty |) => { $type };
    ( @choose_field_type |) => { String };
    (
        @endpoints $mod_str_name:expr, [
            $($name:ident ($from:ty) -> $to:ty {
                url_path { $($path_tokens:tt)* }
                $(exec = $($exec:expr),*;)?
            })*
        ]
    ) => {
        $(pub mod $name {
            #![allow(unused)]
            use super::*;
            use crate::ctx::with_ctx;

            pub fn route(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
                warp::path($mod_str_name)
                    .and(warp::path::param::<$from>())
                    .and(warp::get())
                    .and(with_ctx(ctx))
                    .and_then(run)
            }

            pub async fn run(arg: $from, ctx: Ctx) -> Result<impl Reply, Rejection> {
                async fn handler(ctx: &mut FetchCtx, arg: $from) -> Result<$to, Error> {
                    compose!(ctx, arg, [ $($($exec),*)? ])
                }

                let mut c = ctx.lock().await;
                match handler(&mut c, arg).await {
                    Ok(val) => Ok(warp::reply::json(&val)),
                    Err(e) => Err(warp::reject::custom(e))
                }
            }

        })*

        pub fn route(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
            let d = warp::get().map(|| { $mod_str_name });
            crate::build_route!(ctx, d, [ $($name::route),* ])
        }
    };
    //
    // Macro generates the `Fields` type, the `Mapped` type,
    // which correspond respectively, to the JSON shape that comes back
    // and we want to parse for the module, as well as the more complex
    // mapping of the fully hydrated type that will be constructed.
    (
        @fields $table:expr, [
            $($name:ident $(($from:ty))? $(-> $to:ty)? {
                source = $rename:expr;
                $(exec = $($exec:expr),*;)?
            })*
        ]
    ) => {
        #[derive(Debug, Deserialize)]
        pub struct Fields {
            $(
                #[serde(rename = $rename)]
                pub $name: __gen_inner!(@choose_field_type $($from)? | $($to)?),
            )*
        }

        #[derive(Debug, Serialize)]
        pub struct Mapped {
            $( pub $name: __gen_inner!(@choose_field_type $($to)? |),)*
        }

        impl Table for Mapped {
            const NAME: &'static str = $table;
            type Fields = Fields;
        }

        impl Mapped {

            pub async fn create_one(ctx: &mut FetchCtx, one: One<Fields>) -> Result<Self, Error> {
                Ok(Self {
                    $($name: compose!(ctx, one.fields.$name, [ $($($exec),*)? ])?),*
                })
            }

            pub async fn create_many(ctx: &mut FetchCtx, many: Vec<One<Fields>>) -> Result<Vec<Self>, Error> {
                // TODO: this should not block/await on each loop,
                // but let them all run in parallel until they're done,
                // then we can accumulate them into the output vector.
                let mut result = Vec::with_capacity(many.len());
                for one in many {
                    result.push(Self::create_one(ctx, one).await?);
                }
                Ok(result)
            }

            pub async fn fetch_and_create_first(ctx: &mut FetchCtx, ids: Vec<String>) -> Result<Self, Error> {
                let params: Param<Self> = Param::new_id(ids);
                compose!(ctx, params, [ one, Self::create_one ])
            }

            pub async fn fetch_and_create_many(ctx: &mut FetchCtx, ids: Vec<String>) -> Result<Vec<Self>, Error> {
                let params: Param<Self> = Param::new_id(ids);
                compose!(ctx, params, [ many, Self::create_many ])
            }
        }

    };
}

#[macro_export]
macro_rules! gen_airtable_schema2 {
    ($($tt:tt)*) => {
        __gen_inner!{@main $($tt)*}
    }
}

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
    ($ctx:expr, $r:expr, [ $(,)* ]) => {
        $r
    };
    ($ctx:expr, $r:expr, [ $name:expr ]) => {
        $r.or($name($ctx.clone()))
    };
    ($ctx:expr, $r:expr, [ $name:expr, ]) => {
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
                let default = warp::get().map(|| {
                    ":shrug:"
                });
                crate::build_route!(ctx, default, [ $($name::route),* ])
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
                        Err(e) => Err(warp::reject::custom(e))
                    }
                }
            })*

        }
    };

    (
        @gen $(
            $ns:ident, $ns_name:expr, $name:expr, $mapped_name:ident [ $(
                $json_name:expr, $field_name:ident, $field_type:ty, -> [ $($t:expr,)* ]: $t_field:ty
            )*], { mod $($mod_tokens:tt)* } { endpoints $($endpoints_tokens:tt)* }
        )*
    ) => {

        use crate::airtable::FetchCtx;
        use crate::gen_schema::Table;
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

                $($mod_tokens)*

                gen_airtable_schema!(@endpoints $ns_name; $($endpoints_tokens)*);
            }

            pub type $mapped_name = $ns::Mapped;

            impl Table for $mapped_name {
                const NAME: &'static str = $name;
                type Fields = $ns::Fields;
            }

            #[allow(unused)]
            impl $mapped_name {

                pub async fn create_one(ctx: &mut FetchCtx, one: One<$ns::Fields>) -> Result<Self,  Error> {
                    Ok(Self {
                        $($field_name: compose!(ctx, one.fields.$field_name, [ $($t),* ])?),*
                    })
                }

                pub async fn create_many(ctx: &mut FetchCtx, many: Vec<One<$ns::Fields>>) -> Result<Vec<Self>, Error> {
                    // TODO: this should not block/await on each loop,
                    // but let them all run in parallel until they're done,
                    // then we can accumulate them into the output vector.
                    let mut result = Vec::with_capacity(many.len());
                    for one in many {
                        result.push(Self::create_one(ctx, one).await?);
                    }
                    Ok(result)
                }

                pub async fn fetch_and_create_first(ctx: &mut FetchCtx, ids: Vec<String>) -> Result<Self, Error> {
                    let params: Param<Self> = Param::new_id(ids);
                    compose!(ctx, params, [ one, Self::create_one ])
                }

                pub async fn fetch_and_create_many(ctx: &mut FetchCtx, ids: Vec<String>) -> Result<Vec<Self>, Error> {
                    let params: Param<Self> = Param::new_id(ids);
                    compose!(ctx, params, [ many, Self::create_many ])
                }

            }


        )*  // end for each

        /// Generated `warp::Filter` for all endpoints created by the schema.
        pub fn route(ctx: crate::ctx::Ctx) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            use warp::Filter;
            let ctx_cache = crate::ctx::ctx_cache::route(ctx.clone());
            crate::build_route!(ctx, ctx_cache, [
                $(
                    gen_airtable_schema!(@endpoints_route $ns { $($endpoints_tokens),* })
                ),*
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
                },
                mod { $($mod_tokens:tt)* },
                endpoints { $($endpoints_tokens:tt)* };
        )*
    ) => {
        gen_airtable_schema! {
            @gen $(
                $ns, stringify!($ns), $name, $mapped_name [
                    $(
                        $k, $fn, $ft, -> [ $($tfs,)* ]: $t_ft
                     )*
                ],
                { mod $($mod_tokens)* }
                { endpoints $($endpoints_tokens)* }
            )*
        }
    }
}
