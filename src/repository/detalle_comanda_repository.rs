use crate::models::detalle_comanda::{
    ActualizarDetalleComanda, DetalleComanda, NuevoDetalleComanda,
};
use sqlx::{PgPool, Row};

pub struct DetalleComandaRepository {
    pool: PgPool,
}

impl DetalleComandaRepository {
    pub fn nuevo(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_todos(&self) -> sqlx::Result<Vec<DetalleComanda>> {
        let filas = sqlx::query("SELECT id_detalle, id_comanda, id_platillo, cantidad, notas_especiales FROM detalle_comanda")
        .fetch_all(&self.pool)
        .await?;

        let detalles_comanda = filas
            .into_iter()
            .map(|fila| DetalleComanda {
                id_detalle: fila.get("id_detalle"),
                id_comanda: fila.get("id_comanda"),
                id_platillo: fila.get("id_platillo"),
                cantidad: fila.get("cantidad"),
                notas_especiales: fila.get("notas_especiales"),
            })
            .collect();
        Ok(detalles_comanda)
    }

    pub async fn obtener_por_id(&self, id: i32) -> sqlx::Result<DetalleComanda> {
        let fila = sqlx::query("SELECT id_detalle, id_comanda, id_platillo, cantidad, notas_especiales FROM detalle_comanda WHERE id_detalle = $1")
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(DetalleComanda {
            id_detalle: fila.get("id_detalle"),
            id_comanda: fila.get("id_comanda"),
            id_platillo: fila.get("id_platillo"),
            cantidad: fila.get("cantidad"),
            notas_especiales: fila.get("notas_especiales"),
        })
    }

    pub async fn crear_detalle(
        &self,
        nuevo_detalle: NuevoDetalleComanda,
    ) -> sqlx::Result<DetalleComanda> {
        let fila = sqlx::query("INSERT INTO detalle_comanda (id_comanda, id_platillo, cantidad, notas_especiales) VALUES ($1, $2, $3, $4) RETURNING id_detalle, id_comanda, id_platillo, cantidad, notas_especiales")
        .bind(nuevo_detalle.id_comanda)
        .bind(nuevo_detalle.id_platillo)
        .bind(nuevo_detalle.cantidad)
        .bind(nuevo_detalle.notas_especiales)
        .fetch_one(&self.pool)
        .await?;

        Ok(DetalleComanda {
            id_detalle: fila.get("id_detalle"),
            id_comanda: fila.get("id_comanda"),
            id_platillo: fila.get("id_platillo"),
            cantidad: fila.get("cantidad"),
            notas_especiales: fila.get("notas_especiales"),
        })
    }

    pub async fn actualizar_detalle(
        &self,
        id: i32,
        detalle_actualizado: ActualizarDetalleComanda,
    ) -> sqlx::Result<DetalleComanda> {
        let fila = sqlx::query("UPDATE detalle_comanda SET id_comanda = $1, id_platillo = $2, cantidad = $3, notas_especiales = $4 WHERE id_detalle = $5 RETURNING id_detalle, id_comanda, id_platillo, cantidad, notas_especiales")
        .bind(detalle_actualizado.id_comanda)
        .bind(detalle_actualizado.id_platillo)
        .bind(detalle_actualizado.cantidad)
        .bind(detalle_actualizado.notas_especiales)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(DetalleComanda {
            id_detalle: fila.get("id_detalle"),
            id_comanda: fila.get("id_comanda"),
            id_platillo: fila.get("id_platillo"),
            cantidad: fila.get("cantidad"),
            notas_especiales: fila.get("notas_especiales"),
        })
    }

    pub async fn eliminar_detalle(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM detalle_comanda WHERE id_detalle = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
