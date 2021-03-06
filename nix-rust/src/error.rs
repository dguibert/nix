use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidPath(crate::store::StorePath),
    BadStorePath(std::path::PathBuf),
    BadNarInfo,
    BadBase32,
    StorePathNameEmpty,
    StorePathNameTooLong,
    BadStorePathName,
    NarSizeFieldTooBig,
    BadNarString,
    BadNarPadding,
    BadNarVersionMagic,
    MissingNarOpenTag,
    MissingNarCloseTag,
    MissingNarField,
    BadNarField(String),
    BadExecutableField,
    IOError(std::io::Error),
    #[cfg(unused)]
    HttpError(hyper::error::Error),
    Misc(String),
    Foreign(CppException),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

#[cfg(unused)]
impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Self {
        Error::HttpError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidPath(_) => write!(f, "invalid path"),
            Error::BadNarInfo => write!(f, ".narinfo file is corrupt"),
            Error::BadStorePath(path) => write!(f, "path '{}' is not a store path", path.display()),
            Error::BadBase32 => write!(f, "invalid base32 string"),
            Error::StorePathNameEmpty => write!(f, "store path name is empty"),
            Error::StorePathNameTooLong => {
                write!(f, "store path name is longer than 211 characters")
            }
            Error::BadStorePathName => write!(f, "store path name contains forbidden character"),
            Error::NarSizeFieldTooBig => write!(f, "size field in NAR is too big"),
            Error::BadNarString => write!(f, "NAR string is not valid UTF-8"),
            Error::BadNarPadding => write!(f, "NAR padding is not zero"),
            Error::BadNarVersionMagic => write!(f, "unsupported NAR version"),
            Error::MissingNarOpenTag => write!(f, "NAR open tag is missing"),
            Error::MissingNarCloseTag => write!(f, "NAR close tag is missing"),
            Error::MissingNarField => write!(f, "expected NAR field is missing"),
            Error::BadNarField(s) => write!(f, "unrecognized NAR field '{}'", s),
            Error::BadExecutableField => write!(f, "bad 'executable' field in NAR"),
            Error::IOError(err) => write!(f, "I/O error: {}", err),
            #[cfg(unused)]
            Error::HttpError(err) => write!(f, "HTTP error: {}", err),
            Error::Foreign(_) => write!(f, "<C++ exception>"), // FIXME
            Error::Misc(s) => write!(f, "{}", s),
        }
    }
}

impl From<Error> for CppException {
    fn from(err: Error) -> Self {
        match err {
            Error::Foreign(ex) => ex,
            _ => unsafe { make_error(&err.to_string()) },
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CppException(*const libc::c_void); // == std::exception_ptr*

extern "C" {
    #[allow(improper_ctypes)] // YOLO
    fn make_error(s: &str) -> CppException;
}
