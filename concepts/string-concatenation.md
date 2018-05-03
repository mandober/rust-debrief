# concatenation

add a &str to a String using push_str()

let mut realstring = String::from("hello ");
let str1 = "world!";
realstring.push_str(str1);

add &strs using format!
let str1 = "hello ";
let str2 = "world!";
let message = format!("{}{}", str1, str2);