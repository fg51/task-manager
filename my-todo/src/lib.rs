pub mod domain;
pub mod app;
pub mod errors;

#[cfg(test)]
mod tests {
    //use super::*;

    fn add(left: usize, right: usize) -> usize {
        left + right
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
