# Slices
Los slices son una característica de Rust que nos permite tomar prestada una porción de un 
contenedor.
### Ejemplo en Rust:
```rust
fn main() {
    let slice = String::from("Hola, mundo");

    let hola = &slice[0..5];
    let mundo = &slice[7..12];

    println!("El valor de hola es: {}", hola);
    println!("El valor de mundo es: {}", mundo);
}
```

Más información en [**Rust Book: Slices**](https://rustlanges.github.io/rust-book-es/ch04-03-slices.html)