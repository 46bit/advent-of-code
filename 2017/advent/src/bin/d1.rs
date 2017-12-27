fn main() {
  println!("fmt");
}

fn d1(digit_string: &str) -> i64 {
  return 0;
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn provided_testcase_1() {
        assert_eq!(d1("1122"), 3);
    }

    #[test]
    fn provided_testcase_2() {
        assert_eq!(d1("1111"), 4);
    }

    #[test]
    fn provided_testcase_3() {
        assert_eq!(d1("1234"), 0);
    }

    #[test]
    fn provided_testcase_4() {
        assert_eq!(d1("91212129"), 9);
    }
}
