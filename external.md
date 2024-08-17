#  crate    colored

## trait    colored::Colorize
```rust
green() -> ColoredString
// Convierte un texto al color verde.

red() -> ColoredString
// Convierte un texto al color rojo.
```


#  crate    rand
```rust
thread_rng() -> ThreadRng
// Inicializa un hilo que genera un número aleatorio
// Se usa en cadena con otros métodos.
```

## trait    rand::Rng
```rust
gen_range(&mut self, low: T, high: T) -> T
// Devuelve un valor aleatorio entre dos números de tipo `T`
// En la generación incluye `low` pero excluye `min`.
```


#  crate    unicode_segmentation

## trait    unicode_segmentation::UnicodeSegmentation
```rust
graphemes<'a>(&'a self, is_extended: bool) -> Graphemes<'a>
// Itera los grapheme clusters de una cadena de texto.
```