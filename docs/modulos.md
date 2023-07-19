# Módulos
Los módulos son una característica de Rust que nos permiten organizar nuestro código en archivos 
separados.
### Ejemplo en Rust:
```rust
mod modulo {
    pub fn saludar() {
        println!("Hola!");
    }
}

fn main() {
    modulo::saludar();
}
```

