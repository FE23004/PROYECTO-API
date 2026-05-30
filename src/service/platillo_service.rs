use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::platillo::{ActualizarPlatillo, NuevoPlatillo, Platillo};
use crate::repository::platillo_repository;

pub async fn obtener_platillos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Platillo>>, (StatusCode, String)> {
    platillo_repository::obtener_todos_platillos(&pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn obtener_platillo_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Platillo>, (StatusCode, String)> {
    match platillo_repository::obtener_platillo_por_id(&pool, id).await {
        Ok(Some(p)) => Ok(Json(p)),
        Ok(None) => Err((StatusCode::NOT_FOUND, format!("Platillo {} no encontrado", id))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn crear_platillo(
    State(pool): State<PgPool>,
    Json(nuevo): Json<NuevoPlatillo>,
) -> Result<(StatusCode, Json<Platillo>), (StatusCode, String)> {
    platillo_repository::crear_platillo(&pool, nuevo)
        .await
        .map(|p| (StatusCode::CREATED, Json(p)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn eliminar_platillo_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    match platillo_repository::eliminar_platillo_por_id(&pool, id).await {
        Ok(filas) if filas > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err((StatusCode::NOT_FOUND, format!("Platillo {} no encontrado", id))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn actualizar_platillo_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<ActualizarPlatillo>,
) -> Result<Json<Platillo>, (StatusCode, String)> {
    match platillo_repository::actualizar_platillo_por_id(&pool, id, datos).await {
        Ok(Some(p)) => Ok(Json(p)),
        Ok(None) => Err((StatusCode::NOT_FOUND, format!("Platillo {} no encontrado", id))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn actualizar_platillo(
    State(pool): State<PgPool>,
    Json(datos): Json<ActualizarPlatillo>,
) -> Result<Json<Vec<Platillo>>, (StatusCode, String)> {
    platillo_repository::actualizar_todos_platillos(&pool, datos)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}