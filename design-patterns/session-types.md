# Session Types

2017/02/16
https://leotindall.com/tutorial/session-types/

Session types are a technique for using a type system to prevent the 
representation of certain kinds of illegal states.

## Use-case

Example: a system representing packaging and shipping boxes.
Create a `Package` datastructure, pack data into it, close it (preventing adding 
data), address it, and then ship it. It makes no sense to send an un-addressed 
Package, or to insert data into a closed one. We could represent this like this:

```rust
struct Package {
    is_closed: bool,
    is_addressed: bool,
    data: T
}
```

We would then have lots of runtime typechecking of whether boxes submitted for 
shipping were addressed and such, which costs time and is error-prone. 
Wouldn’t it be nice if the compiler could enforce this?


## Impl

The easiest way to implement this is with three different types: 
an OpenPackage, a ClosedPackage, and an Addressed Package.

```rust
struct OpenPackage {
    pub contents: T
}
```

Crucially, the contents field is pub.

This has a few capabilities: 
- `pack`, which takes control of whatever is supposed to go in the package and 
  creates an OpenPackage around it, 
- `unpack`, which destroys the OpenPackage and gives back its contents, and 
- `close`, which converts the OpenPackage to a ClosedPackage.


```rust
impl  OpenPackage {
    fn new(contents: T) -> Self {
        OpenPackage:: {contents: contents}
    }

    fn pack(contents: T) -> Self {
        println!("\tPut some data in a package.");
        Self::new(contents)
    }

    fn unpack(self) -> T {
        println!("\tPackage unpacked.");
        self.contents
    }

    fn close(self) -> ClosedPackage {
        println!("\tPackage closed.");
        ClosedPackage::::new(self.contents)
    }
}
```

This leads to:

```rust
struct ClosedPackage {
  contents: T
}
```

Very similar, but without the pub attribute. 
This means that a ClosedPackage isn’t in danger of having its contents 
manipulated in any way.

ClosedPackages can be opened again, yielding an OpenPackage, or addressed, 
creating an AddressedPackage.

```rust
impl  ClosedPackage {
    fn new(contents: T) -> Self {
        ClosedPackage:: { contents: contents }
    }
    fn open(self) -> OpenPackage {
        println!("\tPackage opened.");
        OpenPackage::::new(self.contents)
    }
    fn address(self, address: String) -> AddressedPackage {
        println!("\tAddressed a closed package.");
        AddressedPackage::new(self.contents, address)
    }
}
```

Finally, the AddressedPackage struct represents one with a specified destination.
I used a String for the address here, but it would be trivial to create a generic version.

```rust
struct AddressedPackage {
    contents: T,
    pub address: String
}
```

To understand the access controls here, just think of a physical package. The 
address is on the outside; anyone can read it or cross it out with a sharpie. 
The contents, however, are sealed away. This struct can be turned back into a 
ClosedPackage by receiveing it:

```rust
impl  AddressedPackage {
    fn new(contents: T, address: String) -> Self {
        AddressedPackage:: { contents: contents, address: address }
    }
    fn receive(self) -> ClosedPackage {
        println!("\tPackage recieved.");
        ClosedPackage::::new(self.contents)
    }
}
```

Finally, I created an example function to “send” the package somewhere.

```rust
fn send_package(package: AddressedPackage) -> Result {
    // Save the address.
    let address = package.address.clone();

    // Destroy the package to get at the contents
    let contents = package.receive().open().unpack();
    println!("Destination recieved: {}", contents);
    // Success! Theoretically this function could fail, but not with this implementation.
    Ok(format!("Sent package to {}", address))
}
    
fn main() {
    // Make a box and then destroy it.
    let contents: String = "Here is some data.".into();
    let package = OpenPackage::pack(contents);
    // The package owns its contents.
    // println!("{}", contents); is invalid.
    println!("{}", package.unpack());

    // Now, make a box, close it, and address it.
    let contents: String = "Here is some MORE data.".into();
    let package = OpenPackage::pack(contents);
    let closed_package = package.close();
    // We now can't unpack the package to get to its contents. This is an error:
    // let contents = closed_package.unpack();
    // because ClosedPackage doesn't have .unpack()
    // Also, package is no longer valid, so no duplication can occur.
    // Finally, we can't send_package() this package; we have to address it.
    let addressed_package = closed_package.address("6902 East Pass, Madison, WI".into());
    // Now we can send the package.
    println!("{:?}", send_package(addressed_package));
}
```

This ends up printing out:
```
$ ./session_types
    Put some data in a package.
    Package unpacked.
Here is some data.
    Put some data in a package.
    Package closed.
    Addressed a closed package.
    Package received.
    Package opened.
    Package unpacked.
Destination received: Here is some MORE data.
Ok("Sent package to 6902 East Pass, Madison, WI")
```

In the real world, the Rust crate hyper makes heavy use of session types to 
ensure the integrity of HTTP requests and responses.