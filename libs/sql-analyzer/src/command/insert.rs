use crate::*;
use sql_parser::command::insert::*;

impl SqlAnalyzer for Insert {
    fn analyse(&self, ctx: &mut Ctx) -> AnalyseResult {
        if let Some(table) = ctx.get_table(&self.table_name.alias) {
            for column_name in &self.column_name {
                if let Some(_c) = ctx.get_column(table, column_name) {
                    // ...
                }
            }
        }
        Ok(())
    }
}
