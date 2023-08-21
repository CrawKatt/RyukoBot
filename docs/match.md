# Match
Match es una expresión que nos permite comparar un valor con una serie de patrones y ejecutar un 
bloque de código dependiendo de si el patrón coincide o no.
### Ejemplo en Rust:
```rust
fn main() {
    let numero = 5;
    match numero {
        1 => println!("El número es 1"),
        2 => println!("El número es 2"),
        3 => println!("El número es 3"),
        4 => println!("El número es 4"),
        5 => println!("El número es 5"),
        _ => println!("El número no es 1, 2, 3, 4 o 5"),
    }
}
```

Más información en [**Rust Book: Match**](https://rustlanges.github.io/rust-book-es/ch06-02-match.html)