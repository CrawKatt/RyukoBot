# Lifetimes
Los Lifetimes son una característica de Rust que nos permiten especificar la duración de una 
referencia.

### Ejemplo en Rust:
```rust
fn main() {
    let string1 = String::from("Hola");
    let resultado;

    {
        let string2 = String::from("mundo");
        resultado = combinar_strings(&string1, &string2);
        println!("El resultado es: {}", resultado);
    } // string2 se sale del scope aquí

    // Esto causaría un error de tiempo de compilación porque string2 ya no es válido
    // println!("El resultado es: {}", resultado);
}

fn combinar_strings<'a>(s1: &'a str, s2: &'a str) -> String {
    let resultado = format!("\\{s1} \\{s2}\\");
    resultado
}
```
En este ejemplo, tenemos una función combinar_strings que toma dos referencias a Strings (&str).
La anotación <'a> indica que combinar_strings tiene un lifetime 'a, lo que significa que las
referencias s1 y s2 deben vivir al menos durante el tiempo de vida 'a.

Al intentar imprimir el resultado fuera del bloque de código de string2, obtendrías un error de
tiempo de compilación porque resultado hace referencia a string2, que ya no es válido.

Más información en [**Rust Book: Lifetimes**](https://rustlanges.github.io/rust-book-es/ch10-03-lifetime-syntax.html)