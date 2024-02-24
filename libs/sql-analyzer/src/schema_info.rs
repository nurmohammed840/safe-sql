use postgres::{Error, NoTls};
use std::collections::BTreeMap;

pub type Table = BTreeMap<String, Column>;
pub type Tables = BTreeMap<String, Table>;
pub type Schema = BTreeMap<String, Tables>;

#[derive(Debug)]
pub struct SchemaInfo(Schema);

impl SchemaInfo {
    pub fn get_tables(&self, schema_name: impl AsRef<str>) -> Option<&Tables> {
        self.0.get(schema_name.as_ref())
    }

    pub fn get_public_tables(&self) -> Option<&Tables> {
        self.0.get("public").or_else(|| {
            if self.0.len() == 1 {
                self.0.values().next()
            } else {
                None
            }
        })
    }
}

pub enum DataType {
    /// TRUE, FALSE, UNKNOWN (NULL)
    Boolean,
    /// `i8`
    TinyInt,
    /// `i16`
    SmallInt,
    /// `i32`
    Integer,
    /// `i64`
    BigInt,
    /// `f32`
    Real,
    /// `f64`
    DoublePrecision,
    // NUMERIC,
    Unknown,
}

#[derive(Debug, Default)]
pub struct Column {
    pub ordinal_position: i32,
    pub default: Option<String>,
    pub is_nullable: bool,
    pub data_type: String,
    pub udt_name: String,
    pub dtd_identifier: Option<String>,
    pub character_maximum_length: Option<i32>,
}

impl SchemaInfo {
    pub fn new(url: &str) -> Result<Self, Error> {
        let mut client = postgres::Client::connect(url, NoTls)?;
        let rows = client.query(
            "SELECT 
                        table_schema,
                        table_name,
                        column_name,
                        ordinal_position,
                        column_default,
                        is_nullable,
                        data_type,
                        udt_name,
                        dtd_identifier,
                        character_maximum_length
                    FROM
                        information_schema.columns
                    WHERE
                        table_schema = 'public';",
            &[],
        )?;

        let mut schemas: Schema = Default::default();

        for row in rows {
            let table_schema = row.get(0);
            let table_name = row.get(1);
            let column_name = row.get(2);

            let tables = schemas.entry(table_schema).or_default();
            let table = tables.entry(table_name).or_default();
            let column = table.entry(column_name).or_default();

            column.ordinal_position = row.get(3);
            column.default = row.get(4);

            let is_nullable: String = row.get(5);
            column.is_nullable = is_nullable.eq_ignore_ascii_case("YES");

            column.data_type = row.get(6);
            column.udt_name = row.get(7);
            column.dtd_identifier = row.get(8);
            column.character_maximum_length = row.get(9);
        }
        println!("{:#?}", schemas);
        Ok(Self(schemas))
    }
}
