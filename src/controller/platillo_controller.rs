use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::platillo_service::{
    actualizar_platillo,
    actualizar_platillo_por_id,
    crear_platillo,
    eliminar_platillo_por_id,
    obtener_platillo_por_id,
    obtener_platillos,
};

pub fn platillo_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/platillos",      get(obtener_platillos))
        .route("/api/platillos",      post(crear_platillo))
        .route("/api/platillos",      put(actualizar_platillo))
        .route("/api/platillos/{id}", get(obtener_platillo_por_id))
        .route("/api/platillos/{id}", put(actualizar_platillo_por_id))
        .route("/api/platillos/{id}", delete(eliminar_platillo_por_id))
        .with_state(pool)
}