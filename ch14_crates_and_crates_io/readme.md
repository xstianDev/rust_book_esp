# Publicar una crate

1. En la página de [Crates.io](https://crates.io) hacemos login con GitHub.
2. Verificamos el correo
3. Después, vamos al Menu > [Account settings](https://crates.io/me), y desde ahí a API Access > New Token
4. Asignamos nombre del token y creamos. Devuelve `cargo login [token]` (no compartir token)
5. Pegar comando en la terminal para hacer login a Crates.io
6. Se comprueba la metadata en Cargo.toml
   - `name` debe ser único en Crates.io
   - `description` descripción del proyecto
   - `license` licencia del proyecto
7. Publicar con `cargo publish`

Los proyectos subidos a Crates.io no se pueden borrar, ya que otros pueden depender de él.

Para eliminar una versión se usa `cargo yank --vers [version]`.
Esto solo permite descargar la versión a los usuarios que la tenían ya en su proyecto.

Para deshacer el cambio se usa `cargo yank --vers [version] --undo`.