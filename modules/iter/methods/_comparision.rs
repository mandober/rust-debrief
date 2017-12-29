// |> cmp
fn cmp<I>(self, other: I) -> Ordering
  where
    I: IntoIterator<Item = Self::Item>,
    Self::Item: Ord,
1.5.0
Lexicographically compares the elements of this Iterator with those of another.


// |> partial_cmp
fn partial_cmp<I>(self, other: I) -> Option<Ordering>
  where
    I: IntoIterator,
    Self::Item: PartialOrd<<I as IntoIterator>::Item>
1.5.0
Lexicographically compares the elements of this Iterator with those of another.


// |> eq
fn eq<I>(self, other: I) -> bool
  where
    I: IntoIterator,
    Self::Item: PartialEq<<I as IntoIterator>::Item>
1.5.0
Determines if the elements of this Iterator are equal to those of another.


// |> ne
fn ne<I>(self, other: I) -> bool
  where
    I: IntoIterator,
    Self::Item: PartialEq<<I as IntoIterator>::Item>
1.5.0
Determines if the elements of this Iterator are unequal to those of another.


// |> lt
fn lt<I>(self, other: I) -> bool
  where
    I: IntoIterator,
    Self::Item: PartialOrd<<I as IntoIterator>::Item>
1.5.0
Determines if the elements of this Iterator are
lexicographically less than those of another.


// |> le
fn le<I>(self, other: I) -> bool
  where
    I: IntoIterator,
    Self::Item: PartialOrd<<I as IntoIterator>::Item>
1.5.0
Determines if the elements of this Iterator are
lexicographically less or equal to those of another.


// |> gt
fn gt<I>(self, other: I) -> bool
  where
    I: IntoIterator,
    Self::Item: PartialOrd<<I as IntoIterator>::Item>
1.5.0
Determines if the elements of this Iterator are
lexicographically greater than those of another.


// |> ge
fn ge<I>(self, other: I) -> bool
  where
    I: IntoIterator,
    Self::Item: PartialOrd<<I as IntoIterator>::Item>,
1.5.0
Determines if the elements of this Iterator are lexico-
graphically greater than or equal to those of another.
