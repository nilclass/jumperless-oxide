//pub mod connection;
//pub mod nets;
pub mod matrix_state;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn stuff() {
    //     let mut f = std::fs::File::create("graph.dot").unwrap();
    //     matrix_state::State::default().to_dot(&mut f).unwrap();
    // }
}
