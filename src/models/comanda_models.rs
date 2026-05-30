use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

/** Tabla Comandas — comanda de una mesa con mesero asignado */
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Comanda {
    pub id_comanda: i32,
    pub id_mesa: Option<i32>,
    pub fecha_hora: Option<NaiveDateTime>,
    pub mesero: Option<String>,
}

/** Modelo para crear una comanda (sin id ni fecha, se autogeneran) */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaComanda {
    pub id_mesa: i32,
    pub mesero: String,
}

/** Modelo para actualizar una comanda por id */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarComanda {
    pub id_mesa: Option<i32>,
    pub mesero: Option<String>,
}