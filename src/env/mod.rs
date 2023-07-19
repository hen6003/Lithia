use crate::{errors::LispError, lisp::LispBuilder};

mod maths;
mod std;

#[cfg(feature = "std")]
mod sys;

impl LispBuilder {
    #[cfg(feature = "std")]
    pub fn add_default_envs(self) -> Result<Self, LispError> {
        self.add_env_std()?.add_env_maths()?.add_env_sys()
    }

    #[cfg(not(feature = "std"))]
    pub fn add_default_envs(self) -> Result<Self, LispError> {
        self.add_env_std()?.add_env_maths()
    }
}
