CREATE TABLE Mesas (
    id_mesa SERIAL PRIMARY KEY,
    numero_mesa INT NOT NULL,
    capacidad INT,
    ubicacion VARCHAR(50) -- 'Terraza', 'Interior', 'Bar'
);

CREATE TABLE Categorias_Menu (
    id_categoria SERIAL PRIMARY KEY,
    nombre_categoria VARCHAR(50) NOT NULL -- 'Bebidas', 'Platos Fuertes', 'Postres'
);

CREATE TABLE Platillos (
    id_platillo SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    precio DECIMAL(10,2) NOT NULL,
    id_categoria INT REFERENCES Categorias_Menu(id_categoria)
);

CREATE TABLE Comandas (
    id_comanda SERIAL PRIMARY KEY,
    id_mesa INT REFERENCES Mesas(id_mesa),
    fecha_hora TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    mesero VARCHAR(50)
);

CREATE TABLE Detalle_Comanda (
    id_detalle SERIAL PRIMARY KEY,
    id_comanda INT REFERENCES Comandas(id_comanda),
    id_platillo INT REFERENCES Platillos(id_platillo),
    cantidad INT NOT NULL,
    notas_especiales TEXT
);