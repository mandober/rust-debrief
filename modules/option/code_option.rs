// Option


// using as_ref and map
//
// Create an owned String
let payload: String = "colony".to_string();
// Wrap the String in an Option - this moves the String
let opt_payload: Option<String> = Some(payload);
// Mapping an Option in order to find the length of the String
let opt_len: Option<usize> = opt_payload.map(|pl| pl.len());
// We get Option<usize> with the value Some(6), but wrapped String is gone.
// Since map consumes self, the Option<String> is devoured.
// In order to preserve Option<String> and get the same result (length of the
// Sting), as_ref comes to rescue: by calling it on Option<String>, it produces
// Option<&String>, which will be sacrificed to map instead.
let rf: Option<&String> = opt_payload.as_ref();
// map converts Option<T> to Option<U> by applying fn to the wrapped value.
// if map would eat Option<String>, opt_payload would be moved. In order to save opt_payload
// as_ref comes to the rescue by producing a Option<&String> for map to eat instead.
let opt_len: Option<usize> = rf.map(|payload| payload.len());
println!("{:?}", opt_len);
println!("{:?}", opt_payload);


// mem::swap
//
// needs 2 mut refs: it just swaps the two, returning nothing.
let mut opt = Some(6_u8);
let mut nada: Option<u8> = None;
std::mem::swap(&mut nada, &mut opt);
println!("{:?}", nada); // Some(6)


// mem::replace
//
// needs a mut ref as a subject and a value of the same type as replacement.
// It steals the subject and puts the replacement value in its place.
// it will return the subject.
let opt = Some(6_u8);
let mut nada: Option<u8> = None;
let res = std::mem::replace(&mut nada, opt);
println!("{:?}", nada); // Some(6)
println!("{:?}", res); // None
