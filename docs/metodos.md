# Métodos
Los métodos son funciones que se definen dentro del contexto de un struct, enum o 
implementación de un trait.

### Ejemplo en Rust:
```rust
struct Persona {
    nombre: String,
    edad: u8,
}

impl Persona {
    fn saludar(&self) {
        println!("Hola, mi nombre es {} y tengo {} años.", self.nombre, self.edad);
    }
}

fn main() {
    let juan = Persona {
        nombre: String::from("Juan"),
        edad: 25,
    };

    juan.saludar();
}
```

Más información en [**Rust Book: Methods**](https://rustlanges.github.io/rust-book-es/ch05-03-method-syntax.html)