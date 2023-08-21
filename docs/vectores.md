# Vectores
Los Vectores son similares a los Arrays, pero con la diferencia de que pueden almacenar distintos 
tipos de datos. En Rust, los Vectores se definen con la macro `vec!` y separando cada dato con una 
coma.

### Ejemplo en Rust:
```rust
fn main() {
    let vector = vec![1, 2, 3, 4, 5];

    println!("{}", vector[0]);
    println!("{}", vector[1]);
    println!("{}", vector[2]);
    println!("{}", vector[3]);
    println!("{}", vector[4]);
}
```

## Consejo
Consejo: En Rust, los Vectores se rigen por la regla de los índices. A cada elemento le 
corresponde un índice y los índices comienzan en cero.
Si tomamos nuestro ejemplo el índice en dicho ejemplo es:
```
0 -> 1
1 -> 2
2 -> 3
3 -> 4
4 -> 5
```

Más información en [**Rust Book: Vectores**](https://rustlanges.github.io/rust-book-es/ch08-01-vectors.html)