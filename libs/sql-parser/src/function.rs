#![allow(non_camel_case_types)]
use crate::*;
use grammar::ast::{Factor, OrExpr};
use utils::Many;

macro_rules! define_function {
    [
        $ty: ident {
            $(  $name: ident   $(| $alies: ident)*  (  $($arg: ty),*   )  ),*
        }
    ] => {
        pub enum $ty { $($name ($($arg),*)),* }
        impl Parse for $ty {
            fn parse(input: ParseStream) -> Result<Self> {
                input.step(|c| {
                    let (name, rest) = c.ident().ok_or(c.error("expated math function"))?;
                    let (tt, rest) = rest.token_tree().ok_or(c.error("unexpated eof"))?;
                    let TokenTree::Group(group) = tt else { return Err(c.error("exprcted `(..)`")); };
                    if group.delimiter() != Delimiter::Parenthesis {
                        return Err(c.error("exprcted `(..)`"));
                    }
                    let stream = group.stream();
                    let parse = |i: ParseStream| Ok(match name.to_string().to_uppercase().as_str() {
                        $(stringify!($name) $(| stringify!($alies))* => Self::$name($(i.parse::<$arg>()?),*),)*
                        _ => return Err(c.error("unknown function")),
                    });
                    Ok((parse.parse2(stream)?, rest))
                })
            }
        }
    };
}

define_function! {
    Function {
        // # Numeric Function
        ABS(Factor),
        ACOS(Factor),
        ASIN(Factor),
        ATAN(Factor),
        COS(Factor),
        COSH(Factor),
        COT(Factor),
        SIN(Factor),
        SINH(Factor),
        TAN(Factor),
        TANH(Factor),

        ATAN2(Factor, Factor),

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

        MOD(Factor, Factor),

        CEIL | CEILING(Factor),
        DEGREES(Factor),
        EXP(Factor),
        FLOOR(Factor),
        LN(Factor),

        LOG(Factor, Factor),
        LOG10(Factor),

        ORA_HASH(OrExpr),
        RADIANS(LitInt),
        SQRT(Factor),
        PI(),
        POWER(Factor, Factor),
        RAND | RANDOM(Factor),
        RANDOM_UUID | UUID(),
        ROUND(Factor),
        SECURE_RAND(LitInt),
        SIGN(Factor),

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
