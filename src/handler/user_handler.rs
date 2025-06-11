use askama::Template;
use axum::{
    Form, Router,
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{AppState, model::user::User};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index_page))
        .route("/users/create", get(create_page))
        .route("/users/create", post(create_user))
        .route("/users/{id}/delete", post(delete_user))
        .route("/users/{id}/update", get(update_page))
        .route("/users/{id}/update", post(update_user))
}

#[derive(Template)]
#[template(path = "create.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
}

pub async fn create_page() -> impl IntoResponse {
    let title = "Create";
    let tmpl = RegisterTemplate { title };

    Html(tmpl.render().unwrap())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    users: Vec<User>,
}

pub async fn index_page(State(state): State<AppState>) -> impl IntoResponse {
    let users = User::all(&state.pool).await.unwrap_or_else(|_| vec![]);

    let tmpl = IndexTemplate {
        title: "Home",
        users,
    };

    Html(tmpl.render().unwrap()).into_response()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterData {
    pub name: String,
    pub email: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Form(data): Form<RegisterData>,
) -> impl IntoResponse {
    if let Err(e) = User::create(&state.pool, data).await {
        println!("Erro ao criar usuário: {}", e);
        return Redirect::to("/");
    }

    Redirect::to("/")
}

#[derive(Template)]
#[template(path = "update.html")]
struct UpdateTemplate<'a> {
    title: &'a str,
    user: User,
}

pub async fn update_page(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    if let Err(_e) = User::find(&state.pool, id).await {
        println!("Usuário não encontrado");
        return Redirect::to("/").into_response();
    }

    let user = User::find(&state.pool, id).await.unwrap();

    let tmpl = UpdateTemplate {
        title: "Update",
        user,
    };

    Html(tmpl.render().unwrap()).into_response()
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(data): Form<RegisterData>,
) -> impl IntoResponse {
    if let Err(e) = User::find(&state.pool, id).await {
        println!("Erro ao atualizar usuário: {}", e);
        return Redirect::to("/create");
    }
    if let Err(e) = User::update(&state.pool, id, data).await {
        println!("Erro ao atualizar usuário: {}", e);
        return Redirect::to("/create");
    }
    Redirect::to("/")
}

pub async fn delete_user(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    if let Err(e) = User::find(&state.pool, id).await {
        println!("Erro ao excluir usuário: {}", e);
    }

    if let Err(e) = User::delete(&state.pool, id).await {
        print!("Erro ao excluir usuario {}", e);
    }

    return Redirect::to("/");
}
