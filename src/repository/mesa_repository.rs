/**
 * REPOSITORY - Mesa
 * Contiene todas las consultas SQL para la tabla Mesas.
 * Solo se comunica con la base de datos, no contiene lógica de negocio.
 */
use sqlx::PgPool;
use crate::models::mesa::{Mesa, NuevaMesa};

/// Obtiene todas las mesas de la base de datos
pub async fn obtener_todas(pool: &PgPool) -> Result<Vec<Mesa>, sqlx::Error> {
    sqlx::query_as!(Mesa,
        "SELECT id_mesa, numero_mesa, capacidad, ubicacion FROM mesas ORDER BY id_mesa"
    )
    .fetch_all(pool)
    .await
}

/// Busca una mesa por su id
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Mesa>, sqlx::Error> {
    sqlx::query_as!(Mesa,
        "SELECT id_mesa, numero_mesa, capacidad, ubicacion FROM mesas WHERE id_mesa = $1",
        id
    )
    .fetch_optional(pool)
    .await
}

/// Inserta una nueva mesa y retorna el registro creado
pub async fn crear(pool: &PgPool, nueva: NuevaMesa) -> Result<Mesa, sqlx::Error> {
    sqlx::query_as!(Mesa,
        "INSERT INTO mesas (numero_mesa, capacidad, ubicacion)
         VALUES ($1, $2, $3)
         RETURNING id_mesa, numero_mesa, capacidad, ubicacion",
        nueva.numero_mesa,
        nueva.capacidad,
        nueva.ubicacion
    )
    .fetch_one(pool)
    .await
}

/// Actualiza una mesa existente y retorna el registro actualizado
pub async fn actualizar(pool: &PgPool, id: i32, datos: NuevaMesa) -> Result<Option<Mesa>, sqlx::Error> {
    sqlx::query_as!(Mesa,
        "UPDATE mesas
         SET numero_mesa = $1, capacidad = $2, ubicacion = $3
         WHERE id_mesa = $4
         RETURNING id_mesa, numero_mesa, capacidad, ubicacion",
        datos.numero_mesa,
        datos.capacidad,
        datos.ubicacion,
        id
    )
    .fetch_optional(pool)
    .await
}

/// Elimina una mesa por su id, retorna cuántas filas se eliminaron
pub async fn eliminar(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let resultado = sqlx::query!("DELETE FROM mesas WHERE id_mesa = $1", id)
        .execute(pool)
        .await?;
    Ok(resultado.rows_affected())
}
