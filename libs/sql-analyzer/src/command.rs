mod delete;
mod insert;
mod select;
mod update;

use crate::*;
use sql_parser::command::*;

impl SqlAnalyzer for Command {
    fn analyse(&self, ctx: &mut Ctx) -> AnalyseResult {
        match self {
            Command::Select(c) => c.analyse(ctx),
            Command::Insert(c) => c.analyse(ctx),
            Command::Delete(c) => c.analyse(ctx),
            Command::Update(c) => c.analyse(ctx),
        }
    }
}
