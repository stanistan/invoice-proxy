pub trait Table {
    const NAME: &'static str;
    const MODULE_NAME: &'static str;
    type Fields: serde::de::DeserializeOwned;
}

#[macro_export(local_inner_macros)]
macro_rules! __gen_inner {

    (
        @main $($name:ident ($table:expr) -> $out:ident { $($inner:tt)* })*
    ) => {
        pub mod gen {
            use super::*;
            use $crate::airtable::FetchCtx;
            use $crate::compose;
            use $crate::error::Error;
            use $crate::gen_schema::Table;
            use $crate::network::request::*;
            use $crate::network::response::One;
            use $crate::transform::*;

            use $crate::warp;

            // TODO: comment/splanations
            $(__gen_inner!{@table $name, std::stringify!($name), ($table) -> $out { $($inner)* }})*

            /// Generated `warp::Filter` for all endpoints created by the schema.
            pub fn route(ctx: $crate::ctx::Ctx) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
                use warp::Filter;
                let ctx_cache = $crate::ctx::ctx_cache::route(ctx.clone());
                build_route!(ctx, ctx_cache, [ $( $name::endpoints::route ),* ])
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
            $(__gen_inner!{@fields $mod_str_name, $table, [ $($fields)* ]})?

            // insert any module that's been done there, inlined
            $($($module)*)?

            pub mod param {
                #![allow(unused)]
                use super::*;
                pure!(as_id_query(id: String) -> Param<Mapped> {
                    Param::new_query("ID".to_string(), id)
                });
            }

            // generate an endpoints module
            pub mod endpoints {
                #![allow(unused)]
                use super::*;
                use $crate::ctx::Ctx;
                use $crate::warp;
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
            use $crate::ctx::with_ctx;

            use $crate::warp;
            use $crate::warp::{Filter, Rejection, Reply};

            pub fn route(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
                warp::path($mod_str_name)
                    .and(warp::path::param::<$from>())
                    .and(warp::get())
                    .and(with_ctx(ctx))
                    .and_then(run)
            }

            async fn handler(ctx: &mut FetchCtx, arg: $from) -> Result<$to, Error> {
                trace!("exec [{}] with arg={}", std::stringify!( $($($exec),*)? ), arg);
                compose!(ctx, arg, [ $($($exec),*)? ])
            }

            pub async fn run(arg: $from, ctx: Ctx) -> Result<impl Reply, Rejection> {
                let mut c = ctx.lock().await;
                match handler(&mut c, arg).await {
                    Ok(val) => Ok(warp::reply::json(&val)),
                    Err(e) => Err(warp::reject::custom(e))
                }
            }

        })*

        pub fn route(ctx: Ctx) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            use warp::Filter;
            let default_route = warp::path($mod_str_name)
                .and(warp::get())
                .map(|| std::format!("fine.... {}", $mod_str_name));

            build_route!(ctx, [ $($name::route),* ], default_route)
        }
    };
    //
    // Macro generates the `Fields` type, the `Mapped` type,
    // which correspond respectively, to the JSON shape that comes back
    // and we want to parse for the module, as well as the more complex
    // mapping of the fully hydrated type that will be constructed.
    (
        @fields $mod_str_name:expr, $table:expr, [
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
            const MODULE_NAME: &'static str = $mod_str_name;
            type Fields = Fields;
        }

        impl Mapped {

            pub async fn create_one(ctx: &mut FetchCtx, one: One<Fields>) -> Result<Self, Error> {
                Ok(Self {
                    $(
                        $name: match compose!(ctx, one.fields.$name, [ $($($exec),*)? ]) {
                            Ok(val) => val,
                            Err(e) => return Err(Error::Create {
                                table: $table,
                                source: Box::new(e),
                            })
                        }
                     ),*
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
macro_rules! gen_airtable_schema {
    ($($tt:tt)*) => {
        __gen_inner!{@main $($tt)*}
    }
}

#[macro_export(local_inner_macros)]
macro_rules! build_route {
    ($ctx:expr, [ $name:expr ], $($default:expr)?) => {
        $name($ctx.clone())
    };
    ($ctx:expr, [ $name:expr, $($ns:expr),+ ], $($default:expr)?) => {
        build_route!($ctx, $name($ctx.clone()), [ $($ns),+ ])
    };
    ($ctx:expr, $r:expr, [ ]) => {
        $r
    };
    ($ctx:expr, [ ], $default:expr) => {
        $default
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
