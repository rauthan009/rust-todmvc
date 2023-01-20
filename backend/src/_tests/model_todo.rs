use super::{TodoMac, TodoPatch, TodoStatus};
use crate::security::utx_from_token;
use crate::{
    model,
    model::{db::init_db, todo::Todo},
};

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;
    let data_fx = TodoPatch {
        title: Some("test - mode_todo_create 1".to_string()),
        ..Default::default()
    };

    let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;

    assert!(todo_created.id >= 1000, "Id should be greater than 1000");
    assert_eq!(data_fx.title.unwrap(), todo_created.title);
    assert_eq!(TodoStatus::Open, todo_created.status);
    Ok(())
}

#[tokio::test]
async fn model_todo_get_ok() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    let todo = TodoMac::get(&db, &utx, 100).await?;
    assert_eq!(todo.id, 100);
    assert_eq!(todo.title, "todo 100");
    assert_eq!(todo.status, TodoStatus::Close);
    Ok(())
}

#[tokio::test]
async fn model_todo_get_wrong_id() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    let result = TodoMac::get(&db, &utx, 999).await;
    match result {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::EntityNotFound(typ, id)) => {
            assert_eq!("todo", typ);
            assert_eq!(999.to_string(), id);
        }
        other_error => assert!(false, "Wrong Error {:?}", other_error),
    }
    Ok(())
}

#[tokio::test]
async fn model_todo_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;
    let data_fx = TodoPatch {
        title: Some("test - mode_todo_update_ok 1".to_string()),
        ..Default::default()
    };
    let todo_fx = TodoMac::create(&db, &utx, data_fx.clone()).await?;
    let update_data_fx = TodoPatch {
        title: Some("test - mode_todo_update_ok 2".to_string()),
        ..Default::default()
    };

    let todo_updated = TodoMac::update(&db, &utx, todo_fx.id, update_data_fx.clone()).await?;
    let todos = TodoMac::list(&db, &utx).await?;
    assert_eq!(todos.len(), 3);
    assert_eq!(todo_updated.id, todo_fx.id);
    assert_eq!(todo_updated.title, update_data_fx.title.unwrap());
    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;
    let todos = TodoMac::list(&db, &utx).await?;
    assert_eq!(2, todos.len());
    Ok(())
}

#[tokio::test]
async fn model_todo_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;

    // -- ACTION
    let todo = TodoMac::delete(&db, &utx, 100).await?;

    // -- CHECK - deleted item
    assert_eq!(100, todo.id);
    assert_eq!("todo 100", todo.title);

    // -- CHECK - list
    let todos: Vec<Todo> = sqlb::select().table("todo").fetch_all(&db).await?;
    assert_eq!(1, todos.len());

    Ok(())
}
