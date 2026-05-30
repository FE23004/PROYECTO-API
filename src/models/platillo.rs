use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Platillo {
    pub id_platillo: i32,
    pub nombre: String,
    pub precio: f64,
    pub id_categoria: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoPlatillo {
    pub nombre: String,
    pub precio: f64,
    pub id_categoria: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarPlatillo {
    pub nombre: String,
    pub precio: f64,
    pub id_categoria: Option<i32>,
}