# Array
En Rust, los arrays son estructuras de datos que almacenan un número fijo de elementos del mismo tipo.

### Ejemplo en Rust:

```rust
fn main() {
    let array = [1, 2, 3, 4, 5];
    println!("{}", array[0]); // 1
}
```
## Consejo
En Rust, los Arrays se rigen por la regla de los índices. A cada elemento le corresponde un índice y los índices comienzan en cero.
Si tomamos nuestro ejemplo el índice en dicho ejemplo es:
```
0 -> 1
1 -> 2
2 -> 3
3 -> 4
4 -> 5
```

Más información en [**Rust Book: Arrays**](https://phosphorus-m.github.io/rust-book-es/ch03-02-data-types.html#el-tipo-arreglo)