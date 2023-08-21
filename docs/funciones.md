# Funciones
Las funciones son bloques de código que se pueden reutilizar en diferentes partes de nuestro programa.
### Ejemplo en Rust:
```rust
fn saludar(nombre: &str) {
   println!("Hola, {}!", nombre);
}

fn main() {
    saludar("Juan");
}
```

Más información en [**Rust Book: Funciones**](https://rustlanges.github.io/rust-book-es/ch03-03-how-functions-work.html)