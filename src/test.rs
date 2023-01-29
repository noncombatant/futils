use crate::util::{unescape_backslashes, ShellResult};

pub fn test_main(arguments: &[String]) -> ShellResult {
    for a in arguments {
        let s = unescape_backslashes(a);
        println!("{}", s.unwrap());
    }
    Ok(0)
}
