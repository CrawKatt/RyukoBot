# Closures
Los Closures son funciones anónimas que se pueden almacenar en variables o pasar como argumentos a otras funciones
### Ejemplo en Rust:
```rust
let suma = |a: i32, b: i32| -> i32 {
    a + b
 };

fn main() {
    let resultado = suma(5,5);
    println!("{}", resultado);
}
```

Más información en [**Rust Book: Closures**](https://rustlanges.github.io/rust-book-es/ch13-01-closures.html)