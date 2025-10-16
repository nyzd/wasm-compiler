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

pub struct ErrorBucket<T> {
    errors: Vec<T>,
}

impl<T> ErrorBucket<T> {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }
}

impl<T> ErrorReporter<T> for ErrorBucket<T>
where
    T: CompilerError,
{
    fn report(&mut self, error: T) {
        self.errors.push(error);
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn errors(&self) -> &[T] {
        &self.errors
    }
}
