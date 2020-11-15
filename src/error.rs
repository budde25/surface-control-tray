use failure::{Context, Fail, Backtrace};


pub type Result<T> = std::result::Result<T, Error>;
pub use failure::ResultExt;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Fail)]
pub enum ErrorKind {
    #[fail(display = "I/O failure")]
    Io,

    #[fail(display = "Received invalid data")]
    InvalidData,

    #[fail(display = "Cannot access device")]
    DeviceAccess,
}


impl Error {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }

    pub fn iter_causes(&self) -> failure::Causes {
        ((&self.inner) as &dyn Fail).iter_causes()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}


pub type CliResult = std::result::Result<(), CliError>;

pub struct CliError {
    error: Error,
}

impl From<Error> for CliError {
    fn from(error: Error) -> Self {
        CliError { error }
    }
}

impl std::fmt::Debug for CliError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}.", self.error.kind())?;
        for cause in self.error.iter_causes() {
            write!(fmt, "\n       {}.", cause)?;
        }

        Ok(())
    }
}
