/**
 * MODEL - Mesa
 * Define la estructura de datos para la tabla Mesas.
 * Serde permite convertir automáticamente entre JSON y el struct.
 * FromRow permite que SQLx mapee las filas de la DB a este struct.
 */
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Representa una mesa tal como existe en la base de datos
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Mesa {
    pub id_mesa: i32,
    pub numero_mesa: i32,
    pub capacidad: Option<i32>,
    pub ubicacion: Option<String>,
}

/// Datos necesarios para CREAR una nueva mesa (sin el id, lo genera la DB)
#[derive(Debug, Serialize, Deserialize)]
pub struct NuevaMesa {
    pub numero_mesa: i32,
    pub capacidad: Option<i32>,
    pub ubicacion: Option<String>,
}
