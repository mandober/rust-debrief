# Trait `std::fmt::Display`
1.0.0
https://doc.rust-lang.org/std/fmt/trait.Display.html

```rust
pub trait Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error>;
}
```
Format trait for an empty format, `{}`.

Display is similar to Debug, but Display is for 
user-facing output, and so cannot be derived.


## Implementing Display on a type

```rust
// EXAMPLE 1:
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let coo = Point { x: 10, y: 20 };
println!("Point coordinates are: {}", coo);
//: Point coordinates are: (10, 20)

// EXAMPLE 2:
struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

assert_eq!("(1.987, 2.983)".to_owned(),
           format!("{}", Position { longitude: 1.987, latitude: 2.983, }));

```


## REQUIRED METHODS

```rust
fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
```
Formats the value using the given formatter.



## IMPLEMENTORS

```rust
impl Display for i8
impl Display for u8
impl Display for i16
impl Display for u16
impl Display for i32
impl Display for u32
impl Display for i64
impl Display for u64
impl Display for i128
impl Display for u128
impl Display for isize
impl Display for usize
impl Display for f32
impl Display for f64
impl Display for bool
impl Display for char
impl Display for str
impl Display for String

impl Display for ToUppercase
impl Display for ToLowercase
impl Display for !

impl Display for AccessError
impl Display for AddrParseError
impl Display for AllocErr
impl Display for BorrowError
impl Display for BorrowMutError
impl Display for CharsError
impl Display for CharTryFromError
impl Display for DecodeUtf16Error
impl Display for ParseBoolError
impl Display for ParseCharError
impl Display for ParseFloatError
impl Display for ParseError
impl Display for ParseIntError
impl Display for FromBytesWithNulError
impl Display for FromUtf8Error
impl Display for FromUtf16Error
impl Display for IntoStringError
impl Display for JoinPathsError
impl Display for NulError
impl Display for RecvTimeoutError
impl Display for RecvError
impl Display for std::fmt::Error
impl Display for std::io::Error
impl Display for StripPrefixError
impl Display for SystemTimeError
impl Display for TryFromIntError
impl Display for TryRecvError
impl Display for Utf8Error
impl Display for VarError

impl<W> Display for IntoInnerError<W>
impl<T> Display for PoisonError<T>
impl<T> Display for SendError<T>
impl<T> Display for TryLockError<T>
impl<T> Display for TrySendError<T>

impl Display for TokenStream
impl Display for TokenTree
impl Display for Literal
impl Display for Utf8Lossy
impl Display for EscapeUnicode
impl Display for EscapeDebug
impl Display for EscapeDefault
impl Display for CannotReallocInPlace
impl Display for IpAddr
impl Display for Ipv4Addr
impl Display for Ipv6Addr
impl Display for SocketAddr
impl Display for SocketAddrV4
impl Display for SocketAddrV6
impl Display for ExitStatus

impl<'a> Display for Display<'a>
impl<'a> Display for Arguments<'a>

impl<'a, T: ?Sized + Display> Display for MutexGuard<'a, T>
impl<'a, T: ?Sized + Display> Display for RwLockReadGuard<'a, T>
impl<'a, T: ?Sized + Display> Display for RwLockWriteGuard<'a, T>

impl<'a, T> Display for Ref<'a, T>      where T: Display + ?Sized,
impl<'a, T> Display for RefMut<'a, T>   where T: Display + ?Sized,
impl<'a, T> Display for &'a T           where T: Display + ?Sized,
impl<'a, T> Display for &'a mut T       where T: Display + ?Sized,

impl<T> Display for Wrapping<T>         where T: Display,
impl<T> Display for Arc<T>              where T: Display + ?Sized,
impl<T> Display for Rc<T>               where T: Display + ?Sized,
impl<T> Display for Box<T>              where T: Display + ?Sized,

impl<'a, B> Display for Cow<'a, B>      where
    B: Display + ToOwned + ?Sized,
    <B as ToOwned>::Owned: Display,
```