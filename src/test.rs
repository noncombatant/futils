use crate::util::unescape_backslashes;

pub fn test_main(arguments: &[String]) {
    for a in arguments {
        println!("{:?}", unescape_backslashes(&a));
    }
}
