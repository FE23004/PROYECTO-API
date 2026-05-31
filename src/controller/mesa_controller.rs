/**
 * CONTROLLER - Mesa
 * Maneja las solicitudes HTTP para el recurso /mesas.
 * Solo se encarga de recibir requests, llamar al service y retornar responses.
 * No contiene lógica de negocio ni consultas SQL.
 */
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};

use sqlx::PgPool;
use serde_json::{json, Value};

use crate::models::mesa::NuevaMesa;
use crate::service::mesa_service;

/// GET /mesas - Lista todas las mesas
async fn listar(
    State(pool): State<PgPool>,
) -> (StatusCode, Json<Value>) {
    match mesa_service::listar_mesas(&pool).await {
        Ok(mesas) => (
            StatusCode::OK,
            Json(json!(mesas))
        ),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e }))
        ),
    }
}

/// GET /mesas/{id} - Obtiene una mesa por id
async fn obtener(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Value>) {
    match mesa_service::obtener_mesa(&pool, id).await {
        Ok(mesa) => (
            StatusCode::OK,
            Json(json!(mesa))
        ),

        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": e }))
        ),
    }
}

/// POST /mesas - Crea una nueva mesa
async fn crear(
    State(pool): State<PgPool>,
    Json(body): Json<NuevaMesa>,
) -> (StatusCode, Json<Value>) {
    match mesa_service::crear_mesa(&pool, body).await {
        Ok(mesa) => (
            StatusCode::CREATED,
            Json(json!(mesa))
        ),

        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e }))
        ),
    }
}

/// PUT /mesas/{id} - Actualiza una mesa existente
async fn actualizar(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(body): Json<NuevaMesa>,
) -> (StatusCode, Json<Value>) {
    match mesa_service::actualizar_mesa(&pool, id, body).await {
        Ok(mesa) => (
            StatusCode::OK,
            Json(json!(mesa))
        ),

        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": e }))
        ),
    }
}

/// DELETE /mesas/{id} - Elimina una mesa
async fn eliminar(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Value>) {
    match mesa_service::eliminar_mesa(&pool, id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "mensaje": "Mesa eliminada correctamente"
            }))
        ),

        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": e }))
        ),
    }
}

/// Registra todas las rutas de mesas en el router
pub fn mesa_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/mesas", get(listar).post(crear))
        .route(
            "/api/mesas/{id}",
            get(obtener)
                .put(actualizar)
                .delete(eliminar),
        )
        .with_state(pool)
}
