#![allow(non_camel_case_types)]
use crate::*;
use grammar::ast::{Arithmetic, OrExpr};
use utils::Many;

macro_rules! parse_arg {
    [$name: ident, $i: ident,] => {
        Self::$name()
    };
    [$name: ident, $i: ident, $_1: ty] => {
        Self::$name($i.parse()?)
    };
    [$name: ident, $i: ident, $_1: ty, $_2: ty] => {
        {
            let a = $i.parse()?;
            let _ = $i.parse::<Token![,]>()?;
            let b = $i.parse()?;
            Self::$name(a, b)
        }
    }
}

macro_rules! define_function {
    [
        $ty: ident {
            $(  $name: ident   $(| $alies: ident)*  (  $($arg: tt)*   )  ),*
        }
    ] => {
        pub const SQL_FUNC_NAMES: &[&str] = &[$(stringify!($name)),* ];
        // #[derive(Debug)]
        pub enum $ty { $($name ($($arg)*)),* }
        impl Parse for $ty {
            fn parse(input: ParseStream) -> Result<Self> {
                let fn_name = input.parse::<Ident>()?.to_string();
                let i;
                parenthesized!(i in input);
                
                Ok(match fn_name.to_uppercase().as_str() {
                    $(stringify!($name) $(| stringify!($alies))* => parse_arg!($name, i, $($arg)*),)*
                    kw => return Err(input.error(format!("unknown function: `{fn_name}` \nhint: {}", utils::suggest(kw, SQL_FUNC_NAMES.iter())))),
                })
            }
        }
    };
}

define_function! {
    Function {
        // # Numeric Function
        ABS(Arithmetic),
        ACOS(Arithmetic),
        ASIN(Arithmetic),
        ATAN(Arithmetic),
        COS(Arithmetic),
        COSH(Arithmetic),
        COT(Arithmetic),
        SIN(Arithmetic),
        SINH(Arithmetic),
        TAN(Arithmetic),
        TANH(Arithmetic),

        ATAN2(Arithmetic, Arithmetic),

        BITAND(OrExpr, OrExpr),
        BITOR(OrExpr, OrExpr),
        BITXOR(OrExpr, OrExpr),
        BITNOT(OrExpr),
        BITNAND(OrExpr, OrExpr),
        BITNOR(OrExpr, OrExpr),
        BITXNOR(OrExpr, OrExpr),
        BITGET(OrExpr, LitInt),
        BITCOUNT(OrExpr),

        LSHIFT(OrExpr, LitInt),
        RSHIFT(OrExpr, LitInt),
        ULSHIFT(OrExpr, LitInt),
        URSHIFT(OrExpr, LitInt),

        ROTATELEFT(OrExpr, LitInt),
        ROTATERIGHT(OrExpr, LitInt),

        MOD(Arithmetic, Arithmetic),

        CEIL | CEILING(Arithmetic),
        DEGREES(Arithmetic),
        EXP(Arithmetic),
        FLOOR(Arithmetic),
        LN(Arithmetic),

        LOG(Arithmetic, Arithmetic),
        LOG10(Arithmetic),

        ORA_HASH(OrExpr),
        RADIANS(LitInt),
        SQRT(Arithmetic),
        PI(),
        POWER(Arithmetic, Arithmetic),
        RAND | RANDOM(Arithmetic),
        RANDOM_UUID | UUID(),
        ROUND(Arithmetic),
        SECURE_RAND(LitInt),
        SIGN(Arithmetic),

        // ENCRYPT(..),
        // DECRYPT(..),
        // HASH(..),
        // TRUNC | TRUNCATE(..),
        // COMPRESS(..),
        // EXPAND(..),
        // ZERO(..)

        // # String Function

        ASCII(LitStr),
        // BIT_LENGTH(),
        CHAR_LENGTH | CHARACTER_LENGTH | LENGTH(LitStr),
        // OCTET_LENGTH(),
        CHAR | CHR(LitInt),
        CONCAT(Many<LitStr>),

        // CONCAT_WS(),
        DIFFERENCE(LitStr, LitStr),
        HEXTORAW(LitStr),
        // RAWTOHEX(),
        // INSERT(),
        LOWER | LCASE(LitStr),
        UPPER | UCASE(LitStr),
        LEFT(LitStr, LitInt),
        RIGHT(LitStr, LitInt),
        // LOCATE(),
        // LPAD(),
        // RPAD(),
        // LTRIM(),
        // RTRIM(),
        // BTRIM(),
        // TRIM(),
        // REGEXP_REPLACE(),
        // REGEXP_LIKE(),
        // REGEXP_SUBSTR(),
        REPEAT(LitStr, LitInt),
        // REPLACE(),
        SOUNDEX(LitStr),
        SPACE(LitInt)
        // STRINGDECODE(),
        // STRINGENCODE(),
        // STRINGTOUTF8(),
        // SUBSTRING(),
        // UTF8TOSTRING(),
        // QUOTE_IDENT(),
        // XMLATTR(),
        // XMLNODE(),
        // XMLCOMMENT(),
        // XMLCDATA(),
        // XMLSTARTDOC(),
        // XMLTEXT(),
        // TO_CHAR(),
        // TRANSLATE()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_name() {
//         let g: Function = utils::test::syntex! {
//             add(6 + 5)
//         }
//         .unwrap();
//         println!("{:#?}", g);
//     }
// }
