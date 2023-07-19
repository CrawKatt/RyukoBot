# Ciclo Loop
El ciclo `loop` es un ciclo infinito que se puede utilizar para ejecutar un bloque de código 
infinitamente.
### Ejemplo en Rust:
```rust
fn main() {
    let mut contador = 0;
    loop {
        contador += 1;
        println!("El valor de contador es: {}", contador);

        if contador == 10 {
            break;
        }
    }
}
```

