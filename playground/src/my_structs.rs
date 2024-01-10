#[derive(Debug)]

struct Rectangle {
    width: u8,
    height: u8,
}

impl Rectangle {
    fn area(&self) -> u8 {
        self.width * self.height
    }
}

fn _check_rect_area(width: u8, height: u8) -> u8 {
    let rect1 = Rectangle {
        width: width,
        height: height,
    };

    dbg!(rect1.area());
    rect1.area()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_rect_area() {
        let result: u8 = _check_rect_area(3, 4);
        assert_eq!(result, 12);
        dbg!(result);
    } 
}
