
pub const HELP_MESSAGE: &str = "
Comandos Disponibles:
```
$ping: Responde con un pong!
$variables: Explica el uso de variables en Rust
```
";

pub const VARIABLES: &str = "
# Variables
Son un espacio en memoria que contiene un valor
### Ejemplo en Rust
```rust
fn main() {
    let variable = 10;
}
```
En este ejemplo, utilizamos `let` para definir una variable, le damos un nombre que es **variable**, \
le asignamos el valor de **10** y finalizamos la sentencia con un punto y coma `;`
";

pub const ARRAYS: &str = "
# Array
En Rust, los arrays son estructuras de datos que almacenan un número fijo de elementos del mismo tipo.

### Ejemplo en Rust:

```rust
let array = [1, 2, 3, 4, 5];
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
";

pub const BORROWING: &str = "\
# Borrowing
En Rust, el Borrowing es una característica que permite prestar una referencia a una variable en \
lugar de tomar la propiedad de ella. Esto significa que puedes utilizar la variable sin tener que \
transferir la propiedad de la misma, lo que te permite usarla tanto dentro como fuera de la función \
donde la prestaste.
### Ejemplo en Rust:
```rust
fn main() {
    let mut hola = String::from(\"Hola\");
    cambiar(&mut hola);
    println!(\"{}\", hola);
}

fn cambiar(algun_string : &mut String) {
    algun_string.push_str(\", mundo!\");
}
```
En este ejemplo, estamos prestando una referencia mutable a la variable `hola` al llamar a la \
función `cambiar()`
";

pub const CLOSURES: &str = "
# Closures
Los Closures son funciones anónimas que se pueden almacenar en variables o pasar como argumentos a otras funciones
### Ejemplo en Rust:
```rust
let suma = |a: i32, b: i32| -> i32 {
    a + b
 };

fn main() {
    let resultado = suma(5,5);
    println!(\"{}\", resultado);
}
```
";

pub const CONDICIONALES: &str = "
# Condicionales
Son grupos de sentencias o sentencias individuales que te permiten condicionar la decisión entre \
la elección de una opción y otra.
### Ejemplo en Rust:
```rust
fn main() {
    let color = \"verde\";
        if color == \"rojo\" {
            println!(\"El color es rojo\");
        } else if color == \"azul\" {
            println!(\"El color es azul\");
        } else {
            println!(\"El color es otro\");
        }
}
```
";

pub const CONSTANTES: &str = "
# Constantes
Son un tipo especial de variable que no permite que su valor cambie durante la ejecución del programa.
### Ejemplo en Rust:
```rust
fn main() {
    const PI: f32 = 3.141592;
}
A diferencia de las variables, el nombre de las constantes se escribe en mayúsculas y su tipo de \
dato debe ser especificado.
```
";

pub const ENUMS: &str = "
# Enums
Un Enum es un tipo de dato especial que almacena diferentes variantes, almacena diferentes opciones.
### Ejemplo en Rust:
```rust
enum Opcion {
    Opcion1,
    Opcion2,
    Opcion3,
}
```
";

pub const FOR: &str = "
# Ciclo For
En Rust, el ciclo `for` nos permitirá iterar sobre una colección de datos. Ya sea un `Vector`, un \
`Array`, una `Tupla`, etc.
El ciclo `for` funcionará como un `for each` en otros lenguajes de programación.
### Ejemplo en Rust:
```rust
fn main() {
    let numeros: [i32; 5] = vec![1, 2, 3, 4, 5];
    for numero in numeros {
        println!(\"El valor de número {:?}\", numero);
    }
}
```
Ejemplo de algoritmo Fizz Buzz utilizando el ciclo `for` en Rust:
```rust
fn main() {
    for numero in 1..101 {
        if numero % 15 == 0 {
            println!(\"FizzBuzz\");
        } else if numero % 3 == 0 {
            println!(\"Fizz\");
        } else if numero % 5 == 0 {
            println!(\"Buzz\");
        } else {
            println!(\"{}\", numero);
        }
    }
}
```
";

pub const FUNCIONES: &str = "
# Funciones
Las funciones son bloques de código que se pueden reutilizar en diferentes partes de nuestro programa.
### Ejemplo en Rust:
```rust
fn saludar(nombre: &str) {
   println!(\"Hola, {}!\", nombre);
}

fn main() {
    saludar(\"Juan\");
}
```
";

pub const GENERICS: &str = "
# Generics
Los Generics son una característica de Rust que nos permiten crear tipos de datos genéricos, que \
pueden ser de cualquier tipo.
### Ejemplo en Rust:
```rust
struct Generic<T> {
    valor: T,
}

fn main() {
    let entero = Generico { valor: 5 };
    let flotante = Generico { valor: 5.0 };
}
```
";

pub const IF_LET: &str = "
# If let
If let se utiliza para desempaquetar y hacer coincidir un valor de tipo Option<T> o Result de forma \
concisa.
### Ejemplo en Rust:
```rust
fn main() {
    let option_value: Option<i32> = Some(5);

    if let Some(x) = option_value {
        println!(\"El valor de x es: {}\", x);
    } else {
        println!(\"La opción no contiene un valor.\");
    }
}
```
";

pub const ITERATORS: &str = "
# Iterators
Los Iterators son una característica de Rust que nos permiten recorrer una colección de datos de \
forma secuencial.
### Ejemplo en Rust:
```rust
fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
        for numero in numeros.iter() {
            println!(\"El valor de número {:?}\", numero);
        }
}
```
";

pub const LIFETIMES: &str ="
# Lifetimes
Los Lifetimes son una característica de Rust que nos permiten especificar la duración de una \
referencia.
### Ejemplo en Rust:
```rust
fn main() {
    let string1 = String::from(\"Hola\");
    let resultado;

    {
        let string2 = String::from(\"mundo\");
        resultado = combinar_strings(&string1, &string2);
        println!(\"El resultado es: {}\", resultado);
    } // string2 se sale del scope aquí

    // Esto causaría un error de tiempo de compilación porque string2 ya no es válido
    // println!(\"El resultado es: {}\", resultado);
}

fn combinar_strings<'a>(s1: &'a str, s2: &'a str) -> String {
    let resultado = format!(\"{} {}\", s1, s2);
    resultado
}
```
En este ejemplo, tenemos una función combinar_strings que toma dos referencias a Strings (&str).
La anotación <'a> indica que combinar_strings tiene un lifetime 'a, lo que significa que las
referencias s1 y s2 deben vivir al menos durante el tiempo de vida 'a.

Al intentar imprimir el resultado fuera del bloque de código de string2, obtendrías un error de
tiempo de compilación porque resultado hace referencia a string2, que ya no es válido.
";

pub const LOOP: &str ="
# Ciclo Loop
El ciclo `loop` es un ciclo infinito que se puede utilizar para ejecutar un bloque de código \
infinitamente.
### Ejemplo en Rust:
```rust
fn main() {
    let mut contador = 0;
    loop {
        contador += 1;
        println!(\"El valor de contador es: {}\", contador);
    }
}
```
";

pub const MACROS: &str = "
# Macros
Las macros son una característica de Rust que nos permiten escribir código que genera código.
### Ejemplo en Rust:
```rust
macro_rules! suma {
    ($x:expr, $y:expr) => {
        $x + $y
    };
}

fn main() {
    let resultado = suma!(5, 5);
    println!(\"El resultado es: {}\", resultado);
}
```
";

pub const MATCH: &str = "
# Match
Match es una expresión que nos permite comparar un valor con una serie de patrones y ejecutar un \
bloque de código dependiendo de si el patrón coincide o no.
### Ejemplo en Rust:
```rust
fn main() {
    let numero = 5;
    match numero {
        1 => println!(\"El número es 1\"),
        2 => println!(\"El número es 2\"),
        3 => println!(\"El número es 3\"),
        4 => println!(\"El número es 4\"),
        5 => println!(\"El número es 5\"),
        _ => println!(\"El número no es 1, 2, 3, 4 o 5\"),
    }
}
```
";

pub const METODOS: &str = "
# Métodos
Los métodos son funciones que se definen dentro del contexto de un struct, enum o \
implementación de un trait.
### Ejemplo en Rust:
```rust
struct Persona {
    nombre: String,
    edad: u8,
}

impl Persona {
    fn saludar(&self) {
        println!(\"Hola, mi nombre es {} y tengo {} años.\", self.nombre, self.edad);
    }
}

fn main() {
    let juan = Persona {
        nombre: String::from(\"Juan\"),
        edad: 25,
    };

    juan.saludar();
}
```
";

pub const MODULOS:&str = "
# Módulos
Los módulos son una característica de Rust que nos permiten organizar nuestro código en archivos \
separados.
### Ejemplo en Rust:
```rust
mod modulo {
    pub fn saludar() {
        println!(\"Hola!\");
    }
}

fn main() {
    modulo::saludar();
}
```
";

pub const OPERADORES: &str = "
# Operadores
En programación, tenemos distintos tipos de operadores para manejar datos en nuestras variables.
### Entre estos están:
```
// Los Operadores Básicos:
+   // Suma
-   // Resta
*   // Multiplicación
/   // División
%   // División (Con resto/residuo)

// Los Operadores Relacionales:
>   // Mayor que
<   // Menor que
>=  // Mayor o igual que
<=  // Menor o igual que
==  // Igual
!=  // Diferente de

// Los Operadores Lógicos
||  // Or (o)
&&  // And (y)
```
En Rust, los operadores básicos son los mismos que en la mayoría de los lenguajes de programación, \
y se utilizan para realizar operaciones aritméticas y lógicas básicas.
";

pub const OPTION: &str = "
# Option
Option es un tipo de dato genérico que nos permite representar la posibilidad de que un valor \
esté presente o no.
### Ejemplo en Rust:
```rust
fn main() {
    let numero: Option<i32> = Some(5);

    match numero {
        Some(x) => println!(\"El valor de x es: {}\", x),
        None => println!(\"La opción no contiene un valor.\"),
    }
}
```
## Consejo
En Rust, no existe el `Null`, por lo que Option es una forma de representar la posibilidad de que \
un valor no esté presente.
";

pub const OWNERSHIP: &str = "
# Ownership
Ownership es una característica de Rust que nos permite manejar la memoria de forma segura.
### Ejemplo en Rust:
```rust
fn main() {
    let s = String::from(\"Hola\"); // s entra en el scope

    tomar_prestado(&s); // s se presta a la función tomar_prestado

    println!(\"{}\", s); // s se imprime aquí, pero ya no es válida
} // s sale del scope y se libera de la memoria

fn tomar_prestado(s: &String) {
    println!(\"{}\", s);
} // s sale del scope pero no se libera de la memoria
```
En este ejemplo, tenemos una función tomar_prestado que toma una referencia a un String (&String).
La referencia s se presta a la función tomar_prestado, pero no se libera de la memoria cuando sale \
del scope, ya que no es el dueño del valor.
";