mod command;
mod expr_analyzer;
mod schema_info;
mod utils;

use schema_info::{Column, SchemaInfo, Table};
use sql_parser::grammar::Name;
use std::{error::Error, sync::Once};
use syn::__private::Span;

type AnalyseResult = Result<(), Box<dyn Error>>;
type AnalyseError = (Span, String);

pub struct Ctx {
    pub info: &'static SchemaInfo,
    pub errs: Vec<AnalyseError>,
}

impl Ctx {
    pub fn add_err<T>(&mut self, r: Result<T, AnalyseError>) -> Option<T> {
        match r {
            Ok(v) => Some(v),
            Err(e) => {
                self.errs.push(e);
                None
            }
        }
    }

    pub fn get_table<'a>(&mut self, name: &Name) -> Option<&'a Table> {
        self.add_err(utils::get_table(self.info, name))
    }

    pub fn get_column<'a>(&mut self, table: &'a Table, name: &Name) -> Option<&'a Column> {
        self.add_err(utils::get_column(table, name))
    }
}

pub trait SqlAnalyzer {
    fn analyse(&self, ctx: &mut Ctx) -> AnalyseResult;
}

pub static mut SCHEMA_INFO: Option<SchemaInfo> = None;
pub static SCHEMA_INFO_INIT: Once = Once::new();

pub fn analyse_command(
    c: sql_parser::command::Command,
) -> Result<Vec<(Span, String)>, Box<dyn Error>> {
    SCHEMA_INFO_INIT.call_once(|| {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            if let Ok(info) = SchemaInfo::new(&url) {
                unsafe {
                    let _ = SCHEMA_INFO.insert(info);
                }
            }
        }
    });
    unsafe {
        match &SCHEMA_INFO {
            Some(info) => {
                let mut ctx = Ctx { info, errs: vec![] };
                c.analyse(&mut ctx)?;
                Ok(ctx.errs)
            }
            None => Ok(vec![]),
        }
    }
}
