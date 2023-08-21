# Return
La palabra clave return se utiliza para devolver un valor de una función.
### Ejemplo en Rust:
```rust
fn main() {
    let resultado = sumar(5, 5);
    println!("El resultado es: {}", resultado);
}

fn sumar(x: i32, y: i32) -> i32 {
    return x + y;
}
```

Más información en [**Rust Book: Funciones**](https://rustlanges.github.io/rust-book-es/ch03-03-how-functions-work.html#funciones-con-valores-de-retorno)