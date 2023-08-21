# Tuplas
Las Tuplas son similares a los Arrays, pero con la diferencia de que pueden almacenar distintos
tipos de datos. En Rust, las tuplas se definen con paréntesis y separando cada dato con una coma
### Ejemplo en Rust:
```rust
fn main() {
    let tupla = (1, "Hola", true);

    println!("{}", tupla.0);
    println!("{}", tupla.1);
    println!("{}", tupla.2);
}
```
## Consejo
En Rust, las Tuplas se rigen por la regla de los índices. A cada elemento le corresponde un índice
y los índices empiezan desde el 0.
Si tomamos nuestro ejemplo el índice en dicho ejemplo es:
```
0 -> 1
1 -> 2
2 -> 3
3 -> 4
4 -> 5
```
Más información en [**Rust Book: Tuplas**](https://rustlanges.github.io/rust-book-es/ch03-02-data-types.html#el-tipo-tupla)