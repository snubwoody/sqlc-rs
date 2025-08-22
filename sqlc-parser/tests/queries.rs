use sqlparser::ast::{CreateTable, Statement};
use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;
use sqlc_parser::{Table};


#[test]
fn query(){
    let sql = r#"
        --name: get_users: many
        /*name: get_users: many */
        SELECT * FROM users;
    "#;
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    dbg!(&ast);
}

