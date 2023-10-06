pub fn hello() {
    print!("Hello!")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }
}
