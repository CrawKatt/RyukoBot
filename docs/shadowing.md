# Shadowing
Shadowing es una característica de Rust que nos permite declarar una variable con el mismo nombre 
que otra variable en un scope superior.
### Ejemplo en Rust:
```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("El valor de x es: {}", x);
}
```

