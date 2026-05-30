use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::PgPool;
use crate::service::categoria_service::{
    actualizar_categoria,
    actualizar_categoria_por_id,
    crear_categoria,
    eliminar_categoria_por_id,
    obtener_categorias,
};

pub fn categoria_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/categorias", get(obtener_categorias))
        .route("/api/categorias", post(crear_categoria))
        .route("/api/categorias/{id}", delete(eliminar_categoria_por_id))
        .route("/api/categorias/{id}", put(actualizar_categoria_por_id))
        .route("/api/categorias", put(actualizar_categoria))
        .with_state(pool)
}