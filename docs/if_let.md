# If let
If let se utiliza para desempaquetar y hacer coincidir un valor de tipo Option<T> o Result de forma concisa.

### Ejemplo en Rust:
```rust
fn main() {
    let option_value: Option<i32> = Some(5);

    if let Some(x) = option_value {
        println!("El valor de x es: {}", x);
    } else {
        println!("La opción no contiene un valor.");
    }
}
```

Más información en [**Rust Book: Control Flow**](https://rustlanges.github.io/rust-book-es/ch06-03-if-let.html)