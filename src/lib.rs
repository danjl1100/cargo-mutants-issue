pub mod toplevel {
    pub mod nested;
}

#[cfg(test)]
mod tests {
    use super::toplevel::nested::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
