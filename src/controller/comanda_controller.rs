use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::comanda_service::{
    obtener_comandas,
    obtener_comanda_por_id,
    crear_comanda,
    actualizar_comanda_por_id,
    eliminar_comanda_por_id,
};

pub fn comanda_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/comandas", get(obtener_comandas))
        .route("/api/comandas", post(crear_comanda))
        .route("/api/comandas/{id}", get(obtener_comanda_por_id))
        .route("/api/comandas/{id}", put(actualizar_comanda_por_id))
        .route("/api/comandas/{id}", delete(eliminar_comanda_por_id))
        .with_state(pool)
}