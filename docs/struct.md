# Struct
Struct es una característica de Rust que nos permite crear nuestros propios tipos de datos.

### Ejemplo en Rust:
```rust
struct Persona {
    nombre: String,
    edad: u8,
}

fn main() {
    let persona = Persona {
        nombre: String::from("Juan"),
        edad: 25,
    };

    println!("{} tiene {} años", persona.nombre, persona.edad);
}
```

Más información en [**Rust Book: Struct**](https://rustlanges.github.io/rust-book-es/ch05-01-defining-structs.html)