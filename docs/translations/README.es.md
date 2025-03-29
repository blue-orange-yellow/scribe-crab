# Scribe Crab

Un servidor MCP que genera comentarios de documentación para funciones Rust.

## Descripción general

Scribe Crab es un servidor MCP (Model Context Protocol) que ayuda a los desarrolladores a generar comentarios de documentación para sus funciones Rust. Toma el código de una función Rust como entrada y produce comentarios de documentación siguiendo un formato personalizable.

## Características

- Genera comentarios de documentación para funciones Rust
- Utiliza plantillas de formato personalizables
- Se integra con clientes MCP (como el IDE Cursor)

## Instalación

Clona el repositorio y compila:

```bash
git clone https://github.com/blue-orange-yellow/scribe-crab.git
cd scribe-crab
cargo build --release
```

## Configuración

Para usar con clientes como Cursor, necesitas configurarlo como servidor MCP. Ejemplo de configuración en Cursor:

```json
{
  "mcpServers": {
    "scribe-crab": {
      "command": "/path/to/scribe-crab/target/release/scribe-crab",
      "args": [],
      "cwd": "/path/to/scribe-crab",
      "env": {
        "FORMAT_PATH": "/path/to/scribe-crab/.format.md"
      }
    }
  }
}
```

## Uso

1. Establece la ruta del archivo de formato como una variable de entorno
2. Inicia el servidor MCP
3. Utiliza la herramienta a través de un cliente MCP (como Cursor)
4. Pide al Agente de Cursor "Genera un comentario de documentación para la función XX" o instrucciones similares

## Formato de documentación

El formato de documentación se puede personalizar editando el archivo `.format.md`.
Ejemplo:

```rust
/// # Description
/// 
/// This function does XYZ.
/// 
/// # Arguments
/// 
/// * - Description of the first parameter
/// * - Description of the second parameter
/// 
/// # Returns
/// 
/// Description of the return value
```

## Idiomas

Este README también está disponible en:
- [English](../../README.md)
- [日本語](README.ja.md)
- [中文](README.zh.md)

## Licencia

MIT

## Contribuciones

¡Las contribuciones son bienvenidas! No dudes en enviar un Pull Request. 