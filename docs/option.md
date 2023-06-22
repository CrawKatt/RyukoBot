# Option
Option es un tipo de dato genérico que nos permite representar la posibilidad de que un valor \
esté presente o no.
### Ejemplo en Rust:
```rust
fn main() {
    let numero: Option<i32> = Some(5);

    match numero {
        Some(x) => println!("El valor de x es: {}", x),
        None => println!("La opción no contiene un valor."),
    }
}
```

## Consejo
En Rust, no existe el `Null`, por lo que Option es una forma de representar la posibilidad de que \
un valor no esté presente.

