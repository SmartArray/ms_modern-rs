pub mod parse;
pub mod format;

pub use parse::parse;
pub use format::format;

pub fn ms<T: Into<MsInput>>(input: T) -> Result<MsOutput, &'static str> {
    match input.into() {
        MsInput::Str(s) => parse(&s).map(MsOutput::Milliseconds),
        MsInput::Int(i) => Ok(MsOutput::Str(format(i))),
    }
}

pub enum MsInput {
    Str(String),
    Int(i64),
}

impl From<&str> for MsInput {
    fn from(s: &str) -> Self {
        MsInput::Str(s.to_string())
    }
}

impl From<String> for MsInput {
    fn from(s: String) -> Self {
        MsInput::Str(s)
    }
}

impl From<i64> for MsInput {
    fn from(i: i64) -> Self {
        MsInput::Int(i)
    }
}

pub enum MsOutput {
    Str(String),
    Milliseconds(i64),
}

impl MsOutput {
    pub fn unwrap_str(self) -> String {
        if let MsOutput::Str(s) = self {
            s
        } else {
            panic!("Expected MsOutput::Str")
        }
    }

    pub fn unwrap_number(self) -> i64 {
        if let MsOutput::Milliseconds(ms) = self {
            ms
        } else {
            panic!("Expected MsOutput::Milliseconds")
        }
    }
}

pub trait UnwrapMsOutput {
    fn unwrap_str(self) -> String;
    fn unwrap_number(self) -> i64;
}

impl<E> UnwrapMsOutput for Result<MsOutput, E> {
    fn unwrap_str(self) -> String {
        match self {
            Ok(MsOutput::Str(s)) => s,
            Ok(_) => panic!("Expected MsOutput::Str"),
            Err(_) => panic!("Unwrapping Result failed"),
        }
    }

    fn unwrap_number(self) -> i64 {
        match self {
            Ok(MsOutput::Milliseconds(ms)) => ms,
            Ok(_) => panic!("Expected MsOutput::Milliseconds"),
            Err(_) => panic!("Unwrapping Result failed"),
        }
    }
}

// Some basic tests for this library.
// ToDo: Add more tests here. I needed to craft those quickly.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("2 days").unwrap(), 172_800_000);
        assert_eq!(parse("1 minute").unwrap(), 60_000);
        assert!(parse("unknown").is_err());
    }

    #[test]
    fn test_format() {
        assert_eq!(format(172_800_000), "2 days");
        assert_eq!(format(60_000), "1 minute");
        assert_eq!(format(999), "999 ms");
    }

    #[test]
    fn test_negative_parse() {
        assert_eq!(parse("-2 days").unwrap(), -172_800_000);
    }

    #[test]
    fn test_negative_format() {
        assert_eq!(format(-172_800_000), "-2 days");
    }

    #[test]
    fn test_ms_wrapper() {
        match ms("2 days").unwrap() {
            MsOutput::Milliseconds(ms) => assert_eq!(ms, 172_800_000),
            _ => panic!("Expected milliseconds"),
        }
        match ms(60_000).unwrap() {
            MsOutput::Str(s) => assert_eq!(s, "1 minute"),
            _ => panic!("Expected string"),
        }

        // Test invalid input
        assert!(ms("unknown").is_err());
        assert!(ms("10 unknown_units").is_err());
        
        // Test unwrap_str on Result
        assert_eq!(ms(60_000).unwrap_str(), "1 minute");
        assert_eq!(ms(172_800_000).unwrap_str(), "2 days");
        
        // Test unwrap_number on Result
        assert_eq!(ms("2 days").unwrap_number(), 172_800_000);
        assert_eq!(ms("1 minute").unwrap_number(), 60_000);

        // Test unwrap_number on negative
        assert_eq!(ms("-2 days").unwrap_number(), -172_800_000);
        assert!(ms("--2 days").is_err());
        assert!(ms("unknown").is_err());
    }
}
