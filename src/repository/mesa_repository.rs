use sqlx::{PgPool, Row};
use crate::models::mesa::{Mesa, NuevaMesa};

/// Obtiene todas las mesas de la base de datos
pub async fn obtener_todas(pool: &PgPool) -> Result<Vec<Mesa>, sqlx::Error> {
    let mesas = sqlx::query_as::<_, Mesa>(
        "SELECT id_mesa, numero_mesa, capacidad, ubicacion FROM mesas ORDER BY id_mesa"
    )
    .fetch_all(pool)
    .await?;

    Ok(mesas)
}

/// Busca una mesa por su id
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Mesa>, sqlx::Error> {
    let mesa = sqlx::query_as::<_, Mesa>(
        "SELECT id_mesa, numero_mesa, capacidad, ubicacion FROM mesas WHERE id_mesa = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(mesa)
}

/// Inserta una nueva mesa y retorna el registro creado
pub async fn crear(pool: &PgPool, nueva: NuevaMesa) -> Result<Mesa, sqlx::Error> {
    let mesa_creada = sqlx::query_as::<_, Mesa>(
        "INSERT INTO mesas (numero_mesa, capacidad, ubicacion)
         VALUES ($1, $2, $3)
         RETURNING id_mesa, numero_mesa, capacidad, ubicacion"
    )
    .bind(nueva.numero_mesa)
    .bind(nueva.capacidad)
    .bind(&nueva.ubicacion)
    .fetch_one(pool)
    .await?;

    Ok(mesa_creada)
}

/// Actualiza una mesa existente y retorna el registro actualizado
pub async fn actualizar(pool: &PgPool, id: i32, datos: NuevaMesa) -> Result<Option<Mesa>, sqlx::Error> {
    let mesa_actualizada = sqlx::query_as::<_, Mesa>(
        "UPDATE mesas
         SET numero_mesa = $1, capacidad = $2, ubicacion = $3
         WHERE id_mesa = $4
         RETURNING id_mesa, numero_mesa, capacidad, ubicacion"
    )
    .bind(datos.numero_mesa)
    .bind(datos.capacidad)
    .bind(&datos.ubicacion)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(mesa_actualizada)
}

/// Elimina una mesa por su id, retorna cuántas filas se eliminaron
pub async fn eliminar(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let resultado = sqlx::query("DELETE FROM mesas WHERE id_mesa = $1")
        .bind(id)
        .execute(pool)
        .await?;
        
    Ok(resultado.rows_affected())
}