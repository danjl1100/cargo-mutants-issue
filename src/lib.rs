pub mod toplevel;

#[cfg(test)]
mod tests {
    use super::toplevel::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
