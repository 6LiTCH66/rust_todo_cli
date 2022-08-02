use diesel::prelude::*;
use crate::todo::models::*;

// use crate::connection::*;

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

#[cfg(test)]
mod tests{
    use crate::connection::database_connection;
    use super::*;
    use crate::todo::models::*;


    #[test]
    fn deleting_new_todo_test(){
        let conn = database_connection();
        let todo_test = create_new_todo(&conn, "testing delete_todo".to_owned());
        let is_delete = delete_todo(&conn, todo_test.id);
        assert_eq!(1, is_delete);
    }

    #[test]
    fn creating_new_todo_test(){
        let conn = database_connection();
        let new_todo = create_new_todo(&conn, "testing new todo".to_owned());
        assert_eq!("testing new todo".to_string(), new_todo.todo_title);
        delete_todo(&conn, new_todo.id);
    }

    #[test]
    fn deleting_non_existing_todo_test(){
        let conn = database_connection();
        let delete_todo_id = delete_todo(&conn, 9999);
        assert_eq!(0, delete_todo_id);
    }

    #[test]
    fn listing_todos_title_test(){
        let conn = database_connection();

        let todos_titles: Vec<String> = vec![
            "testing 1".to_string(),
            "testing 2".to_string(),
            "testing 3".to_string(),
        ];

        let new_todo_1 = create_new_todo(&conn, "testing 1".to_string());
        let new_todo_2 = create_new_todo(&conn, "testing 2".to_string());
        let new_todo_3 = create_new_todo(&conn, "testing 3".to_string());


        let all_todos = get_all_todos(&conn);

        let all_todos_titles =
            all_todos
                .iter()
                .map(|todo| todo.todo_title.clone()).collect::<Vec<String>>();

        let is_contains =
            todos_titles
                .iter()
                .all(|title| all_todos_titles.contains(title));


        delete_todo(&conn,new_todo_1.id);
        delete_todo(&conn, new_todo_2.id);
        delete_todo(&conn, new_todo_3.id);

        assert!(is_contains);

    }


}
