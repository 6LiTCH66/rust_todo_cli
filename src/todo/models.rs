use crate::schema::todos;
use cli_table::{format::Justify, Table};

#[derive(Queryable, Debug, AsChangeset, Table, Clone)]
pub struct Todo{
    #[table(title = "ID", justify = "Justify::Right")]
    pub id: i32,
    #[table(title = "Todo title")]
    pub todo_title: String,
}

#[derive(Insertable, Debug)]
#[table_name = "todos"]
pub struct NewTodo{
    pub todo_title: String
}


