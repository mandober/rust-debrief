pub stuct Rust<'a,'b,T:'a,'a:'b>
where T: PartailOrd + PartialEq
{
  f1: u8,
  f2: &'a mut u8,
  f3: &mut char,
  f4: &mut u16,
}


fn main() {
    println!("pushing for detection of this repo as if containing Rust code");
}

pub trait GitHub<'c> {
    type Repo;

    fn make_me_a_rust_repo(&'c self) -> bool;
}
