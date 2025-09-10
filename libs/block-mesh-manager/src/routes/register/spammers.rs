use regex::Regex;

pub struct SpammersHelper;

impl SpammersHelper {
    pub fn check_email_address(email: &str) -> bool {
        let patterns = vec![
            r"^crypto\d+wallet\d+",
            r"^verify\d+",
            r"^valid\d+account\d+",
            r"^user\d+",
            r"^shaafishaafi.*",
            r"^real\d+person\d+",
            r"^otp\d+user\d+",
            r"^michealemerald\d+",
            r"^laho\d+",
            r"^confirm\d+",
            r"^block\d+mesh\d+",
            r"1secmail",
        ];
        let mut regs: Vec<Regex> = Vec::new();
        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                regs.push(re);
            }
        }
        for reg in regs {
            if reg.is_match(email) {
                return false;
            }
        }
        true
    }
}
