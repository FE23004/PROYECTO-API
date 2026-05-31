//use crate::repository::detalle_comanda_repository;
use crate::{
    models::detalle_comanda::{
        ActualizarDetalleComanda, DetalleComanda, NuevoDetalleComanda,
    },
    repository::detalle_comanda_repository::DetalleComandaRepository,
};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub async fn obtener_detalle(State(pool): State<PgPool>) -> Json<Vec<DetalleComanda>> {
    let repo = DetalleComandaRepository::nuevo(pool);
    match repo.obtener_todos().await {
        Ok(detalles) => Json(detalles),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_detalle_por_id(
    State(pool): State<PgPool>,  
    Path(id_detalle): Path<i32>
) -> Json<DetalleComanda> {
    let repo = DetalleComandaRepository::nuevo(pool);

    match repo.obtener_por_id(id_detalle).await {
        Ok(detalle) => Json(detalle),
        Err(_) => Json(DetalleComanda {
            id_detalle: 0,
            id_comanda: 0,
            id_platillo: 0,
            cantidad: 0,
            notas_especiales: "No encontrado o error en base de datos".to_string(),
        }),
    }
}

pub async fn crear_detalle(
    State(pool): State<PgPool>,
    Json(nuevo_detalle): Json<NuevoDetalleComanda>,
) -> Json<DetalleComanda> {
    let repo = DetalleComandaRepository::nuevo(pool);
    match repo.crear_detalle(nuevo_detalle).await {
        Ok(detalle) => Json(detalle),
        Err(_) => Json(DetalleComanda {
            id_detalle: 0,
            id_comanda: 0,
            id_platillo: 0,
            cantidad: 0,
            notas_especiales: "Error al crear el detalle".to_string(),
        }),
    }
}

pub async fn eliminar_detalle(
    State(pool): State<PgPool>,
    Json(id_detalle): Json<i32>,
) -> Json<bool> {
    let repo = DetalleComandaRepository::nuevo(pool);
    match repo.eliminar_detalle(id_detalle).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_detalle_por_id(
    State(pool): State<PgPool>,
    Path(id_detalle): Path<i32>,
) -> Json<bool> {
    let repo = DetalleComandaRepository::nuevo(pool);
    match repo.eliminar_detalle(id_detalle).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn actualizar_detalle(
    State(pool): State<PgPool>,
    Json(detalle_actualizado): Json<DetalleComanda>,
) -> Json<DetalleComanda> {
    let repo = DetalleComandaRepository::nuevo(pool);
    let id = detalle_actualizado.id_detalle;
    let nuevo_detalle = ActualizarDetalleComanda {
        id_comanda: detalle_actualizado.id_comanda,
        id_platillo: detalle_actualizado.id_platillo,
        cantidad: detalle_actualizado.cantidad,
        notas_especiales: detalle_actualizado.notas_especiales,
    };
    match repo.actualizar_detalle(id, nuevo_detalle).await {
        Ok(detalle) => Json(detalle),
        Err(_) => Json(DetalleComanda {
            id_detalle: 0,
            id_comanda: 0,
            id_platillo: 0,
            cantidad: 0,
            notas_especiales: "Error al actualizar el detalle".to_string(),
        }),
    }
}


pub async fn actualizar_detalle_por_id(
    State(pool): State<PgPool>,
    Path(id_detalle): Path<i32>,
    Json(detalle_actualizado): Json<ActualizarDetalleComanda>,
)   -> Json<crate::models::detalle_comanda::DetalleComanda>{
    let repo = DetalleComandaRepository::nuevo(pool);
    let nuevo_detalle = ActualizarDetalleComanda{
        id_comanda: detalle_actualizado.id_comanda,
        id_platillo: detalle_actualizado.id_platillo,
        cantidad: detalle_actualizado.cantidad,
        notas_especiales: detalle_actualizado.notas_especiales,
    };

    match repo.actualizar_detalle(id_detalle, nuevo_detalle).await{
        Ok(detalle) => Json(detalle),
        Err(_) => Json(DetalleComanda { 
            id_detalle: 0, 
            id_comanda: 0, 
            id_platillo: 0, 
            cantidad: 0, 
            notas_especiales: "Error al actualizar detalle".to_string(), 
        }),
    }
}
