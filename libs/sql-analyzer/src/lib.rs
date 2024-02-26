mod command;
mod expr_analyzer;
mod schema_info;
mod utils;

use schema_info::{Column, SchemaInfo, Table};
use sql_parser::grammar::Name;
use std::{env, error::Error};
use syn::__private::Span;

type AnalyseResult = Result<(), Box<dyn Error>>;
type AnalyseError = (Span, String);

pub struct Ctx<'s> {
    pub info: &'s SchemaInfo,
    pub errs: Vec<AnalyseError>,
}

impl<'s> Ctx<'s> {
    pub fn add_err<T>(&mut self, r: Result<T, AnalyseError>) -> Option<T> {
        match r {
            Ok(v) => Some(v),
            Err(e) => {
                self.errs.push(e);
                None
            }
        }
    }

    pub fn get_table(&mut self, name: &Name) -> Option<&'s Table> {
        self.add_err(utils::get_table(self.info, name))
    }

    pub fn get_column(&mut self, table: &'s Table, name: &Name) -> Option<&'s Column> {
        self.add_err(utils::get_column(table, name))
    }
}

pub trait SqlAnalyzer {
    fn analyse(&self, ctx: &mut Ctx) -> AnalyseResult;
}

thread_local! {
    static SCHEMA_INFO: Option<SchemaInfo> = {
        let path = env::var("CARGO_TARGET_DIR").map(|dir| dir + "/target/safe-sql.log").unwrap_or_else(|_| "./safe-sql.log".to_string());
        let logger = utils::new_logger(path);
        let _ = log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(log::LevelFilter::Info));

        match std::env::var("DATABASE_URL") {
            Ok(url) => match SchemaInfo::new(&url) {
                Ok(info) => Some(info),
                Err(err) => {
                    log::error!("{err}");
                    None
                }
            },
            Err(err) => {
                log::warn!("`DATABASE_URL` {err}");
                None
            },
        }
    };
}

pub fn analyse_command(c: sql_parser::command::Command) -> Vec<(Span, String)> {
    let mut errs = vec![];
    SCHEMA_INFO.with(|v| {
        if let Some(info) = v {
            let mut ctx = Ctx { info, errs: vec![] };
            if let Ok(_) = c.analyse(&mut ctx) {
                errs = ctx.errs;
            }
        }
    });
    errs
}
