fn get_message() -> String {
    "Hello, world!".to_string()
}
fn main() {
    println!("Message: {0}", get_message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message() {
        assert_eq!(get_message(), "Hello, world!");
    }
}
