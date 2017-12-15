// ! string literal
/**

`&str` or `&'static str`

- text kind, valid utf8
- ref type: string literals are string slices
  always in borrowed form as &str (never as str)
- stored as text in final binary
- all string literals have 'static lifetime
- coupled with owned type String: String/&str (owned/borrowed)

*/

// declare string literal
let sl = "string literal";
// annotated string literal
let asl: &str = "annotated string literal";
// fully annotated string literal
let asl: &'static str = "lifetime annotated string literal";

// conversion to String
let s1: String = "literal".to_string();
let s2: String = "literal".to_owned();
let s3: String = "literal".into();
let s4: String = String::from("literal");
