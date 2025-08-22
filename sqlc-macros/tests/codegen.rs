use sqlc_macros::parse_sql;

#[test]
fn codegen(){
    mod db{
        use super::*;
        
        parse_sql!{}
    }
    let user = db::User{
        id: 1,
        email: String::from("")
    };
    dbg!(user);
}