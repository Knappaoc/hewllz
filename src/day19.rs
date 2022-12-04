use std::io::Read;

pub fn main(input: impl Read) -> Result<String, !> {
    main_01(input)
}

fn main_01(_input: impl Read) -> Result<String, !> {
    todo!()
}

fn _main_02(_input: impl Read) -> Result<String, !> {
    todo!()
}

/*
#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{main_01, main_02};

    #[test]
    fn test_main_01() {
        let file = indoc!("");
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "");
    }
    #[test]
    fn test_main_02() {
        let file = indoc!("");
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "");
    }
}
*/
