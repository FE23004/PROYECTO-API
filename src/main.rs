mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

//cambiar por los controllers que tengas
use controller::platillo_controller::platillo_router;
use controller::categoria_menu_controller::categoria_router;
use controller::comanda_controller::comanda_router;
use controller::mesa_controller::mesa_router;
use controller::detalle_comanda_controller::detalle_comanda_router;


use config::config::crear_pool;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}


fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    platillo_router(pool.clone())
        .merge(categoria_router(pool.clone()))
        .merge(comanda_router(pool.clone()))
        .merge(mesa_router(pool.clone()))
        .merge(detalle_comanda_router(pool)) 
}