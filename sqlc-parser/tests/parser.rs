use sqlparser::ast::{CreateTable, Statement};
use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;
use sqlc_parser::{parse_schema, ColumnType, Table};

#[test]
fn parse_multiple_tables(){
    let sql = "CREATE TABLE images(); CREATE TABLE users()";
    let tables = parse_schema(sql);
    assert_eq!(tables.len(), 2)
}

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
    let balance = table.get_column("balance").unwrap();
    let short_balance = table.get_column("short_balance").unwrap();
    let pending = table.get_column("pending").unwrap();
    assert_eq!(balance.field_type,ColumnType::Integer);
    assert_eq!(pending.field_type,ColumnType::BigInt);
    assert_eq!(short_balance.field_type,ColumnType::SmallInt);
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
    let balance = table.get_column("balance").unwrap();
    assert_eq!(balance.field_type,ColumnType::Integer);
}

#[test]
fn unique_constraint(){
    let sql = r#"
        CREATE TABLE accounts(
            id TEXT UNIQUE
        );
    "#;
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    let query: &CreateTable;
    match &ast[0]{
        Statement::CreateTable(_query) => {
            dbg!(_query);
            query = _query;
        },
        _ => panic!("Not a CreateTable")
    }
    
    let table = Table::parse_query(query);
    let id = table.get_column("id").unwrap();
    assert!(id.is_unique);
}

#[test]
fn default_to_null(){
    let sql = r#"
        CREATE TABLE accounts(
            id TEXT UNIQUE
        );
    "#;
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    let query: &CreateTable;
    match &ast[0]{
        Statement::CreateTable(_query) => {
            dbg!(_query);
            query = _query;
        },
        _ => panic!("Not a CreateTable")
    }
    
    let table = Table::parse_query(query);
    let id = table.get_column("id").unwrap();
    assert!(id.nullable);
}