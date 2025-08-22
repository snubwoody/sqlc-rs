use sqlc_macros::parse_sql_str;

#[test]
fn parse_table() {
    mod db{
        use super::*;
        
        parse_sql_str!(r#"CREATE TABLE user(
        id INTEGER PRIMARY KEY,
        email TEXT NOT NULL UNIQUE
        );"#);
    }
    
    db::User{
        id: 1,
        email: String::from("")
    };
}

#[test]
fn parse_multiple_tables() {
    mod db {
        use super::*;

        parse_sql_str!(r#"
        CREATE TABLE user(
            id INTEGER PRIMARY KEY
        );
        
        CREATE TABLE group(
            id TEXT PRIMARY KEY
        );
        "#);
    }

    db::User { id: 200, };

    db::Group{ id: String::new() };

}