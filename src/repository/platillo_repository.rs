use sqlx::PgPool;
use crate::models::platillo::{ActualizarPlatillo, NuevoPlatillo, Platillo};

pub async fn obtener_todos_platillos(pool: &PgPool) -> Result<Vec<Platillo>, sqlx::Error> {
    sqlx::query_as::<_, Platillo>(
        "SELECT id_platillo, nombre, precio::float8, id_categoria FROM Platillos"
    )
        .fetch_all(pool)
        .await
}

pub async fn obtener_platillo_por_id(pool: &PgPool, id: i32) -> Result<Option<Platillo>, sqlx::Error> {
    sqlx::query_as::<_, Platillo>(
        "SELECT id_platillo, nombre, precio::float8, id_categoria FROM Platillos WHERE id_platillo = $1"
    )
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn crear_platillo(pool: &PgPool, nuevo: NuevoPlatillo) -> Result<Platillo, sqlx::Error> {
    sqlx::query_as::<_, Platillo>(
        "INSERT INTO Platillos (nombre, precio, id_categoria)
         VALUES ($1, $2, $3)
         RETURNING id_platillo, nombre, precio::float8, id_categoria"
    )
        .bind(&nuevo.nombre)
        .bind(nuevo.precio)
        .bind(nuevo.id_categoria)
        .fetch_one(pool)
        .await
}

pub async fn eliminar_platillo_por_id(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM Platillos WHERE id_platillo = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub async fn actualizar_platillo_por_id(
    pool: &PgPool,
    id: i32,
    datos: ActualizarPlatillo,
) -> Result<Option<Platillo>, sqlx::Error> {
    sqlx::query_as::<_, Platillo>(
        "UPDATE Platillos
         SET nombre = $1, precio = $2, id_categoria = $3
         WHERE id_platillo = $4
         RETURNING id_platillo, nombre, precio::float8, id_categoria"
    )
        .bind(&datos.nombre)
        .bind(datos.precio)
        .bind(datos.id_categoria)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn actualizar_todos_platillos(
    pool: &PgPool,
    datos: ActualizarPlatillo,
) -> Result<Vec<Platillo>, sqlx::Error> {
    sqlx::query_as::<_, Platillo>(
        "UPDATE Platillos
         SET nombre = $1, precio = $2, id_categoria = $3
         RETURNING id_platillo, nombre, precio::float8, id_categoria"
    )
        .bind(&datos.nombre)
        .bind(datos.precio)
        .bind(datos.id_categoria)
        .fetch_all(pool)
        .await
}