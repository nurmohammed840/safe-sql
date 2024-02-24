// #![allow(warnings)]
// use std::error::Error;
// use tokio_postgres::NoTls;

// const URL: &str = "postgresql://postgres:@localhost:5432/test";

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let (client, conn) = tokio_postgres::connect(URL, NoTls).await?;
//     tokio::spawn(async { println!("{:#?}", conn.await) });

//     let rows = client
//         .query(
//             "
//         SELECT
//             table_schema,
//             table_name,
//             column_name,
//             ordinal_position,
//             column_default,
//             is_nullable,
//             data_type,
//             udt_name,
//             dtd_identifier,
//             character_maximum_length
//         FROM
//             information_schema.columns
//         WHERE
//             table_schema = 'public';
//     ",
//             &[],
//         )
//         .await?;

//     let columns: Vec<ColumnInformation> = rows
//         .iter()
//         .map(|row| ColumnInformation {
//             table_schema: row.get(0),
//             table_name: row.get(1),
//             column_name: row.get(2),
//             ordinal_position: row.get(3),
//             column_default: row.get(4),
//             is_nullable: row.get(5),
//             data_type: row.get(6),
//             udt_name: row.get(7),
//             dtd_identifier: row.get(8),
//             character_maximum_length: row.get(9),
//         })
//         .collect();

//     println!("{:#?}", columns);
//     Ok(())
// }

// #[derive(Debug)]
// struct ColumnInformation {
//     table_schema: String,
//     table_name: String,
//     column_name: String,
//     ordinal_position: i32,
//     column_default: Option<String>,
//     is_nullable: String,
//     data_type: String,
//     udt_name: String,
//     dtd_identifier: Option<String>,
//     character_maximum_length: Option<i32>,
// }

// // struct Data {
// //     table_schema: String,  // public
// //     table_name: String,    // my_collection
// //     column_name: String,   // id, name, age, email, ...
// //     ordinal_position: u32, // 1, 2,  3, 4

// //     /// The column_default column contains the default value for the column. This default value is the expression or literal that PostgreSQL will automatically assign to the column if no explicit value is provided during an INSERT operation where the column is not specified.
// //     /// For example, if a column named age has a default value of 30, then whenever a new row is inserted into the table and no value for age is specified, PostgreSQL will automatically set age to 30.
// //     column_default: String, // null, 0,
// //     is_nullable: bool,
// //     data_type: String,
// //     udt_name: String,

// //     /// Represents the data type identifier. This column provides a unique identifier for each distinct data type used in the database schema.
// //     /// The data type identifier (dtd_identifier) is particularly useful when you need to differentiate between different data types, especially when dealing with user-defined types or special types specific to PostgreSQL.
// //     dtd_identifier: u32,

// //     // exest if `data_type` is character varying;
// //     character_maximum_length: Option<u32>,
// // }
fn main() {}
