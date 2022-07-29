use diesel::prelude::*;
use crate::todo::models::*;
pub fn get_all_todos(conn: &PgConnection) -> Vec<Todo>{
    use crate::schema::todos::dsl::*;

    todos.select((id, todo_title))
        .load::<Todo>(conn)
        .expect("Error loading todos")
}


pub fn create_new_todo(conn: &PgConnection, new_todo_title: String) -> Todo {
    use crate::schema::todos;
    let new_todo = NewTodo{
        todo_title:new_todo_title
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_todo(conn: &PgConnection, todo_id: i32) -> usize{
    use crate::schema::todos::dsl::*;
    diesel::delete(todos.find(todo_id)).execute(conn).unwrap()
}