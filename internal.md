#   crate   core

##  module  core::cmp
```rust
cmp(&self, other: &Self) -> core::cmp::Ordering
// Compara la referencia de sí mismo con la de otro valor.
// Devuelve una enumeración `Ordering`.
```

##  module  core::str
```rust
split_whitespace(&self) -> SplitWhitespace<'_>
// Divide un `str` donde haya espacios. Devuelve un iterador sobre los valores.
```

##  module  core::slice
```rust
get<I>(&self, index: I) -> Option<&I::Output>
// Devuelve el valor del índice como `Some` o `None` si no existe.

iter(&self) -> Iter<'_, T>
// Devuelve un iterador (itera sobre &mut T).

iter_mut(&mut self) -> IterMut<'_, T>
// Devuelve un iterador que permite modificar los valores (itera sobre &mut T).
```



#   crate   std

##  module  std::cell

### struct  std::cell::RefCell
```rust
borrow(&self) -> Ref<'_, T>
where
    T: ?Sized,
// Hace borrow inmutable del valor que envuelve hasta que acaba el scope.
// Varios borrows inmutables se pueden hacer a la vez.

borrow_mut(&self) -> RefMut<'_, T>
where
    T: ?Sized,
// Hace borrow mutable del valor que envuelve hasta que acaba el scope.
// No se puede hacer borrow del valor si ya hay un borrow activo.
```


##  module  std::cmp

### enum    std::cmp::Ordering
```rust
Ordering::Less      // valor menor que el comparado
Ordering::Greater   // valor mayor que el comparado
Ordering::Equal     // valor igual que el comparado
```


##  module  std::collections::hash_map

### struct  std::collections::hash_map::HashMap
```rust
entry(&mut self, key: K) -> Entry<'_, K, V>
// Obtiene la entrada correspondiente a la clave `key`.

insert(&mut self, k: K, v: V) -> Option<V>
// Introduce la `key` con el `value` asociado dentro del `HashMap`.
```

### enum    std::collections::hash_map::Entry
```rust
or_insert(self, default: V) -> &'a mut V
// Asegura que haya un valor en la entrada o introduce uno por defecto.
// Devuelve el valor de la entrada.
```


##  module  std::io
```rust
io::stdin() -> Stdin
// Maneja input del usuario.
// Devuelve una referencia compartida al buffer global `Stdin`.
```

### struct  std::io::Stdin
```rust
read_line(&self, buf: &mut String) -> io::Result<usize>
// Añade contenido al buffer sin tomar ownership.
```


##  module  std::iter
```rust
iter(&self) -> Iter<'_, T>
// Devuelve el objeto iterador actual.

into_iter(self) -> Self::IntoIter
// Crea un iterador a partir de un valor (itera sobre T).
```

### trait   std::iter::Iterator
```rust
collect<B>(self) -> B
where
    B: FromIterator<Self::Item>,
    Self: Sized,
// Transforma un iterador en una colección.

enumerate(self) -> Enumerate<Self>
// Crea un iterador que da el número de iteración actual y el siguiente valor.

filter<P>(self, predicate: P) -> Filter<Self, P>
where
    Self: Sized,
    P: FnMut(&Self::Item) -> bool,
// Crea un iterator que llama al closure `predicate`.
// Si devuelve `true`, la función devolverá el valor.

map<B, F>(self, f: F) -> Map<Self, F>
where
    Self: Sized,
    F: FnMut(Self::Item) -> B,
{
    Map::new(self, f)
}
// Crea un iterator que llama al closure `f` en cada iteración.

skip(self, n: usize) -> Skip<Self>
where
    Self: Sized,
// Crea un iterator que salta los primeros `n` elementos.

zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
where
    Self: Sized,
    U: IntoIterator,
// Convierte los argumentos en un iterador con pares de valores.
```


##  module  std::mem
```rust
drop<T>(_x: T)
// Elimina un valor que implementa el trait `Drop`.
```


##  module  std::option

### enum    std::option::Option
```rust
Option<T>::None     // sin valor (`null` en otros lenguajes)
Option<T>::Some(T)  // valor de tipo `T`
```

```rust
unwrap_or(self, default: T) -> T
// Devuelve el valor de `T` o un valor por defecto
```


