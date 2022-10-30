pub struct SsString {
    string: String,
}

impl SsString {
    pub fn new() -> SsString {
        SsString {
            string: String::from(""),
        }
    }

    pub fn from(string: &str) -> SsString {
        SsString {
            string: string.to_owned(),
        }
    }

    pub fn get_string(&self) -> String {
        self.string.clone()
    }

    pub fn split(&self) -> Vec<&str> {
        self.string.split(';').map(|string| string.trim()).collect()
    }
}

impl std::fmt::Debug for SsString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
