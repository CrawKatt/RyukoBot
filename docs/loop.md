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

Más información en [**Rust Book: Loop**](https://rustlanges.github.io/rust-book-es/ch03-05-control-flow.html#repetici%C3%B3n-con-bucles)