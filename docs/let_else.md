# Let else
Let else se utiliza para desempaquetar y hacer coincidir un valor de tipo Option<T> o Result de forma concisa. Es el opuesto de if let, permitiendo usar cláusulas de guarda para evitar la sobre-anidación de código.
### Ejemplo en Rust:
```rust
fn main() {
    let option_value: Option<i32> = Some(5);

    let Some(x) = option_value else {
        println!("La opción no contiene un valor.");
    };

    println!("El valor de x es: {}", x);
}
```