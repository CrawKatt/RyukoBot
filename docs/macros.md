# Macros
Las macros son una característica de Rust que nos permiten escribir código que genera código.
### Ejemplo en Rust:
```rust
macro_rules! suma {
    ($x:expr, $y:expr) => {
        $x + $y
    };
}

fn main() {
    let resultado = suma!(5, 5);
    println!("El resultado es: {}", resultado);
}
```

