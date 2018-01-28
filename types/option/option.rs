// Option manipulations


/// Convert an Option
// (owned) String
let payload: String = "colony".to_string();

// Wrap the String in an Option. This moves it, so payload is no more
let optw: Option<String> = Some(payload);

// in order to keep Option<String> alive as_ref will
// produce Option<&String> which will be sacrificed to map
let rf: Option<&String> = optw.as_ref();

// map converts Option<T> to Option<U> by applying fn to the wrapped value.
// if map would eat Option<String>, optw would be moved. In order to save optw
// as_ref comes to the rescue by producing a Option<&String> for map to eat instead.
let opt_len: Option<usize> = rf.map(|payload| payload.len());

println!("{:?}", opt_len);

println!("{:?}", optw);
