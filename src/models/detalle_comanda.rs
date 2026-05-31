use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct DetalleComanda{
    pub id_detalle : i32,
    pub id_comanda : i32,
    pub id_platillo :i32,
    pub cantidad : i32,
    pub notas_especiales: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoDetalleComanda{
    pub id_comanda : i32,
    pub id_platillo : i32,
    pub cantidad : i32,
    pub notas_especiales: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarDetalleComanda{
    pub id_comanda : i32,
    pub id_platillo : i32,
    pub cantidad : i32,
    pub notas_especiales: String,
}

