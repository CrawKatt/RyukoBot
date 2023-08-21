# Iterators
Los Iterators son una característica de Rust que nos permiten recorrer una colección de datos de 
forma secuencial.

### Ejemplo en Rust:
```rust
fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
        for numero in numeros.iter() {
            println!("El valor de número {:?}", numero);
        }
}
```

Más información en [**Rust Book: Iterators**](https://rustlanges.github.io/rust-book-es/ch13-02-iterators.html)