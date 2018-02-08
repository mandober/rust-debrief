// References: ref, box

fn main() {

    /// MUT REFS
    let mut p = 55;
    // take a mut ref
    let e: &mut i32 = &mut p;
    println!("e:&mut i32 is {}", e);
    // deref a mut ref
    *e = 65;
    println!("*e is {}", e);
    
    // WEAKENING THE TYPE
    // Weaken the ref by loosing mutability: &mut >> &
    // Original var is dropped, new eponymous var shadows it.
    // Type annotation is necessary for this coercion:
    let e: &i32 = e;
    println!("e:&i32 is {}", e);
    
    // since it is now just a ref, we can take another shared ref by copying it
    let w = e;
    // both refs are live
    println!("w:&i32 is {}; e:&i32 is {}", w, e);
    
    
    /// BOX
    let b = Box::new(11_u8);
    println!("b:Box<u8> is {:?}", b);
    
    // Borrow
    use std::borrow::Borrow;
    // borrow() -- get a ref (from the box) to the boxed value -- get unboxed ref
    let f: &u8 = b.borrow();
    assert_eq!(f, &11);

    // as_ref() -- get a ref (from the box) to the boxed value -- get unboxed ref
    let f: &u8 = b.as_ref();
    assert_eq!(f, &11);
    
    // deref() -- get a ref (from the box) to the boxed value -- get unboxed ref
    use std::ops::Deref;
    let f: &u8 = b.deref();
    assert_eq!(f, &11);

    // all such refs (from borrow, as_ref, deref) can be dereferenced:
    assert_eq!(*f, 11);

    // Box is apecial ref -- it can be derefed (unboxed) directly
    println!("b:u8 is {}", *b);

    
} // main