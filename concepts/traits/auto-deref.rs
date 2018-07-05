// https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules/28552082#28552082
// Rust automatically dereferences objects when making method calls.
// I made some tests to determine the exact behaviour:

struct X {
    val: i32
}

impl std::ops::Deref for X {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.val
    }
}


trait            M                   { fn m(self); }
impl             M for i32           { fn m(self) { println!("i32::m()"); } }
impl             M for X             { fn m(self) { println!("X::m()"); } }
impl<'a>         M for &'a X         { fn m(self) { println!("&X::m()"); } }
impl<'a, 'b>     M for &'a &'b X     { fn m(self) { println!("&&X::m()"); } }
impl<'a, 'b, 'c> M for &'a &'b &'c X { fn m(self) { println!("&&&X::m()"); } }

trait            RefM                   { fn refm(&self); }
impl             RefM for i32           { fn refm(&self) { println!("i32::refm()"); } }
impl             RefM for X             { fn refm(&self) { println!("X::refm()"); } }
impl<'a>         RefM for &'a X         { fn refm(&self) { println!("&X::refm()"); } }
impl<'a, 'b>     RefM for &'a &'b X     { fn refm(&self) { println!("&&X::refm()"); } }
impl<'a, 'b, 'c> RefM for &'a &'b &'c X { fn refm(&self) { println!("&&&X::refm()"); } }


struct Y { val: i32 }

impl std::ops::Deref for Y {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.val
    }
}

struct Z { val: Y }

impl std::ops::Deref for Z {
    type Target = Y;
    fn deref(&self) -> &Y {
        &self.val
    }
}


struct A;

impl std::marker::Copy for A {}

impl             M for             A { fn m(self) { println!("A::m()"); } }

impl<'a, 'b, 'c> M for &'a &'b &'c A { fn m(self) { println!("&&&A::m()"); } }

impl             RefM for             A { fn refm(&self) { println!("A::refm()"); } }

impl<'a, 'b, 'c> RefM for &'a &'b &'c A { fn refm(&self) { println!("&&&A::refm()"); } }


fn main() {
    // I'll use @ to denote left side of the dot operator
    (*X{val:42}).m();        // i32::refm() , self == @
    X{val:42}.m();           // X::m()      , self == @
    (&X{val:42}).m();        // &X::m()     , self == @
    (&&X{val:42}).m();       // &&X::m()    , self == @
    (&&&X{val:42}).m();      // &&&X:m()    , self == @
    (&&&&X{val:42}).m();     // &&&X::m()   , self == *@
    (&&&&&X{val:42}).m();    // &&&X::m()   , self == **@

    (*X{val:42}).refm();     // i32::refm() , self == @
    X{val:42}.refm();        // X::refm()   , self == @
    (&X{val:42}).refm();     // X::refm()   , self == *@
    (&&X{val:42}).refm();    // &X::refm()  , self == *@
    (&&&X{val:42}).refm();   // &&X::refm() , self == *@
    (&&&&X{val:42}).refm();  // &&&X::refm(), self == *@
    (&&&&&X{val:42}).refm(); // &&&X::refm(), self == **@

    Y{val:42}.refm();        // i32::refm() , self == *@
    Z{val:Y{val:42}}.refm(); // i32::refm() , self == **@

    A.m();                   // A::m()      , self == @
    // without the Copy trait, (&A).m() would be a compilation error:
    // cannot move out of borrowed content
    (&A).m();                // A::m()      , self == *@
    (&&A).m();               // &&&A::m()   , self == &@
    (&&&A).m();              // &&&A::m()   , self == @
    A.refm();                // A::refm()   , self == @
    (&A).refm();             // A::refm()   , self == *@
    (&&A).refm();            // A::refm()   , self == **@
    (&&&A).refm();           // &&&A::refm(), self == @
}
/**
So, it seems that, more or less:
- compiler will insert as many dereference operators as needed to invoke a method.
- compiler, when resolving methods declared using &self (call-by-reference):
  1. tries calling for a single dereference of self
  2. tries calling for the exact type of self
  3. tries inserting as many dereference operators as necessary for a match
- Methods declared using self (call-by-value) for type T behave as if they were
  declared using &self (call-by-reference) for type &T and called on the
  reference to whatever is on the left side of the dot operator.

The above rules are first tried with raw built-in dereferencing, and if there's
no match, the overload with Deref trait is used.

*/