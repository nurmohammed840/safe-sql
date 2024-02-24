use crate::*;
use sql_parser::command::update::*;

impl SqlAnalyzer for Update {
    fn analyse(&self, _: &mut Ctx) -> AnalyseResult {
        Ok(())
    }
}
