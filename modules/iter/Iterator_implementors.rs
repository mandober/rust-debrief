// ! Iterator Implementors


impl<'a, R> Iterator for AsciiGenerator<'a, R>
    where
        R: Rng,
      type Item = char;

impl<'a, T, R> Iterator for Generator<'a, T, R>
    where
        R: Rng,
        T: Rand,
      type Item = T;

impl<'a> Iterator for SplitWhitespace<'a>
    type Item = &'a str;

impl<'a> Iterator for Utf8LossyChunksIter<'a>
    type Item = Utf8LossyChunk<'a>;

impl Iterator for ToUppercase
    type Item = char;

impl Iterator for ToLowercase
    type Item = char;

impl<I> Iterator for DecodeUtf16<I> where
I: Iterator<Item = u16>,   type Item = Result<char, DecodeUtf16Error>;

impl<I> Iterator for Utf16Encoder<I> where
I: Iterator<Item = char>,   type Item = u16;

impl<'a> Iterator for std::str::Chars<'a>
    type Item = char;

impl<'a, T> Iterator for std::result::IterMut<'a, T>  type Item = &'a mut T;

impl<I> Iterator for Cycle<I> where
I: Clone + Iterator,   type Item = <I as Iterator>::Item;

impl<'a, P> Iterator for RSplitTerminator<'a, P> where
P: Pattern<'a>,
<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,   type Item = &'a str;
impl<'a, T, P> Iterator for RSplitNMut<'a, T, P> where
P: FnMut(&T) -> bool,   type Item = &'a mut [T];
impl<A> Iterator for std::option::IntoIter<A>  type Item = A;
impl Iterator for std::char::EscapeDefault  type Item = char;
impl<I> Iterator for Fuse<I> where
I: FusedIterator,
impl<B, I, St, F> Iterator for Scan<I, St, F> where
F: FnMut(&mut St, <I as Iterator>::Item) -> Option<B>,
I: Iterator,   type Item = B;
impl<B, I, F> Iterator for FilterMap<I, F> where
F: FnMut(<I as Iterator>::Item) -> Option<B>,
I: Iterator,   type Item = B;
impl Iterator for EscapeDebug  type Item = char;
impl<I> Iterator for Fuse<I> where
I: Iterator,   type Item = <I as Iterator>::Item;
impl<A, B> Iterator for Chain<A, B> where
A: Iterator,
B: Iterator<Item = <A as Iterator>::Item>,   type Item = <A as Iterator>::Item;
impl<'a> Iterator for std::str::Bytes<'a>  type Item = u8;
impl<'a, P> Iterator for RMatches<'a, P> where
P: Pattern<'a>,
<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,   type Item = &'a str;
impl<I> Iterator for Rev<I> where
I: DoubleEndedIterator,   type Item = <I as Iterator>::Item;
impl<B, I, F> Iterator for Map<I, F> where
F: FnMut(<I as Iterator>::Item) -> B,
I: Iterator,   type Item = B;
impl<'a, P> Iterator for Matches<'a, P> where
P: Pattern<'a>,   type Item = &'a str;
impl<T> Iterator for Empty<T>  type Item = T;
impl<'a, P> Iterator for std::str::SplitN<'a, P> where
P: Pattern<'a>,   type Item = &'a str;
impl<A> Iterator for std::ops::Range<A> where
A: Step,   type Item = A;
impl<'a, I, T> Iterator for Cloned<I> where
I: Iterator<Item = &'a T>,
T: 'a + Clone,   type Item = T;
impl<I> Iterator for Enumerate<I> where
I: Iterator,   type Item = (usize, <I as Iterator>::Item);
impl<'a, T> Iterator for std::slice::IterMut<'a, T>  type Item = &'a mut T;
impl<A> Iterator for RangeFrom<A> where
A: Step,   type Item = A;
impl<I> Iterator for DecodeUtf8<I> where
I: Iterator<Item = u8>,   type Item = Result<char, InvalidSequence>;
impl<'a, P> Iterator for std::str::RSplit<'a, P> where
P: Pattern<'a>,
<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,   type Item = &'a str;
impl<'a, T> Iterator for Chunks<'a, T>  type Item = &'a [T];
impl<T> Iterator for std::result::IntoIter<T>  type Item = T;
impl<'a, I> Iterator for &'a mut I where
I: Iterator + ?Sized,   type Item = <I as Iterator>::Item;
impl<I> Iterator for Skip<I> where
I: Iterator,   type Item = <I as Iterator>::Item;
impl<'a, T, P> Iterator for std::slice::RSplitN<'a, T, P> where
P: FnMut(&T) -> bool,   type Item = &'a [T];
impl<'a> Iterator for CharIndices<'a>  type Item = (usize, char);
impl<A> Iterator for RangeInclusive<A> where
A: Step,   type Item = A;
impl<I, U, F> Iterator for FlatMap<I, U, F> where
F: FnMut(<I as Iterator>::Item) -> U,
I: Iterator,
U: IntoIterator,   type Item = <U as IntoIterator>::Item;
impl<'a, T, P> Iterator for std::slice::Split<'a, T, P> where
P: FnMut(&T) -> bool,   type Item = &'a [T];
impl<'a, A> Iterator for std::option::IterMut<'a, A>  type Item = &'a mut A;
impl<T> Iterator for Once<T>  type Item = T;
impl<'a, P> Iterator for SplitTerminator<'a, P> where
P: Pattern<'a>,   type Item = &'a str;
impl<'a, T, P> Iterator for SplitNMut<'a, T, P> where
P: FnMut(&T) -> bool,   type Item = &'a mut [T];
impl<'a, T, P> Iterator for std::slice::SplitN<'a, T, P> where
P: FnMut(&T) -> bool,   type Item = &'a [T];
impl<'a, T> Iterator for Windows<'a, T>  type Item = &'a [T];
impl<'a> Iterator for std::str::Lines<'a>  type Item = &'a str;
impl<I, P> Iterator for TakeWhile<I, P> where
I: Iterator,
P: FnMut(&<I as Iterator>::Item) -> bool,   type Item = <I as Iterator>::Item;
impl<I> Iterator for Peekable<I> where
I: Iterator,   type Item = <I as Iterator>::Item;
impl<'a, T, P> Iterator for std::slice::RSplit<'a, T, P> where
P: FnMut(&T) -> bool,   type Item = &'a [T];
impl<I, F> Iterator for Inspect<I, F> where
F: FnMut(&<I as Iterator>::Item) -> (),
I: Iterator,   type Item = <I as Iterator>::Item;
impl<A> Iterator for Repeat<A> where
A: Clone,   type Item = A;
impl<'a, P> Iterator for std::str::Split<'a, P> where
P: Pattern<'a>,   type Item = &'a str;
impl<A, B> Iterator for Zip<A, B> where
A: Iterator,
B: Iterator,   type Item = (<A as Iterator>::Item, <B as Iterator>::Item);
impl<'a, P> Iterator for std::str::RSplitN<'a, P> where
P: Pattern<'a>,
<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,   type Item = &'a str;
impl<'a> Iterator for LinesAny<'a>  type Item = &'a str;
impl<'a, P> Iterator for MatchIndices<'a, P> where
P: Pattern<'a>,   type Item = (usize, &'a str);
impl<'a, T> Iterator for ChunksMut<'a, T>  type Item = &'a mut [T];
impl<'a, T, P> Iterator for RSplitMut<'a, T, P> where
P: FnMut(&T) -> bool,   type Item = &'a mut [T];
impl<I> Iterator for StepBy<I> where
I: Iterator,   type Item = <I as Iterator>::Item;
impl<I> Iterator for Take<I> where
I: Iterator,   type Item = <I as Iterator>::Item;
impl<'a, T> Iterator for std::result::Iter<'a, T>  type Item = &'a T;
impl<'a, A> Iterator for std::option::Iter<'a, A>  type Item = &'a A;
impl<'a, T, P> Iterator for SplitMut<'a, T, P> where
P: FnMut(&T) -> bool,   type Item = &'a mut [T];
impl<'a, T> Iterator for std::slice::Iter<'a, T>  type Item = &'a T;
impl<'a, P> Iterator for RMatchIndices<'a, P> where
P: Pattern<'a>,
<P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,   type Item = (usize, &'a str);
impl<I, P> Iterator for SkipWhile<I, P> where
I: Iterator,
P: FnMut(&<I as Iterator>::Item) -> bool,   type Item = <I as Iterator>::Item;
impl<I, P> Iterator for Filter<I, P> where
I: Iterator,
P: FnMut(&<I as Iterator>::Item) -> bool,   type Item = <I as Iterator>::Item;
impl Iterator for EscapeUnicode  type Item = char;
impl<'a, T> Iterator for std::collections::linked_list::IterMut<'a, T>  type Item = &'a mut T;
impl<T> Iterator for std::collections::linked_list::IntoIter<T>  type Item = T;
impl<'a, T> Iterator for std::collections::btree_set::Intersection<'a, T> where
T: Ord,   type Item = &'a T;
impl<'a, 'b> Iterator for std::string::Splice<'a, 'b>  type Item = char;
impl<'a, T> Iterator for std::collections::btree_set::Union<'a, T> where
T: Ord,   type Item = &'a T;
impl<'a, K, V> Iterator for std::collections::btree_map::ValuesMut<'a, K, V>  type Item = &'a mut V;
impl<'a, T> Iterator for std::collections::btree_set::Iter<'a, T>  type Item = &'a T;
impl<'a, K, V> Iterator for std::collections::btree_map::Range<'a, K, V>  type Item = (&'a K, &'a V);
impl<T> Iterator for std::collections::binary_heap::IntoIter<T>  type Item = T;
impl<'a> Iterator for std::string::Drain<'a>  type Item = char;
impl<'a, T> Iterator for std::collections::binary_heap::Drain<'a, T> where
T: 'a,   type Item = T;
impl<'a, T> Iterator for std::collections::linked_list::Iter<'a, T>  type Item = &'a T;
impl<'a, K, V> Iterator for std::collections::btree_map::Values<'a, K, V>  type Item = &'a V;
impl<'a, K, V> Iterator for std::collections::btree_map::Iter<'a, K, V> where
K: 'a,
V: 'a,   type Item = (&'a K, &'a V);
impl<'a, T> Iterator for std::collections::binary_heap::Iter<'a, T>  type Item = &'a T;
impl<'a, K, V> Iterator for std::collections::btree_map::Keys<'a, K, V>  type Item = &'a K;
impl<'a, T> Iterator for std::collections::btree_set::Range<'a, T>  type Item = &'a T;
impl<T> Iterator for std::vec::IntoIter<T>  type Item = T;
impl<'a, T> Iterator for std::collections::vec_deque::Drain<'a, T> where
T: 'a,   type Item = T;
impl<'a, K, V> Iterator for RangeMut<'a, K, V>  type Item = (&'a K, &'a mut V);
impl<'a, K, V> Iterator for std::collections::btree_map::IterMut<'a, K, V> where
K: 'a,
V: 'a,   type Item = (&'a K, &'a mut V);
impl<'a, T> Iterator for std::vec::Drain<'a, T>  type Item = T;
impl<'a, I> Iterator for std::vec::Splice<'a, I> where
I: Iterator,   type Item = <I as Iterator>::Item;
impl<T> Iterator for std::collections::vec_deque::IntoIter<T>  type Item = T;
impl<'a, T> Iterator for std::collections::vec_deque::IterMut<'a, T>  type Item = &'a mut T;
impl<K, V> Iterator for std::collections::btree_map::IntoIter<K, V>  type Item = (K, V);
impl<I> Iterator for Box<I> where
I: Iterator + ?Sized,   type Item = <I as Iterator>::Item;
impl<T> Iterator for std::collections::btree_set::IntoIter<T>  type Item = T;
impl<'a, T> Iterator for std::collections::btree_set::Difference<'a, T> where
T: Ord,   type Item = &'a T;
impl<'a> Iterator for EncodeUtf16<'a>  type Item = u16;
impl<'a, T> Iterator for std::collections::btree_set::SymmetricDifference<'a, T> where
T: Ord,   type Item = &'a T;
impl<'a, T, F> Iterator for DrainFilter<'a, T, F> where
F: FnMut(&mut T) -> bool,   type Item = T;
impl<'a, T> Iterator for std::collections::vec_deque::Iter<'a, T>  type Item = &'a T;
impl Iterator for std::ascii::EscapeDefault  type Item = u8;
impl<'a, K, V> Iterator for std::collections::hash_map::Iter<'a, K, V>  type Item = (&'a K, &'a V);
impl<'a, K, V> Iterator for std::collections::hash_map::IterMut<'a, K, V>  type Item = (&'a K, &'a mut V);
impl<K, V> Iterator for std::collections::hash_map::IntoIter<K, V>  type Item = (K, V);
impl<'a, K, V> Iterator for std::collections::hash_map::Keys<'a, K, V>  type Item = &'a K;
impl<'a, K, V> Iterator for std::collections::hash_map::Values<'a, K, V>  type Item = &'a V;
impl<'a, K, V> Iterator for std::collections::hash_map::ValuesMut<'a, K, V>  type Item = &'a mut V;
impl<'a, K, V> Iterator for std::collections::hash_map::Drain<'a, K, V>  type Item = (K, V);
impl<'a, K> Iterator for std::collections::hash_set::Iter<'a, K>  type Item = &'a K;
impl<K> Iterator for std::collections::hash_set::IntoIter<K>  type Item = K;
impl<'a, K> Iterator for std::collections::hash_set::Drain<'a, K>  type Item = K;
impl<'a, T, S> Iterator for std::collections::hash_set::Intersection<'a, T, S> where
T: Eq + Hash,
S: BuildHasher,   type Item = &'a T;
impl<'a, T, S> Iterator for std::collections::hash_set::Difference<'a, T, S> where
T: Eq + Hash,
S: BuildHasher,   type Item = &'a T;
impl<'a, T, S> Iterator for std::collections::hash_set::SymmetricDifference<'a, T, S> where
T: Eq + Hash,
S: BuildHasher,   type Item = &'a T;
impl<'a, T, S> Iterator for std::collections::hash_set::Union<'a, T, S> where
T: Eq + Hash,
S: BuildHasher,   type Item = &'a T;
impl Iterator for Vars  type Item = (String, String);
impl Iterator for VarsOs  type Item = (OsString, OsString);
impl<'a> Iterator for SplitPaths<'a>  type Item = PathBuf;
impl Iterator for Args  type Item = String;
impl Iterator for ArgsOs  type Item = OsString;
impl Iterator for ReadDir  type Item = Result<DirEntry>;
impl<R: Read> Iterator for std::io::Bytes<R>  type Item = Result<u8>;
impl<R: Read> Iterator for std::io::Chars<R>  type Item = Result<char, CharsError>;
impl<B: BufRead> Iterator for std::io::Split<B>  type Item = Result<Vec<u8>>;
impl<B: BufRead> Iterator for std::io::Lines<B>  type Item = Result<String>;
impl<'a> Iterator for std::net::Incoming<'a>  type Item = Result<TcpStream>;
impl Iterator for LookupHost  type Item = SocketAddr;
impl<'a> Iterator for std::path::Iter<'a>  type Item = &'a OsStr;
impl<'a> Iterator for Components<'a>  type Item = Component<'a>;
impl<'a, T> Iterator for std::sync::mpsc::Iter<'a, T>  type Item = T;
impl<'a, T> Iterator for TryIter<'a, T>  type Item = T;
impl<T> Iterator for std::sync::mpsc::IntoIter<T>  type Item = T;
impl<'a> Iterator for EncodeWide<'a>  type Item = u16;
impl<'a> Iterator for std::os::unix::net::Incoming<'a>  type Item = Result<UnixStream>;
impl<I: Iterator + ?Sized> Iterator for Box<I>
impl<'a, T> Iterator for Iter<'a, T>
impl<T> Iterator for IntoIter<T>
impl<'a, T: 'a> Iterator for Drain<'a, T>
impl<'a, K: 'a, V: 'a> Iterator for Iter<'a, K, V>
impl<'a, K: 'a, V: 'a> Iterator for IterMut<'a, K, V>
impl<K, V> Iterator for IntoIter<K, V>
impl<'a, K, V> Iterator for Keys<'a, K, V>
impl<'a, K, V> Iterator for Values<'a, K, V>
impl<'a, K, V> Iterator for Range<'a, K, V>
impl<'a, K, V> Iterator for ValuesMut<'a, K, V>
impl<'a, K, V> Iterator for RangeMut<'a, K, V>
impl<'a, T> Iterator for Iter<'a, T>
impl<T> Iterator for IntoIter<T>
impl<'a, T> Iterator for Range<'a, T>
impl<'a, T: Ord> Iterator for Difference<'a, T>
impl<'a, T: Ord> Iterator for SymmetricDifference<'a, T>
impl<'a, T: Ord> Iterator for Intersection<'a, T>
impl<'a, T: Ord> Iterator for Union<'a, T>
impl<'a, T> Iterator for Iter<'a, T>
impl<'a, T> Iterator for IterMut<'a, T>
impl<T> Iterator for IntoIter<T>
impl<'a> Iterator for EncodeUtf16<'a>
impl<'a> Iterator for Drain<'a>
impl<'a, 'b> Iterator for Splice<'a, 'b>
impl<T> Iterator for IntoIter<T>
impl<'a, T> Iterator for Drain<'a, T>
impl<'a, I: Iterator> Iterator for Splice<'a, I>
impl<'a, T, F> Iterator for DrainFilter<'a, T, F> where F: FnMut(&mut T) -> bool,
impl<'a, T> Iterator for Iter<'a, T>
impl<'a, T> Iterator for IterMut<'a, T>
impl<T> Iterator for IntoIter<T>
impl<'a, T: 'a> Iterator for Drain<'a, T>
impl Iterator for TokenTreeIter
impl<I> Iterator for Utf16Encoder<I> where I: Iterator<Item = char>,
impl<'a> Iterator for SplitWhitespace<'a>
impl Iterator for ToLowercase
impl Iterator for ToUppercase
impl<I: Iterator<Item = u16>> Iterator for DecodeUtf16<I>
impl<'a> Iterator for Utf8LossyChunksIter<'a>
