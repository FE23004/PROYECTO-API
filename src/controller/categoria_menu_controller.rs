use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::PgPool;
use crate::service::categoria_service::{
    actualizar_categoria_por_id,
    crear_categoria,
    eliminar_categoria_por_id,
    obtener_categorias,
    obtener_categoria_por_id, 
};

pub fn categoria_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/categorias", get(obtener_categorias))
        .route("/api/categorias/{id}", get(obtener_categoria_por_id))  
        .route("/api/categorias", post(crear_categoria))
        .route("/api/categorias/{id}", delete(eliminar_categoria_por_id))
        .route("/api/categorias/{id}", put(actualizar_categoria_por_id))
        .with_state(pool)
}