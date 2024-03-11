use self::{
    grammar::ast::OrExpr,
    utils::{parse_keywords_if_matched, parse_kw_if_matched},
};
use crate::*;
use grammar::Name;
use syn::token;
use utils::SeparatedByComma;

pub struct Window {
    pub over_kw: Ident,
    pub name_or_spec: NameOrSpec,
}

pub enum NameOrSpec {
    Name(Name),
    Spec {
        paren_token: token::Paren,
        existing_window_name: Option<Name>,
        partition_by: Option<PartitionByClause>,
        order_by: Option<OrderByClause>,
    },
}

pub struct PartitionByClause {
    pub partition_by_kws: (Ident, Ident),
    pub exprs: SeparatedByComma<OrExpr>,
}

pub struct OrderByClause {
    pub order_by_kws: (Ident, Ident),
    pub sort_specs: SeparatedByComma<SortSpec>,
}

pub struct SortSpec {
    pub expr: OrExpr,
    /// [ASC | DESC]
    pub order: Option<Ident>,
    /// [NULLS { FIRST | LAST }]
    pub nulls_order: Option<(Ident, Ident)>,
}

pub struct FrameClause {}

impl Parse for FrameClause {
    fn parse(input: ParseStream) -> Result<Self> {
        let _ = parse_keywords_if_matched(input, &["ROWS", "RANGE", "GROUP"]);
        if let Ok(_exclude_kw) = parse_kw_if_matched(input, "EXCLUDE") {
            
        }
        Ok(Self {})
    }
}

impl Parse for NameOrSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(token::Paren) {
            let content;
            let paren_token = parenthesized!(content in input);

            let fork = content.fork();
            if let Ok(name) = fork.parse::<Name>() {
                if let Name::Ident(c) = name {
                    let c = c.to_string();
                    for kw in ["PARTITION", "ORDER"] {
                        if c.eq_ignore_ascii_case(kw) {}
                    }
                }
            }

            Ok(Self::Spec {
                paren_token,
                existing_window_name: parse_existing_window_name(&content),
                partition_by: match parse_kw_if_matched(&content, "PARTITION") {
                    Ok(ks) => Some(PartitionByClause {
                        partition_by_kws: (ks, parse_kw_if_matched(&content, "BY")?),
                        exprs: content.parse()?,
                    }),
                    Err(_) => None,
                },
                order_by: match parse_kw_if_matched(&content, "ORDER") {
                    Ok(ks) => Some(OrderByClause {
                        order_by_kws: (ks, parse_kw_if_matched(&content, "BY")?),
                        sort_specs: content.parse()?,
                    }),
                    Err(_) => None,
                },
            })
        } else {
            input.parse().map(Self::Name)
        }
    }
}

fn parse_existing_window_name(input: ParseStream) -> Option<Name> {
    let fork = input.fork();
    let name = fork.parse::<Name>().ok()?;
    if let Name::Ident(c) = &name {
        let c = c.to_string();
        for kw in ["PARTITION", "ORDER"] {
            if c.eq_ignore_ascii_case(kw) {
                return None;
            }
        }
    }
    input.advance_to(&fork);
    Some(name)
}

impl Parse for SortSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            order: parse_keywords_if_matched(input, &["ASC", "DESC"]).ok(),
            nulls_order: match parse_kw_if_matched(input, "NULLS") {
                Ok(kw) => Some((kw, parse_keywords_if_matched(input, &["FIRST", "LAST"])?)),
                Err(_) => None,
            },
        })
    }
}
