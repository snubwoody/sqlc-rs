use std::collections::HashMap;
use sqlparser::ast::{CreateTable, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

// TODO: Maybe use hashmap for fields
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
struct Table{
    name: String,
    columns: Vec<ColumnType>,
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
enum ColumnType{
    Serial,
    Int,
    Uuid,
    Text,
    Custom(String)
}

#[derive(Debug,Clone,PartialEq,PartialOrd,)]
struct Column{
    name: String,
    field_type: ColumnType,
    is_unique: bool,
    is_primary: bool,
    nullable: bool
}

fn parse_sql(){
    let sql = include_str!("../../schema.sql");
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    for statement in ast{
        match statement{
            Statement::CreateTable(table) => {
                parse_table(&table);
            },
            _ => {}
        }
    }
}

fn parse_table(query: &CreateTable){
    let mut columns:HashMap<String,String> = HashMap::new();
    for col in &query.columns{
        dbg!(col);
        columns.insert(col.name.to_string(), "TODO".to_string());
    }
    let table = Table{
        name: query.name.to_string(),
        columns: vec![]
    };

    dbg!(table);
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn sql(){
        parse_sql();
    }
}