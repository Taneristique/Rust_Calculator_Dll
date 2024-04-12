#[allow(unused)]
#[no_mangle]
pub extern "C" fn add( a: i32, b: i32) -> i32 {
    return a + b;
}
#[allow(unused)]
#[no_mangle]
pub extern "C" fn mul( a: i32, b: i32) -> i32 {
    return a * b;
}
#[allow(unused)]
#[no_mangle]
pub extern "C" fn sub( a: i32,  b: i32) -> i32 {
    return a - b;
}

#[allow(unused)]
#[no_mangle]
pub extern "C" fn div( a: i32,  b: i32) -> f64 {
    if b != 0 {
        return (a as f64 / b as f64) as f64;
    } else {
        return b as f64 ;
    }
}

#[cfg(test)]
mod tests {
    use crate::{add, div, mul, sub};

    #[test]
    pub fn mock_addition() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    pub fn mock_multiplication() {
        assert_eq!(mul(2, 2), 4);
    }

    #[test]
    pub fn mock_substraction() {
        assert_eq!(sub(2, 2), 0);
    }

    #[test]
    pub fn mock_division() {
        let a: i32 = 2;
        let b: i32 = 1;
        let result = div(a, b);
        assert_eq!(result, 2.0);
    }

    #[test]
    pub fn test_division_by_zero() {
        let a = 5;
        let b = 0;
        let result = div(a, b);
        assert_eq!(result, 0.0);
    }
}
