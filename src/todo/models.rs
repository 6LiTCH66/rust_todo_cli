use super::schema::todos;
#[derive(Queryable, Debug, AsChangeset)]
#[table_name = "todos"]
pub struct Todo{
    pub id: i32,
    pub todo_title: String,
}