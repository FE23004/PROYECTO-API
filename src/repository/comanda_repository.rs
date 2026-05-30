use sqlx::PgPool;
use crate::models::comanda_models::{Comanda, NuevaComanda, ActualizarComanda};

pub async fn obtener_todas(pool: &PgPool) -> Result<Vec<Comanda>, sqlx::Error> {
    sqlx::query_as::<_, Comanda >(
        "SELECT id_comanda, id_mesa, fecha_hora, mesero FROM Comandas ORDER BY id_comanda"
    )
    .fetch_all(pool)
    .await
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Comanda>, sqlx::Error> {
    sqlx::query_as::<_, Comanda>(
        "SELECT id_comanda, id_mesa, fecha_hora, mesero FROM Comandas WHERE id_comanda = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn crear(pool: &PgPool, nueva: NuevaComanda) -> Result<Comanda, sqlx::Error> {
    sqlx::query_as::<_, Comanda>(
        "INSERT INTO Comandas (id_mesa, mesero)
         VALUES ($1, $2)
         RETURNING id_comanda, id_mesa, fecha_hora, mesero"
    )
    .bind(nueva.id_mesa)
    .bind(nueva.mesero)
    .fetch_one(pool)
    .await
}

pub async fn actualizar_por_id(
    pool: &PgPool,
    id: i32,
    datos: ActualizarComanda,
) -> Result<Option<Comanda>, sqlx::Error> {
    sqlx::query_as::<_, Comanda>(
        "UPDATE Comandas
         SET id_mesa = $1, mesero = $2
         WHERE id_comanda = $3
         RETURNING id_comanda, id_mesa, fecha_hora, mesero"
    )
    .bind(datos.id_mesa)
    .bind(datos.mesero)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn eliminar_por_id(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM Comandas WHERE id_comanda = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected())
}