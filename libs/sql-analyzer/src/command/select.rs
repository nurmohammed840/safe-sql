use crate::*;
use sql_parser::command::select::*;

impl SqlAnalyzer for Select {
    fn analyse(&self, _: &mut Ctx) -> AnalyseResult {
        Ok(())
    }
}
