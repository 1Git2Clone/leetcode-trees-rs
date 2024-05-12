/// The crates default error type. `Box<dyn std::error::Error>` by default doesn't give async
/// support so it also implements `Send` and `Sync` in order to have the support (in case its ever
/// needed).
pub type Error = Box<dyn std::error::Error + Send + Sync>;
