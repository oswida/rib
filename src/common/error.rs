use core::fmt;

#[derive(Debug, Clone)]
struct BotError;

type Result<T> = std::result::Result<T, BotError>;

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
