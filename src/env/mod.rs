use crate::{errors::LispError, lisp::LispBuilder};

mod std;

#[cfg(feature = "std")]
mod sys;

impl LispBuilder {
    #[cfg(feature = "std")]
    pub fn add_default_env(self) -> Result<Self, LispError> {
        self.add_stdenv()?.add_sysenv()
    }

    #[cfg(not(feature = "std"))]
    pub fn add_default_env(self) -> Result<Self, LispError> {
        self.add_stdenv()
    }
}
