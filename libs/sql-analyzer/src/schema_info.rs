use postgres::NoTls;
use std::{collections::BTreeMap, error::Error};

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

#[derive(Debug, Default)]
pub enum DataType {
    CharacterVarying,
    Text,

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

    /// generic number type
    #[default]
    Numeric,
}

#[derive(Debug, Default)]
pub struct Column {
    pub ordinal_position: i32,
    pub default: Option<String>,
    pub is_nullable: bool,
    pub data_type: DataType,
    pub udt: String,
    pub dtd_identifier: Option<String>,
    pub character_maximum_length: Option<i32>,
}

impl SchemaInfo {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
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
            let ordinal_position = row.get(3);
            let default = row.get(4);
            let is_nullable: String = row.get(5);
            let data_type: String = row.get(6);
            let udt: String = row.get(7);

            let data_type = if data_type.eq_ignore_ascii_case("TINYINT") {
                DataType::TinyInt
            } else if data_type.eq_ignore_ascii_case("SMALLINT") {
                DataType::SmallInt
            } else if data_type.eq_ignore_ascii_case("integer") {
                DataType::Integer
            } else if data_type.eq_ignore_ascii_case("bigint") {
                DataType::BigInt
            } else if data_type.eq_ignore_ascii_case("numeric") {
                DataType::Numeric
            } 
            else if data_type.eq_ignore_ascii_case("real") {
                DataType::Real
            } 
            else if data_type.eq_ignore_ascii_case("double precision") {
                DataType::DoublePrecision
            } 
            else if data_type.eq_ignore_ascii_case("boolean") {
                DataType::Boolean
            } 
            else if data_type.eq_ignore_ascii_case("character varying") {
                DataType::CharacterVarying
            }
            else if data_type.eq_ignore_ascii_case("text") {
                DataType::Text
            }
             else {
                return Err(format!("{{ unknown_type: {data_type},  udt: {udt}, column: {column_name}, table: {table_name}, schema: {table_schema} }}").into());
            };

            let tables = schemas.entry(table_schema).or_default();
            let table = tables.entry(table_name).or_default();
            let column = table.entry(column_name).or_default();

            column.ordinal_position = ordinal_position;
            column.default = default;
            column.is_nullable = is_nullable.eq_ignore_ascii_case("YES");
            column.data_type = data_type;
            column.udt = udt;
            column.dtd_identifier = row.get(8);
            column.character_maximum_length = row.get(9);
        }
        // println!("{:#?}", schemas);
        Ok(Self(schemas))
    }
}
