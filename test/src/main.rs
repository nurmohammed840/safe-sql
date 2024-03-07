use safe_sql::sql;

// CREATE TABLE IF NOT EXISTS "User" (
//     id SERIAL PRIMARY KEY,           // SERIAL is an auto-incrementing integer
//     username VARCHAR(50),            // VARCHAR for variable-length character strings
//     email VARCHAR(100) UNIQUE,       // UNIQUE ensures email uniqueness
//     age INTEGER,                     // INTEGER for whole numbers
//     height DECIMAL(5,2),             // DECIMAL for fixed-point numbers with precision and scale
//     weight REAL,                     // REAL for floating-point numbers
//     is_active BOOLEAN,               // BOOLEAN for true/false values
//     user_bio TEXT                    // TEXT for long strings of text
// );

fn main() {
    sql! {
        DELETE FROM User
        WHERE
            true AND (id * Cos(47 + 55))
    };
}