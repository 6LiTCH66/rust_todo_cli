#[macro_use] extern crate diesel;
extern crate dotenv;

mod connection;
mod todo;
pub mod schema;
use std::io::{stdin};
use diesel::PgConnection;
use cli_table::{print_stdout, WithTitle};

fn main() {
    let conn = connection::database_connection();
    let mut choise = String::new();

    loop {
        println!("What you want to do?");
        println!("1 - Add new todo.");
        println!("2 - Delete todo.");
        println!("3 - Show all todos.");
        println!("4 - Exit.");
        println!("Your chose:");

        stdin().read_line(&mut choise).unwrap();
        match choise.trim().parse::<i32>() {
            Ok(number) => {
                choise.clear();
                match number {
                    1 => add_new_todo(&conn),
                    2 => {
                        delete_todo_by_id(&conn)
                    },
                    3 => {
                        show_all_todos(&conn)
                    }
                    4 => {
                        println!("See you soon!\nHave a nice day :)");
                        return;
                    },
                    _ => {
                        println!("Please try again!");
                        continue;
                    }
                }
            },
            Err(err) => {
                choise.clear();
                println!("Error: {}", err);
            }
        }
    };
}

fn show_all_todos(conn: &PgConnection){
    let all_todos = todo::repository::get_all_todos(conn);
    print_stdout(all_todos.with_title())
        .expect("Cannot print todos table");

}

fn delete_todo_by_id(conn: &PgConnection){
    let mut todo_id = String::new();
    println!("What todo would you like to delete?\nPlease enter todo's id:");
    stdin().read_line(&mut todo_id).unwrap();
    let todo_id = todo_id.trim().parse::<i32>();

    match todo_id {
        Ok(id) => {
            let check_id = todo::repository::delete_todo(conn, id);

            if check_id > 0 {
                println!("Todo with id: {} was successfully deleted!\n", id)
            } else {
                println!("Todo with id: {} was not found. \nPlease try again!\n", id)
            }
        }
        Err(_) => {
            println!("Invalid digits were entered!\nTry again.\n");
        }
    }
}

fn add_new_todo(conn: &PgConnection){
    let mut todo_title = String::new();
    println!("\nWhat would you like your todo title to be?");
    stdin().read_line(&mut todo_title).unwrap();

    let todo_title = todo_title.trim_end();

    let new_todo = todo::repository::create_new_todo(conn, todo_title.to_string());
    print_stdout(vec![new_todo].with_title()).expect("Cannot print todos table");
}
