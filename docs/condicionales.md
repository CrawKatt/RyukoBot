# Condicionales
Son grupos de sentencias o sentencias individuales que te permiten condicionar la decisión entre 
la elección de una opción y otra.
### Ejemplo en Rust:
```rust
fn main() {
    let color = "verde";
    if color == "rojo" {
        println!("El color es rojo");
    } else if color == "azul" {
        println!("El color es azul");
    } else {
        println!("El color es otro");
    }
}
```

Más información en [**Rust Book: Flujos de Control**](https://rustlanges.github.io/rust-book-es/ch03-05-control-flow.html#condicionales)