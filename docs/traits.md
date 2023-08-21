# Traits
Los traits son una característica de Rust que nos permite definir comportamientos que pueden 
implementar diferentes tipos de datos.

### Ejemplo en Rust:
```rust
struct Persona {
    nombre: String,
    edad: u8,
}

trait Saludar {
    fn saludar(&self);
}

impl Saludar for Persona {
    fn saludar(&self) {
        println!("Hola, mi nombre es {} y tengo {} años", self.nombre, self.edad);
    }
}

fn main() {
    let persona = Persona {
        nombre: String::from("Juan"),
        edad: 25,
    };

    persona.saludar();
}
```

Más información en [**Rust Book: Traits**](https://rustlanges.github.io/rust-book-es/ch10-02-traits.html)