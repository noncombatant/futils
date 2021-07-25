use crate::util::unescape_backslashes;

pub fn test_main(arguments: &[String]) {
    for a in arguments {
        let s = unescape_backslashes(&a);
        println!("{}", s.unwrap());
    }
}
