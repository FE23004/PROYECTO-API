use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::detalle_comanda_service::{
    obtener_detalle,
    obtener_detalle_por_id,
    crear_detalle,
    eliminar_detalle,
    eliminar_detalle_por_id,
    actualizar_detalle,
    actualizar_detalle_por_id,
};

pub fn detalle_comanda_router(pool: PgPool) -> Router{
    Router::new()
    .route("/api/detalleComanda", get(obtener_detalle))
    .route("/api/detalleComanda/{id}", get(obtener_detalle_por_id))
    .route("/api/detalleComanda", post(crear_detalle))
    .route("/api/detalleComanda", delete(eliminar_detalle))
    .route("/api/detalleComanda/{id}", delete(eliminar_detalle_por_id))
    .route("/api/detalleComanda", put(actualizar_detalle))
    .route("/api/detalleComanda/{id}", put(actualizar_detalle_por_id))
    .with_state(pool)
}
