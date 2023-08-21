# Result
Result es un tipo de dato genérico que nos permite representar la posibilidad de que una operación 
falle.
### Ejemplo en Rust:
```rust
fn main() {
    let resultado = dividir(10, 2);

    match resultado {
        Ok(x) => println!("El resultado es: {}", x),
        Err(e) => println!("Error: {}", e),
    }
}

fn dividir(dividendo: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err(String::from("No se puede dividir entre 0"))
    } else {
        Ok(dividendo / divisor)
    }
}
```

Más información en [**Rust Book: Manejo de errores con Result**](https://rustlanges.github.io/rust-book-es/ch09-02-recoverable-errors-with-result.html)