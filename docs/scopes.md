# Scopes
Los scopes son el alcance que tienen las variables en nuestro código.

### Ejemplo en Rust:
```rust
fn main() {
    let x = 5; // x entra en el scope

    { // Inicio de un nuevo scope
        let y = 10; // y entra en el scope
        println!("{}", x); // x está disponible aquí
        println!("{}", y); // y también está disponible aquí
    } // y sale del scope y se libera de la memoria

    println!("{}", x); // x está disponible aquí
    println!("{}", y); // y no está disponible aquí
}
```