##  module  std::rc

### struct  std::rc::Rc
```rust
clone(&self) -> Rc<T, A>
// Clona el puntero `Rc` con las asignaciones
// Incrementa el número de referencias

downgrade(this: &Rc<T, A>) -> Weak<T, A>
where
    A: Clone,
// Crea un puntero `Weak` donde está alojado `this`

strong_count(&self) -> usize
// Devuelve el número de punteros `Rc` que apuntan a esta asignación

weak_count(&self) -> usize`
// Devuelve el número de punteros `Weak` que apuntan a esta asignación
```

### struct  std::rc::Weak
```rust
upgrade(&self) -> Option<Rc<T, A>>
where
    A: Clone,
// Mejora el puntero `Weak` a `Rc`, retrasando que se borre su valor
// Devuelve `None` si el valor interno se ha borrado
```


##  module  std::result

### enum    std::result::Result
```rust
expect(self, msg: &str) -> T
// Devuelve el valor contenido en `T` (`Ok` o `Err`)
```


##  module  std::str

### struct  std::str::Split
```rust
Split<'a, P>(/* private fields */)
where
    P: Pattern<'a>
// Objeto iterador con el contenido de la separación del string
```


##  module  std::string

### struct  std::string::String
```rust
split<'a, P>(&'a self, pat: P) -> Split<'a, P>
where
    P: Pattern<'a>,
// Devuelve un objeto `Split` con los contenidos del string separados por `pat`
```


##  module  std::sync

### struct  std::sync::Mutex
```rust
lock(&self) -> LockResult<MutexGuard<'_, T>>
where
    T: ?Sized,
// Devuelve un smart pointer `MutexGuard` cuyo `Deref` trait apunta al valor de `self`.
// El `Drop` trait de `MutexGuard` hace que se libere el lock.
```


### module  std::sync::mpsc
```rust
channel<T>() -> (Sender<T>, Receiver<T>)
// Crea un canal asíncrono y devuelve las mitades `Sender` y `Receiver`
```

### struct  std::sync::mpsc::Receiver
```rust
recv(&self) -> Result<T, RecvError>
// Bloquea el hilo principal mientras espera que un mensaje se mande en el canal
// Devuelve un `Result` cuando obtiene la respuesta

try_recv(&self) -> Result<T, TryRecvError>
// No bloquea el hilo principal mientras espera un mensaje, sino que devuelve un `Result` inmediatamente
```

### struct  std::sync::mpsc::Sender
```rust
send(&self, t: T) -> Result<(), SendError<T>>
// Envía un mensaje que será recibido por el `Receiver`
```


##  module  std::thread
```rust
sleep(dur: Duration)
// Pausa el hilo actual durante la duración especificada

spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
// Crea un nuevo hilo
```


##  module  std::time

### struct std::time::Duration
```rust
from_millis(millis: u64) -> Duration
// Crea un objeto `Duration` con el tiempo especificado
```



#   crate   alloc

##  module  alloc::string

### struct  alloc::string::String
```rust
String::new()
// Crea un `String` vacío

as_bytes(&self) -> &[u8]
// Devuelve el contenido del `String` como bytes

clear()
// Trunca el `String`, borrando su contenido
// Su `length` será 0, pero su `capacity` queda intacto

clone(&self) -> Self
// Clona el valor del `String` sin tomar ownership de la referencia

from(s: &str) -> String
// Convierte `&str` a `String`
// El resultado se asigna en el heap

len(&self) -> usize
// Devuelve la longitud del `String`

parse<F: FromStr>(&self) -> Result<F, F::Err>
// Convierte el `String` a otro tipo de dato que hay que indicar

push(&mut self, ch: char)
// Añade un carácter al final del `String`

push_str(&mut self, string: &str)
// Añade un substring al final del `String`

trim(&self) -> &str
// Quita los espacios al comienzo y al final del string
```


##  module  alloc::vec

### struct  alloc::vec::Vec
```rust
Vec<T, A = Global>`
// Struct que define un vector

Vec::new()`
// Crea un vector vacío con el tipo especificado

get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
// Devuelve la referencia al elemento del índice indicado o `None` si no existe

push(&mut self, value: T)
// Añade un elemento al final del vector
```