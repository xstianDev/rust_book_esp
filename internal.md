#   crate   core

##  module  core::cmp
`cmp(&self, other: &Self) -> core::cmp::Ordering`\
Compara la referencia de sí mismo con la de otro valor.\
Devuelve una enumeración `Ordering`.

##  module  core::str
`split_whitespace(&self) -> SplitWhitespace<'_>`\
Divide un `string slice` donde haya espacios.\
Devuelve un iterador sobre los valores.

##  module  core::slice
`get<I>(&self, index: I) -> Option<&I::Output>`\
Devuelve el valor del índice como `Some` o `None` si no existe.

`iter(&self) -> Iter<'_, T>`\
Devuelve un iterador (itera sobre &mut T).

`iter_mut(&mut self) -> IterMut<'_, T>`\
Devuelve un iterador que permite modificar los valores (itera sobre &mut T).


#   crate   std

##  module  std::cmp

### enum    std::cmp::Ordering
`Ordering::Less` un valor comparado es menor que otro.

`Ordering::Greater` un valor comparado es mayor que otro.

`Ordering::Equal` un valor comparado es igual que otro.


##  module  std::collections::hash_map

### struct  std::collections::hash_map::HashMap
`entry(&mut self, key: K) -> Entry<'_, K, V>`\
Obtiene la entrada correspondiente a la clave `key`.

`insert(&mut self, k: K, v: V) -> Option<V>`\
Introduce la `key` con el `value` asociado dentro del `HashMap`.


### enum    std::collections::hash_map::Entry
`or_insert(self, default: V) -> &'a mut V`\
Asegura que haya un valor en la entrada e introduce uno por defecto si está vacía.\
Devuelve el valor de la entrada.


##  module std::str

### struct std::str::Split
`Split<'a, P>(/* private fields */) where P: Pattern<'a>`\
Un objeto iterador con el contenido de la separación del string.


##  module std::string

### struct std::string::String
`split<'a, P>(&'a self, pat: P) -> Split<'a, P> where P: Pattern<'a>,`\
Devuelve un objeto `Split` con los contenidos del string separados por `pat`.


##  module  std::io
`io::stdin() -> Stdin`\
Maneja input del usuario.\
Devuelve una referencia compartida al buffer global `Stdin`.

### struct  std::io::Stdin
`Stdin::read_line(&self, buf: &mut String) -> io::Result<usize>`\
Añade contenido al buffer sin tomar ownership.\
Devuelve una enumeración `Result<T, E>`:
- `Ok(T)`  valor de éxito.
- `Err(E)` valor de error.


##  module  std::iter
`std::iter(&self) -> Iter<'_, T>`\
Devuelve el objeto iterador actual de tipo `Iter<'_, T>`:
- `'_` lifetime del iterable.
- `T`  tipo del elemento.


`into_iter(self) -> Self::IntoIter`\
Crea un iterador a partir de un valor (itera sobre T).


### trait   std::iter::Iterator
`collect<B>(self) -> B
where
    B: FromIterator<Self::Item>,
    Self: Sized,`\
Transforma un iterador en una colección.

`enumerate(self) -> Enumerate<Self>`\
Crea un iterador que da el número de iteración actual y el siguiente valor.

`filter<P>(self, predicate: P) -> Filter<Self, P>
where
    Self: Sized,
    P: FnMut(&Self::Item) -> bool,`\
Crea un iterator que llama al closure `predicate`.\
Si devuelve `true` la función devolverá el valor.

`map<B, F>(self, f: F) -> Map<Self, F>
where
    Self: Sized,
    F: FnMut(Self::Item) -> B,
{
    Map::new(self, f)
}`\
Crea un iterator que llama al closure `f` en cada iteración.

`skip(self, n: usize) -> Skip<Self>
where
    Self: Sized,`\
Crea un iterator que salta los primeros `n` elementos.

`zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
where
    Self: Sized,
    U: IntoIterator,`\
Convierte los argumentos en un iterador con pares de valores.


##  module  std::option

### enum    std::option::Option
`Option<T>::None`: sin valor (`null` en otros lenguajes).

`Option<T>::Some(T)`: valor de tipo `T`.

`unwrap_or(self, default: T) -> T`\
Devuelve el valor de `T` o un valor por defecto.


##  module  std::result

### enum    std::result::Result
`Result::expect(self, msg: &str) -> T`\
Si `T` es `Ok`, devuelve el valor contenido en `OK`.\
Si `T` es `Err`, muestra un mensaje y el contenido de `Err`.



#   crate   alloc

##  module  alloc::string

### struct  alloc::string::String
`String::new()`\
Crea un `String` vacío.

`as_bytes(&self) -> &[u8]`\
Devuelve el contenido del `String` como bytes.

`clear()`\
Trunca el `String`, borrando su contenido.\
Su `length` será 0, pero su `capacity` queda intacto.

`clone(&self) -> Self`\
Clona el valor del `String` sin tomar ownership de la referencia.

`from(s: &str) -> String`\
Convierte `&str` a `String`. El resultado se asigna en el heap.

`len(&self) -> usize`\
Devuelve la longitud del `String`.

`parse<F: FromStr>(&self) -> Result<F, F::Err>`\
Convierte el `String` a otro tipo de dato que hay que indicar.\
Devuelve un `Result<T, E>`.

`push(&mut self, ch: char)`\
Añade un carácter al final del `String`.

`push_str(&mut self, string: &str)`\
Añade un substring al final del `String`.

`trim(&self) -> &str`\
Quita los espacios al comienzo y al final del string.


##  module  alloc::vec

### struct  alloc::vec::Vec
`Vec<T, A = Global>`
Struct que define un vector.

`Vec::new()`
Crea un vector vacío con el tipo especificado.

`get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>`\
Devuelve la referencia al elemento del índice indicado o `None` si no existe.

`push(&mut self, value: T)`\
Añade un elemento al final del vector.