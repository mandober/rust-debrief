pub enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {

    pub fn is_some(&self) -> bool {
        match *self {
            Some(_) => true,
            None => false,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            Some(ref x) => Some(x),
            None => None,
        }
    }

    pub fn as_mut(&self) -> Option<&T> {

    }



} //impl
