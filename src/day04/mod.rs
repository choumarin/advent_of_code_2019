use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(Debug, PartialEq, PartialOrd)]
struct Password {
    code: [u32; 6],
}

impl Password {
    fn from_s(s: &str) -> Password {
        let v: Vec<u32> = s
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(v.len(), 6);
        let mut code: [u32; 6] = [0; 6];
        code.copy_from_slice(&v[0..6]);
        Password { code }
    }

    fn from_u(mut u: u32) -> Password {
        let mut code: [u32; 6] = [0; 6];
        for i in 0..code.len() {
            code[code.len() - i - 1] = u % 10;
            u /= 10;
        }
        Password { code }
    }

    fn is_valid(&self) -> bool {
        // 2 same consecutive digits
        let (mut test_a, mut test_b) = (false, true);
        for i in 0..self.code.len() - 1 {
            if self.code[i] == self.code[i + 1] {
                test_a = true;
            }
            if self.code[i + 1] < self.code[i] {
                test_b = false;
            }
        }
        test_a && test_b
    }

    fn is_valid_b(&self) -> bool {
        // 2 same consecutive digits
        let (mut test_a, mut test_b) = (false, true);
        if self.code[0] == self.code[1] && self.code[1] != self.code[2] {
            test_a = true;
        }
        if self.code[self.code.len() - 3] != self.code[self.code.len() - 2]
            && self.code[self.code.len() - 2] == self.code[self.code.len() - 1]
        {
            test_a = true;
        }
        for i in 0..self.code.len() - 3 {
            if self.code[i] != self.code[i + 1]
                && self.code[i + 1] == self.code[i + 2]
                && self.code[i + 2] != self.code[i + 3]
            {
                test_a = true;
            }
        }
        for i in 0..self.code.len() - 1 {
            if self.code[i + 1] < self.code[i] {
                test_b = false;
            }
        }
        test_a && test_b
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            self.code.iter().map(|x| x.to_string()).collect::<String>()
        )
    }
}

fn process_input(s: &str) -> Vec<&str> {
    s.split("-").collect::<Vec<&str>>()
}

pub fn entry_a(wires: String) -> String {
    let v = process_input(wires.as_str());
    let p1 = v.get(0).unwrap().parse().unwrap();
    let p2 = v.get(1).unwrap().parse().unwrap();
    let mut count = 0;
    for p in p1..p2 {
        let pw = Password::from_u(p);
        if pw.is_valid() {
            count += 1;
        }
    }
    count.to_string()
}

pub fn entry_b(wires: String) -> String {
    let v = process_input(wires.as_str());
    let p1 = v.get(0).unwrap().parse().unwrap();
    let p2 = v.get(1).unwrap().parse().unwrap();
    let mut count = 0;
    for p in p1..p2 {
        let pw = Password::from_u(p);
        if pw.is_valid_b() {
            count += 1;
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::split_paths;

    #[test]
    fn test_from() {
        assert_eq!(
            Password::from_s("123456"),
            Password {
                code: [1, 2, 3, 4, 5, 6]
            }
        );
    }

    #[test]
    fn test_to() {
        let p = Password {
            code: [1, 2, 3, 4, 5, 6],
        };
        assert_eq!(p.to_string(), "123456",);
    }

    #[test]
    fn test_is_valid() {
        assert!(Password::from_s("111111").is_valid());
        assert!(!Password::from_s("223450").is_valid());
        assert!(!Password::from_s("123789").is_valid());
    }

    #[test]
    fn test_is_valid_b() {
        assert!(Password::from_s("112233").is_valid_b());
        assert!(Password::from_s("112344").is_valid_b());
        assert!(Password::from_s("111344").is_valid_b());
        assert!(Password::from_s("111122").is_valid_b());
        assert!(Password::from_s("112222").is_valid_b());
        assert!(!Password::from_s("123444").is_valid_b());
        assert!(!Password::from_s("126668").is_valid_b());
    }

    #[test]
    fn test_process_input() {
        assert_eq!(process_input("123123-234234"), vec!["123123", "234234"]);
    }

    #[test]
    fn test_ord() {
        let p1 = Password {
            code: [1, 2, 3, 4, 5, 5],
        };
        let p2 = Password {
            code: [1, 2, 3, 4, 5, 6],
        };
        let p3 = Password {
            code: [1, 2, 0, 4, 5, 6],
        };
        assert!(p1 < p2);
        assert!(p3 < p1);
    }

    #[test]
    fn test_from_u() {
        assert_eq!(
            Password::from_u(123456),
            Password {
                code: [1, 2, 3, 4, 5, 6]
            }
        );
    }
}
