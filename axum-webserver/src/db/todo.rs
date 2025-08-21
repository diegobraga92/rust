use crate::models::todo::{NewTodo, Todo};
use crate::schema::todos;
use diesel::prelude::*;

pub fn create_todo(conn: &mut PgConnection, new_todo: NewTodo) -> QueryResult<Todo> {
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)
}

pub fn list_todos(conn: &mut PgConnection) -> QueryResult<Vec<Todo>> {
    todos::table.select(Todo::as_select()).load(conn)
}
