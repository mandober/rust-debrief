fn main() {
    println!("make it a rust repo");
}

pub trait Traitor {
    type Repo;

    fn make_rust(&self) -> bool;
}
