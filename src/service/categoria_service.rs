use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;
use crate::models::categoria_menu::{ActualizarCategoria, CategoriaMenu, NuevaCategoria};
use crate::repository::categoria_repository::CategoriaRepository;

pub async fn obtener_categorias(State(pool): State<PgPool>) -> Json<Vec<CategoriaMenu>> {
    let repo = CategoriaRepository::new(pool);
    match repo.obtener_categorias().await {
        Ok(categorias) => Json(categorias),
        Err(_) => Json(vec![]),
    }
}

pub async fn crear_categoria(
    State(pool): State<PgPool>,
    Json(nueva_categoria): Json<NuevaCategoria>,
) -> Json<CategoriaMenu> {
    let repo = CategoriaRepository::new(pool);
    match repo.crear_categoria(nueva_categoria).await {
        Ok(categoria) => Json(categoria),
        Err(_) => Json(CategoriaMenu {
            id_categoria: 0,
            nombre_categoria: "Error al crear la categoría".to_string(),
        }),
    }
}

pub async fn eliminar_categoria_por_id(
    State(pool): State<PgPool>,
    Path(id_categoria): Path<i32>,
) -> Json<bool> {
    let repo = CategoriaRepository::new(pool);
    match repo.eliminar_categoria(id_categoria).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn actualizar_categoria(
    State(pool): State<PgPool>,
    Json(categoria_actualizada): Json<CategoriaMenu>,
) -> Json<CategoriaMenu> {
    let repo = CategoriaRepository::new(pool);
    let id = categoria_actualizada.id_categoria;
    let datos = NuevaCategoria {
        nombre_categoria: categoria_actualizada.nombre_categoria,
    };
    match repo.actualizar_categoria(id, datos).await {
        Ok(categoria) => Json(categoria),
        Err(_) => Json(CategoriaMenu {
            id_categoria: 0,
            nombre_categoria: "Error al actualizar la categoría".to_string(),
        }),
    }
}

pub async fn actualizar_categoria_por_id(
    State(pool): State<PgPool>,
    Path(id_categoria): Path<i32>,
    Json(categoria_actualizada): Json<ActualizarCategoria>,
) -> Json<CategoriaMenu> {
    let repo = CategoriaRepository::new(pool);
    let datos = NuevaCategoria {
        nombre_categoria: categoria_actualizada.nombre_categoria,
    };
    match repo.actualizar_categoria(id_categoria, datos).await {
        Ok(categoria) => Json(categoria),
        Err(_) => Json(CategoriaMenu {
            id_categoria: 0,
            nombre_categoria: "Error al actualizar la categoría".to_string(),
        }),
    }
}