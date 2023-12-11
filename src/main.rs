use head_tail::head;
use std::path::PathBuf;

fn main() {
    // println!("Hello, world!");
    let hh = head(PathBuf::from("./src/main.rs"), 10);
    for line in hh {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_math() {
        assert_eq!(1 + 1, 2);
    }
}
