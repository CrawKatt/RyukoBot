# Ownership
Ownership es una característica de Rust que nos permite manejar la memoria de forma segura.
### Ejemplo en Rust:
```rust
fn main() {
    let s = String::from("Hola"); // s entra en el scope

    tomar_prestado(&s); // s se presta a la función tomar_prestado

    println!("{}", s); // s se imprime aquí, pero ya no es válida
} // s sale del scope y se libera de la memoria

fn tomar_prestado(s: &String) {
    println!("{}", s);
} // s sale del scope pero no se libera de la memoria
```
En este ejemplo, tenemos una función tomar_prestado que toma una referencia a un String (&String).
La referencia s se presta a la función tomar_prestado, pero no se libera de la memoria cuando sale \
del scope, ya que no es el dueño del valor.

Más información en [**Rust Book: Ownership**](https://rustlanges.github.io/rust-book-es/ch04-00-understanding-ownership.html)