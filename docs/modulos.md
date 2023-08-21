# Módulos
Los módulos son una característica de Rust que nos permiten organizar nuestro código en archivos 
separados.
### Ejemplo en Rust:
```rust
mod modulo {
    pub fn saludar() {
        println!("Hola!");
    }
}

fn main() {
    modulo::saludar();
}
```

Más información en [**Rust Book: Módulos**](https://rustlanges.github.io/rust-book-es/ch07-02-defining-modules-to-control-scope-and-privacy.html)