
#[derive(Debug)]

enum GivenResult <T, E> {
    // <T, E> is a generic type
    Pass(T),
    Fail(E),
}

fn _check_under_five(number: u8) -> GivenResult<String, String> {
    if number < 5{
        GivenResult::Pass("Number is under five".to_string())
    } else {
        GivenResult::Fail("Number is NOT under five".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_under_five() {
        let result: GivenResult<String, String> = _check_under_five(3);
        dbg!(result);
    } 
}
