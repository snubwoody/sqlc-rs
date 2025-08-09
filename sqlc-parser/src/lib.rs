use std::collections::HashMap;
use sqlparser::ast::{CreateTable, DataType, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Table{
    name: String,
    // TODO: switch to column
    columns: HashMap<String,ColumnType>,
}

impl Table{
    /// Get the name of the table.
    pub fn name(&self) -> &str{
        self.name.as_str()
    }

    /// Get the columns of the table.
    pub fn columns(&self) -> &HashMap<String,ColumnType>{
        &self.columns
    }

    pub fn parse_query(query: &CreateTable) -> Self{
        let mut columns:HashMap<String,ColumnType> = HashMap::new();
        for col in &query.columns{
            let data_type = ColumnType::from(col.data_type.clone());
            columns.insert(col.name.to_string(), data_type);
        }

        Self {
            name: query.name.to_string(),
            columns
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum ColumnType{
    Serial,
    Uuid,
    /// 4 byte integer
    Integer,
    /// 8 byte integer
    BigInt,
    /// 2 byte integer
    SmallInt,
    Boolean,
    /// Any text type
    Text,
    Custom(String)
}

impl From<DataType> for ColumnType {
    fn from(dt:DataType) -> Self {
        match dt{
            DataType::Int(_) | DataType::Integer(_) => ColumnType::Integer,
            DataType::BigInt(_) => ColumnType::BigInt,
            DataType::SmallInt(_) => ColumnType::SmallInt,
            DataType::Boolean => ColumnType::Boolean,
            DataType::Text |
            DataType::Varchar(_) |
            DataType::Character(_) => ColumnType::Text,
            _ => ColumnType::Custom(format!("{}", dt))
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd,)]
pub struct Column{
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
    let mut columns:HashMap<String,ColumnType> = HashMap::new();
    for col in &query.columns{
        let data_type = ColumnType::from(col.data_type.clone());
        columns.insert(col.name.to_string(), data_type);
    }
    let table = Table{
        name: query.name.to_string(),
        columns
    };

    dbg!(table);
}

