use crate::models::todo::{CreateTodoList, TodoItem, TodoList};
use crate::response::{BusinessError, Response};
use actix_web::{web, HttpResponse};
use sqlx::{query, query_as, PgPool};

pub async fn get_todos(db_pool: web::Data<PgPool>) -> Result<HttpResponse, BusinessError> {
  let data = query_as!(TodoList, "select * from todo_list")
    .fetch_all(&**db_pool)
    .await;

  match data {
    Ok(data) => {
      let res = Response::success(data);
      Ok(HttpResponse::Ok().json(res))
    }
    Err(_) => Err(BusinessError::InternalError),
  }
}

pub async fn get_items(db_pool: web::Data<PgPool>, path: web::Path<(i32,)>) -> Result<HttpResponse, BusinessError> {
  let list_id = path.0;
  let data = query_as!(
    TodoItem,
    r#"select * from todo_item where list_id = $1 order by id"#,
    list_id
  )
  .fetch_all(&**db_pool)
  .await;

  match data {
    Ok(data) => {
      let res = Response::success(data);
      Ok(HttpResponse::Ok().json(res))
    }
    Err(_) => Err(BusinessError::InternalError),
  }
}

pub async fn create_todo(
  db_pool: web::Data<PgPool>,
  params: web::Json<CreateTodoList>,
) -> Result<HttpResponse, BusinessError> {
  let data = query_as!(
    TodoList,
    r#"insert into todo_list (title) values ($1) returning id, title"#,
    &params.title,
  )
  .fetch_one(&**db_pool)
  .await;

  match data {
    Ok(data) => {
      let res = Response::success(data);
      Ok(HttpResponse::Ok().json(res))
    }
    Err(_) => {
      let field = String::from("title");
      Err(BusinessError::ValidationError { field })
    }
  }
}

pub async fn update_todo(
  db_pool: web::Data<PgPool>,
  params: web::Json<CreateTodoList>,
  path: web::Path<(i32,)>,
) -> Result<HttpResponse, BusinessError> {
  let data = query_as!(
    TodoList,
    r#"update todo_list set title = $1 where id = $2 returning id, title"#,
    &params.title,
    path.0
  )
  .fetch_one(&**db_pool)
  .await;

  match data {
    Ok(data) => {
      let res = Response::success(data);
      Ok(HttpResponse::Ok().json(res))
    }
    Err(_) => Err(BusinessError::InternalError),
  }
}

pub async fn delete_todo_items(
  db_pool: web::Data<PgPool>,
  path: web::Path<(i32,)>,
) -> Result<HttpResponse, BusinessError> {
  query!("delete from todo_item where list_id = $1", path.0)
    .execute(&**db_pool)
    .await
    .unwrap();

  let res = Response::success("删除成功");
  Ok(HttpResponse::Ok().json(res))
}
