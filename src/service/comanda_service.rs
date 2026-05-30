use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::comanda_models::{Comanda, NuevaComanda, ActualizarComanda};
use crate::repository::comanda_repository;

pub async fn obtener_comandas(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Comanda>>, StatusCode> {
    comanda_repository::obtener_todas(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn obtener_comanda_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Comanda>, StatusCode> {
    comanda_repository::obtener_por_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn crear_comanda(
    State(pool): State<PgPool>,
    Json(nueva): Json<NuevaComanda>,
) -> Result<(StatusCode, Json<Comanda>), StatusCode> {
    comanda_repository::crear(&pool, nueva)
        .await
        .map(|c| (StatusCode::CREATED, Json(c)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn actualizar_comanda_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<ActualizarComanda>,
) -> Result<Json<Comanda>, StatusCode> {
    comanda_repository::actualizar_por_id(&pool, id, datos)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn eliminar_comanda_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> StatusCode {
    match comanda_repository::eliminar_por_id(&pool, id).await {
        Ok(0) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}