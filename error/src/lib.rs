pub trait ErrorReporter<E>: Send + Sync {
    fn report(&mut self, error: E);
    fn has_errors(&self) -> bool;
    fn errors(&self) -> &[E];
}

pub trait CompilerError: Send + Sync {
    /// ID of the error like 001, 002
    fn id(&self) -> usize;

    /// Generalized description
    fn title(&self) -> &str;

    /// Summary of what is gone wrong
    fn summary(&self) -> &str;

    /// More detail
    fn description(&self) -> &str;

    /// Stage of compiler that this
    /// Error is reported.
    fn stage(&self) -> &'static str;
}
