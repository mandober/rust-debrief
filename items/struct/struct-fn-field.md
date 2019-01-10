# Structs and function pointers

https://dtolnay.github.io/rust-quiz/23

## Struct with a function pointer field
A call that looks like `.f()` always resolves to a method, in this case the inherent method `FnStruct::f`.

If there were no method `f` in scope, a call like this would fail to compile even if a field `f` exists and contains a function pointer.

```rust
struct FnStruct {
  f: fn(),
}

impl FnStruct {
  fn f(&self) {
    println!("1");
  }
}

fn main() {
  let print2 = || print!("2");

  let s = FnStruct { f: print2 };
  s.f();
  // or just:
  FnStruct { f: print2 }.f();
}
```

To call the function pointer stored in field `f`, we would need to write parentheses around the field access:

```rust
fn main() {
  let print2 = || print!("2");
  (FnStruct { f: print2 }.f)();
}
```


## Inherent and a trait method with the same name


```rust
trait Trait {
  fn f(&self);
  fn g(&self);
}

struct S;

impl S {
  fn f(&self) {
    print!("1");
  }

  fn g(&mut self) {
    print!("1");
  }
}

impl Trait for S {
  fn f(&self) {
    print!("2");
  }

  fn g(&self) {
    print!("2");
  }
}

fn main() {
  S.f(); // 1
  S.g(); // 2
    
  Trait::f(&S); // 2
  // or
  <S as Trait>::f(&S); // 2
}
```

`S.f()` calls the inherent method `f`.
If an inherent method and a trait method have the same name and receiver type, plain method call syntax will always *prefer the inherent method*.

The caller would need to use `Trait::f(&S)` or `<S as Trait>::f(&S)` in order to call the trait method.

It is important for macro authors to be aware of this. Macro-generated code typically should not use method call syntax to invoke trait methods on types defined by the user. Those calls could get unintentially hijacked by inherent methods having the same name as the trait method.

On the other hand, `S.g()` calls the trait method `g`. *Auto-ref* during method resolution always prefers making something into `&` over making it into `&mut` where either one would work.

See this Stack Overflow answer for a more detailed explanation of auto-ref during method resolution:

https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules/28552082#28552082
