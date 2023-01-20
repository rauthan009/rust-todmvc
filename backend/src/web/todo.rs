use crate::{
    model::{Db, TodoMac, TodoPatch},
    security::{utx_from_token, UserCtx},
};
use serde_json::json;
use std::{convert::Infallible, sync::Arc};
use warp::{reply::Json, Filter, Rejection};

pub fn todo_rest_filters(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_path = warp::path(base_path).and(warp::path("todos"));
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    let list = todos_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(todo_list);

    list
}

async fn todo_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let todos = TodoMac::list(&db, &utx).await.unwrap();
    let response = json!({ "data": todos });
    Ok(warp::reply::json(&response))
}
// fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
//     let response = json!({ "data": data });
//     Ok(warp::reply::json(&response))
// }

//region Filter Utils
pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
pub fn do_auth(db: Arc<Db>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
    warp::any()
        .and_then(|| async { Ok::<UserCtx, Rejection>(utx_from_token("123").await.unwrap()) })
}
// 	warp::any()
// 		.and(with_db(db))
// 		.and(warp::header::optional(HEADER_XAUTH))
// 		.and_then(|db: Arc<Db>, xauth: Option<String>| async move {
// 			match xauth {
// 				Some(xauth) => {
// 					let utx = utx_from_token(&db, &xauth).await?;
// 					Ok::<UserCtx, Rejection>(utx)
// 				}
// 				None => Err(Error::FailAuthMissingXAuth.into()),
// 			}
// 		})
// }
