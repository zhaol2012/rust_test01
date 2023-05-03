
// pub fn strtok(s: &mut &str, pat: char) -> &str {
pub fn strtok<'a>(s: &'a mut &str, pat: char) -> &'a str {
    match s.find(pat) {
        Some(i) => {
            let prefix = &s[..i];
            let suffix = &s[i + pat.len_utf8()..];
            *s = suffix;
            prefix
        },
        None => {
            let prefix = *s;
            *s = "";
            prefix
        },
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn it_works() { 
        let mut s = "hello world";
        // assert_eq!(s.find(' '), Some(5));
        assert_eq!(strtok(&mut s, ' '), "hello");
        assert_eq!(s, "world");

        let a = "hello";
        thread::spawn(move || {
            println!("{}", a)
        });
    }
}