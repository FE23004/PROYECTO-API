use sqlx::PgPool;
use crate::models::categoria_menu::{CategoriaMenu, NuevaCategoria};

pub struct CategoriaRepository {
    pool: PgPool,
}

impl CategoriaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_categorias(&self) -> Result<Vec<CategoriaMenu>, sqlx::Error> {
        sqlx::query_as::<_, CategoriaMenu>(
            "SELECT id_categoria, nombre_categoria FROM Categorias_Menu"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn crear_categoria(
        &self,
        nueva: NuevaCategoria,
    ) -> Result<CategoriaMenu, sqlx::Error> {
        sqlx::query_as::<_, CategoriaMenu>(
            "INSERT INTO Categorias_Menu (nombre_categoria)
             VALUES ($1)
             RETURNING id_categoria, nombre_categoria"
        )
        .bind(nueva.nombre_categoria)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn eliminar_categoria(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM Categorias_Menu WHERE id_categoria = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn actualizar_categoria(
        &self,
        id: i32,
        datos: NuevaCategoria,
    ) -> Result<CategoriaMenu, sqlx::Error> {
        sqlx::query_as::<_, CategoriaMenu>(
            "UPDATE Categorias_Menu
             SET nombre_categoria = $1
             WHERE id_categoria = $2
             RETURNING id_categoria, nombre_categoria"
        )
        .bind(datos.nombre_categoria)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn obtener_categoria_por_id(&self, id: i32) -> Result<CategoriaMenu, sqlx::Error> {
    sqlx::query_as::<_, CategoriaMenu>(
        "SELECT id_categoria, nombre_categoria FROM Categorias_Menu WHERE id_categoria = $1"
    )
    .bind(id)
    .fetch_one(&self.pool)
    .await
}
}