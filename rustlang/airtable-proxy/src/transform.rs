#![allow(unused)]

use crate::error::Error;

pub type MaybeBool = Option<bool>;
pub type IDs = Vec<String>;

#[macro_export(local_inner_macros)]
macro_rules! compose {

    ($c:expr, $e:expr, [ ]) => {
        Ok($e)
    };

    ($c:expr, $e:expr, [ $t:expr ]) => {
        $t($c, $e).await
    };

    ($c:expr, $e:expr, [ $t:expr, $($ts:expr),* ]) => {
        match $t($c, $e).await {
            Ok(val) => compose!($c, val, [ $($ts),* ]),
            Err(e) => Err(e)
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! pure {
    (
        @generate {
            name $fn_name:ident,
            generics $(<$T: ident $(:$T_tokens:tt)?>)?,
            mutable $($mut:ident)?,
            arg_name $arg_name:ident,
            arg_type $arg_type:ty,
            returning $ret:ty {
                $($body:tt)*
            }
        }
    ) => {
        pub async fn $fn_name $(<$T $(:$T_tokens)?>)? (
            _: &$crate::airtable::FetchCtx,
            $($mut)? $arg_name: $arg_type
        ) -> Result<$ret, $crate::error::Error> {
            $($body)*
        }
    };
    (fn $name:ident $(<$T:ident $(:$T_clause:tt)?>)? (mut $a:ident : $t:ty) -> $r:ty { $($b:tt)* }) => {
        pure!(@generate {
            name $name,
            generics $(<$T $(:$T_clause)?>)?,
            mutable mut,
            arg_name $a,
            arg_type $t,
            returning $r {
                $($b)*
            }
        });
    };
    (fn $name:ident $(<$T:ident $(:$T_clause:tt)?>)? ($a:ident : $t:ty) -> $r:ty { $($b:tt)* }) => {
        pure!(@generate {
            name $name,
            generics $(<$T $(:$T_clause)?>)?,
            mutable ,
            arg_name $a,
            arg_type $t,
            returning $r {
                $($b)*
            }
        });
    };
    ($name:ident $(<$T:ident $(:$T_clause:tt)?>)? ($a:ident : $t:ty) -> $r:ty { $($b:tt)* }) => {
        pure!(@generate {
            name $name,
            generics $(<$T $(:$T_clause)?>)?,
            mutable mut,
            arg_name $a,
            arg_type $t,
            returning $r {
                Ok($($b)*)
            }
        });
    };
}

pure!(force_bool(val: MaybeBool) -> bool { val.unwrap_or(false) });

pure!(id<T: Sized>(t: T) -> T { t });


pure!(split_lines(val: String) -> Vec<String> {
    val.split('\n').map(|s| s.to_owned()).collect()
});

pure!(into_vec<T>(value: T) -> Vec<T> { vec![value] });


pure!(fn first<T>(mut vec: Vec<T>) -> T {
    match vec.get(0) {
        None => Err(Error::Map("Cannot get the first item from an empty vec")),
        _ => Ok(vec.swap_remove(0)),
    }
});

