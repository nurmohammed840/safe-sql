#![allow(non_camel_case_types)]
use crate::*;
use grammar::ast::OrExpr;

macro_rules! define_function {
    [
        $(
            $ty: ident {
                $($name: ident ( $($arg: ty),* )),*
            }
        )*
    ] => {
        $(pub enum $ty { $($name ($($arg),*)),* }
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
                        $(stringify!($name) => Self::$name($(i.parse::<$arg>()?),*),)*
                        _ => return Err(c.error("unknown function")),
                    });
                    Ok((parse.parse2(stream)?, rest))
                })
            }
        })*
    };
}

define_function! {
    NumericFunction {
        ABS(LitFloat),
        ACOS(LitFloat),
        ASIN(LitFloat),
        ATAN(LitFloat),
        COS(LitFloat),
        COSH(LitFloat),
        COT(LitFloat),
        SIN(LitFloat),
        SINH(LitFloat),
        TAN(LitFloat),
        TANH(LitFloat),

        ATAN2(LitFloat, LitFloat),

        BITAND(OrExpr, OrExpr),
        BITOR(OrExpr, OrExpr),
        BITXOR(OrExpr, OrExpr),
        BITNOT(OrExpr),
        BITNAND(OrExpr, OrExpr),
        BITNOR(OrExpr, OrExpr),
        BITXNOR(OrExpr, OrExpr),
        BITGET(OrExpr, LitFloat), //
        BITCOUNT(OrExpr),

        LSHIFT(OrExpr, LitFloat),
        RSHIFT(OrExpr, LitFloat),
        ULSHIFT(OrExpr, LitFloat),
        URSHIFT(OrExpr, LitFloat),

        ROTATELEFT(OrExpr, LitFloat),
        ROTATERIGHT(OrExpr, LitFloat),

        MOD(LitFloat, LitFloat),

        CEIL(LitFloat),
        DEGREES(LitFloat),
        EXP(LitFloat),
        FLOOR(LitFloat),
        LN(LitFloat),

        LOG(LitFloat, LitFloat),
        LOG10(LitFloat),

        ORA_HASH(OrExpr),
        RADIANS(LitFloat),
        SQRT(LitFloat),
        PI(),
        POWER(LitFloat, LitFloat),
        RAND(LitFloat),
        RANDOM_UUID(),
        ROUND(LitFloat),
        SECURE_RAND(LitFloat),
        SIGN(LitFloat),

        ENCRYPT(),
        DECRYPT(),
        HASH(),
        TRUNC(),
        COMPRESS(),
        EXPAND(),
        ZERO()
    }
    StringFunction {
        ASCII(LitStr),
        BIT_LENGTH(),
        CHAR_LENGTH(),
        OCTET_LENGTH(),
        CHAR(),
        CONCAT(),
        CONCAT_WS(),
        DIFFERENCE(),
        HEXTORAW(),
        RAWTOHEX(),
        INSERT(),
        LOWER(),
        UPPER(),
        LEFT(),
        RIGHT(),
        LOCATE(),
        LPAD(),
        RPAD(),
        LTRIM(),
        RTRIM(),
        BTRIM(),
        TRIM(),
        REGEXP_REPLACE(),
        REGEXP_LIKE(),
        REGEXP_SUBSTR(),
        REPEAT(),
        REPLACE(),
        SOUNDEX(),
        SPACE(),
        STRINGDECODE(),
        STRINGENCODE(),
        STRINGTOUTF8(),
        SUBSTRING(),
        UTF8TOSTRING(),
        QUOTE_IDENT(),
        XMLATTR(),
        XMLNODE(),
        XMLCOMMENT(),
        XMLCDATA(),
        XMLSTARTDOC(),
        XMLTEXT(),
        TO_CHAR(),
        TRANSLATE()
    }
}
