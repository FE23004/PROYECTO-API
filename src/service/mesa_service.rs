/**
 * SERVICE - Mesa
 * Contiene la lógica de negocio para las mesas.
 * Aquí se hacen validaciones antes de llamar al repository.
 * El service no sabe nada de HTTP, solo trabaja con datos.
 */
use sqlx::PgPool;
use crate::models::mesa::{Mesa, NuevaMesa};
use crate::repository::mesa_repository;

/// Retorna todas las mesas
pub async fn listar_mesas(pool: &PgPool) -> Result<Vec<Mesa>, String> {
    mesa_repository::obtener_todas(pool)
        .await
        .map_err(|e| format!("Error al obtener mesas: {}", e))
}

/// Busca una mesa por id, retorna error si no existe
pub async fn obtener_mesa(pool: &PgPool, id: i32) -> Result<Mesa, String> {
    let mesa = mesa_repository::obtener_por_id(pool, id)
        .await
        .map_err(|e| format!("Error al buscar mesa: {}", e))?;

    mesa.ok_or_else(|| format!("Mesa con id {} no encontrada", id))
}

/// Valida y crea una nueva mesa
pub async fn crear_mesa(pool: &PgPool, nueva: NuevaMesa) -> Result<Mesa, String> {
    // Validación: el número de mesa debe ser positivo
    if nueva.numero_mesa <= 0 {
        return Err("El número de mesa debe ser mayor a 0".to_string());
    }

    mesa_repository::crear(pool, nueva)
        .await
        .map_err(|e| format!("Error al crear mesa: {}", e))
}

/// Valida y actualiza una mesa existente
pub async fn actualizar_mesa(pool: &PgPool, id: i32, datos: NuevaMesa) -> Result<Mesa, String> {
    if datos.numero_mesa <= 0 {
        return Err("El número de mesa debe ser mayor a 0".to_string());
    }

    let resultado = mesa_repository::actualizar(pool, id, datos)
        .await
        .map_err(|e| format!("Error al actualizar mesa: {}", e))?;

    resultado.ok_or_else(|| format!("Mesa con id {} no encontrada", id))
}

/// Elimina una mesa por id
pub async fn eliminar_mesa(pool: &PgPool, id: i32) -> Result<(), String> {
    let filas = mesa_repository::eliminar(pool, id)
        .await
        .map_err(|e| format!("Error al eliminar mesa: {}", e))?;

    if filas == 0 {
        return Err(format!("Mesa con id {} no encontrada", id));
    }
    Ok(())
}
