use sqlparser::ast::{CreateTable, Statement};
use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;
use sqlc_parser::{ColumnType, Table};

#[test]
fn table_name(){
    let sql = "CREATE TABLE images();";
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    let query: &CreateTable;
    match &ast[0]{
        Statement::CreateTable(_query) => {
            query = _query;
        },
        _ => panic!("Not a CreateTable")
    }
    
    let table = Table::parse_query(query);
    assert_eq!(table.name(),"images")
}

#[test]
fn integer_fields(){
    let sql = r#"
        CREATE TABLE accounts(
            balance INT,
            pending BIGINT,
            short_balance SMALLINT
        );
    "#;
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    let query: &CreateTable;
    match &ast[0]{
        Statement::CreateTable(_query) => {
            query = _query;
        },
        _ => panic!("Not a CreateTable")
    }
    
    let table = Table::parse_query(query);
    let balance = table.columns().get("balance").unwrap();
    let short_balance = table.columns().get("short_balance").unwrap();
    let pending = table.columns().get("pending").unwrap();
    assert_eq!(balance,&ColumnType::Integer);
    assert_eq!(pending,&ColumnType::BigInt);
    assert_eq!(short_balance,&ColumnType::SmallInt);
}

#[test]
fn integer_fields_long_names(){
    let sql = r#"
        CREATE TABLE accounts(
            balance INTEGER
        );
    "#;
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    let query: &CreateTable;
    match &ast[0]{
        Statement::CreateTable(_query) => {
            query = _query;
        },
        _ => panic!("Not a CreateTable")
    }
    
    let table = Table::parse_query(query);
    let balance = table.columns().get("balance").unwrap();
    assert_eq!(balance,&ColumnType::Integer);
}

#[test]
fn unique_constraint(){
    let sql = r#"
        CREATE TABLE accounts(
            id SERIAL PRIMARY KEY,
        );
    "#;
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    let query: &CreateTable;
    match &ast[0]{
        Statement::CreateTable(_query) => {
            query = _query;
        },
        _ => panic!("Not a CreateTable")
    }
    
    let table = Table::parse_query(query);
    let balance = table.columns().get("balance").unwrap();
    assert_eq!(balance,&ColumnType::Integer);
}