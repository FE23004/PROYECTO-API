use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/** Tabla de Categorias_Menu */
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct CategoriaMenu {
    pub id_categoria: i32,
    pub nombre_categoria: String,
}

/** Modelo para crear una categoría (sin id autogenerado) */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaCategoria {
    pub nombre_categoria: String,
}

/** Modelo para actualizar una categoría por id */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarCategoria {
    pub nombre_categoria: String,
}