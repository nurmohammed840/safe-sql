use std::{
    fs::File,
    io::{self, Write},
};

use crate::{
    schema_info::{Column, SchemaInfo, Table},
    AnalyseError,
};
use log::{Level, Metadata, Record};
use sql_parser::{grammar::Name, utils::suggest};

pub fn get_table<'a>(info: &'a SchemaInfo, name: &Name) -> Result<&'a Table, AnalyseError> {
    let (name, span) = (name.to_string(), name.span());

    let tables = info
        .get_public_tables()
        .ok_or_else(|| (span, "no table found".to_string()))?;

    tables.get(&name).ok_or_else(|| {
        (
            span,
            format!(
                "table does not exist: `{name}`nsuggest: {}",
                suggest(&name, tables.keys())
            ),
        )
    })
}

pub fn get_column<'a>(table: &'a Table, name: &Name) -> Result<&'a Column, AnalyseError> {
    let (name, span) = (name.to_string(), name.span());
    table.get(&name).ok_or_else(|| {
        (
            span,
            format!(
                "column does not exist: `{name}` \nsuggest: {}",
                suggest(&name, table.keys())
            ),
        )
    })
}

pub struct Logger<W> {
    output: W,
}

pub enum LoggerOutput {
    Console(io::Stdout),
    File(File),
}

pub fn new_logger(path: impl AsRef<std::path::Path>) -> Logger<LoggerOutput> {
    Logger {
        output: match File::options().append(true).create(true).open(path) {
            Ok(file) => LoggerOutput::File(file),
            Err(_) => LoggerOutput::Console(io::stdout()),
        },
    }
}

impl log::Log for Logger<LoggerOutput> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = match &self.output {
                LoggerOutput::Console(stdout) => {
                    let mut o = stdout;
                    o.write_all(format!("[{}] - {}\n", record.level(), record.args()).as_bytes())
                }
                LoggerOutput::File(file) => {
                    let mut o = file;
                    o.write_all(format!("[{}] - {}\n", record.level(), record.args()).as_bytes())
                }
            };
        }
    }
    fn flush(&self) {}
}
