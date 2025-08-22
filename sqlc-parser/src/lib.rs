use quote::{quote, ToTokens};
use sqlparser::ast::{ColumnDef, ColumnOption, CreateTable, DataType, Statement};
use sqlparser::parser::Parser;
use proc_macro2::TokenStream;
use sqlparser::dialect::GenericDialect;

#[derive(Debug,Clone,PartialEq)]
pub struct Table{
    name: String,
    columns: Vec<Column>,
}

impl Table{
    /// Get the name of the table.
    pub fn name(&self) -> &str{
        self.name.as_str()
    }

    /// Get the columns of the table.
    pub fn columns(&self) -> &[Column]{
        &self.columns
    }
    
    /// Get a column by its name.
    pub fn get_column(&self,name: &str) -> Option<&Column>{
        self.columns.iter().find(|c| c.name == name)
    }

    pub fn parse_query(query: &CreateTable) -> Self{
        let mut columns = vec![];
        for def in &query.columns{
            columns.push(Column::parse(def));
        }

        Self {
            name: query.name.to_string(),
            columns
        }
    }
}


#[derive(Debug,Clone,PartialEq,PartialOrd,)]
pub struct Column{
    pub name: String,
    pub field_type: ColumnType,
    pub is_unique: bool,
    pub is_primary: bool,
    pub nullable: bool
}

impl Column{
    pub fn parse(def: &ColumnDef) -> Self{
        let mut nullable = true;
        let mut is_unique = false;
        let mut is_primary = false;
        for option in &def.options{
            match option.option{
                ColumnOption::NotNull => nullable = false,
                ColumnOption::Unique{is_primary:primary_key,..} => {
                    is_unique = true;
                    if primary_key{ is_primary = true}
                },
                _ => {}
            }
        }
        Self{
            name: def.name.to_string(),
            field_type: ColumnType::from(def.data_type.clone()),
            is_primary,
            is_unique,
            nullable: true
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

impl ToTokens for ColumnType{
    fn to_tokens(&self, tokens: &mut TokenStream){
        let ty = match &self{
            ColumnType::Serial | ColumnType::Integer => quote!{ i32 } ,
            ColumnType::BigInt => quote!{ i64 } ,
            ColumnType::SmallInt => quote!{ i16 } ,
            ColumnType::Boolean => quote!{ bool } ,
            ColumnType::Text => quote!{ String } ,
            _ => panic!("Not implemented"),
        };
        tokens.extend(ty);
    }
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


pub fn parse_schema(schema: &str) -> Vec<Table>{
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, schema).unwrap();
    let mut tables = vec![];
    for stmt in &ast{
        match stmt{
            Statement::CreateTable(query) => {
                let table = Table::parse_query(query);
                tables.push(table);
            },
            _ => panic!("Not a CreateTable")
        }
    }

    tables
}
