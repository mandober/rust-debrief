# List of Rust's types

## Primitives
- `bool`  boolean
- `i8`    8-bit signed integer type.
- `u8`    8-bit unsigned integer type.
- `i16`   16-bit signed integer type.
- `u16`   16-bit unsigned integer type.
- `i32`   32-bit signed integer type.
- `u32`   32-bit unsigned integer type.
- `i64`   64-bit signed integer type.
- `u64`   64-bit unsigned integer type.
- `i128`  128-bit signed integer type. <kbd>LAB</kbd>
- `u128`  The 128-bit unsigned integer type. <kbd>LAB</kbd>
- `isize` pointer-sized signed integer type.
- `usize` pointer-sized unsigned integer type.
- `f32`   32-bit floating point type.
- `f64`   64-bit floating point type.
- `char`  character
- array: a fixed-size array, denoted [T; N]
- tuple: a finite heterogeneous sequence, (T, U, ..).
- unit: the unit type, `()`.
- `fn` function pointers, like `fn(usize) -> bool`.
- references: shared and mutable references, `&T` and `&mut T`
- raw pointers: unsafe pointers, `*const T` and `*mut T`.
- slice, `[T]`, always seen in the borrowed form, `&[T]`
- string slice, `str`, always seen in the borrowed form, `&str`
- never: the never type, `!`. <kbd>LAB</kbd>

## Smart pointers
- `String`
- `Box<T>`
- `Vec<T>`
- `Rc<T>`
- `Arc<T>`
- `Cell<T>`
- `RefCell<T>`
- `Mutex<T>`
- `RwLock<T>`

## Common
- `Option<T>`
- `Result<T, E>`
- `Range<T>`
- `PhantomData<T>`
- `Cow<'a, T>`
- `Duration`
- `SystemTime`
- `Path`
- `PathBuf`
- `NonZero<T>` <kbd>LAB</kbd>

## Collections
- `BTreeMap<K, V>`
- `BTreeSet<T>`
- `BinaryHeap<T>`
- `HashMap<K, V, H>`
- `HashSet<T, H>`
- `LinkedList<T>`
- `VecDeque<T>`
- `EnumSet<T>` <kbd>LAB</kbd>

## FFI types:
- `CStr`
- `CString`
- `OsStr`
- `OsString`

## Net
- `IpAddr`
- `Ipv4Addr`
- `Ipv6Addr`
- `SocketAddr`
- `SocketAddrV4`
- `SocketAddrV6`