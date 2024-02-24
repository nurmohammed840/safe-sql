use crate::{utils::levenshtein_distance, *};

pub mod delete;
pub mod insert;
pub mod select;
pub mod update;

use delete::Delete;
use insert::Insert;
use select::Select;
use update::Update;

const SUGGEST_CMD_KW: [&str; 4] = ["SELECT", "INSERT", "DELETE", "UPDATE"];

pub enum Command {
    Select(Select),
    Insert(Insert),
    Delete(Delete),
    Update(Update),
}

impl Parse for Command {
    fn parse(input: ParseStream) -> Result<Self> {
        let err_msg = format!("expected keyword: {}", SUGGEST_CMD_KW.join(" | "));
        let (keyword, _) = input.cursor().ident().ok_or(input.error(&err_msg))?;
        Ok(match keyword.to_string().to_uppercase().as_str() {
            "SELECT" => Self::Select(input.parse()?),
            "INSERT" => Self::Insert(input.parse()?),
            "DELETE" => Self::Delete(input.parse()?),
            "UPDATE" => Self::Update(input.parse()?),
            kw => {
                let mut kws =
                    SUGGEST_CMD_KW.map(|expected| (levenshtein_distance(kw, expected), expected));

                kws.sort();
                return Err(Error::new(
                    keyword.span(),
                    if kws[0].0 <= 2 {
                        format!("did you mean: {}", kws[0].1)
                    } else {
                        err_msg
                    },
                ));
            }
        })
    }
}
