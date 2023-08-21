# Ciclo For
En Rust, el ciclo `for` nos permitirá iterar sobre una colección de datos. Ya sea un `Vector`, un 
`Array`, una `Tupla`, etc.
El ciclo `for` funcionará como un `for each` en otros lenguajes de programación.
### Ejemplo en Rust:
```rust
fn main() {
    let numeros: [i32; 5] = vec![1, 2, 3, 4, 5];
    for numero in numeros {
        println!("El valor de número {:?}", numero);
    }
}
```
Ejemplo de algoritmo Fizz Buzz utilizando el ciclo `for` en Rust:
```rust
fn main() {
    for numero in 1..101 {
        if numero % 15 == 0 {
            println!("FizzBuzz");
        } else if numero % 3 == 0 {
            println!("Fizz");
        } else if numero % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", numero);
        }
    }
}
```

Más información en [**Rust Book: Ciclo For**](https://rustlanges.github.io/rust-book-es/ch03-05-control-flow.html#bucle-a-trav%C3%A9s-de-una-colecci%C3%B3n-con-for)