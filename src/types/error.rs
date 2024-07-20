
#[cfg(feature = "no_std")]
use core_error::Error;
#[cfg(not(feature = "no_std"))]
use std::error::Error;
use std::fmt;
#[cfg(feature = "no_std")]
use std::prelude::v1::*;

/// Evaluation result.
///
/// All wrapped [`Position`] values represent the location in the script where the error occurs.
///
/// Some errors never appear when certain features are turned on.
/// They still exist so that the application can turn features on and off without going through
/// massive code changes to remove/add back enum variants in match statements.
///
/// # Thread Safety
///
/// Currently, [`EvalAltResult`] is neither [`Send`] nor [`Sync`].
/// Turn on the `sync` feature to make it [`Send`] `+` [`Sync`].
#[derive(Debug)]
#[non_exhaustive]
#[must_use]
pub enum EvalAltResult {
    /// System error. Wrapped values are the error message and the internal error.
    #[cfg(not(feature = "sync"))]
    ErrorSystem(String, Box<dyn Error>),
    /// System error. Wrapped values are the error message and the internal error.
    #[cfg(feature = "sync")]
    ErrorSystem(String, Box<dyn Error + Send + Sync>),
    ErrorRuntime(String, String),


}

impl Error for EvalAltResult {}

impl fmt::Display for EvalAltResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorSystem(s, err) if s.is_empty() => write!(f, "{err}")?,
            Self::ErrorSystem(s, err) => write!(f, "{s}: {err}")?,
            Self::ErrorRuntime(s, err) => write!(f, "{s}: {err}")?,


        }

        Ok(())
    }
}

impl<T: AsRef<str>> From<T> for EvalAltResult {
    #[cold]
    #[inline(never)]
    fn from(err: T) -> Self {
        Self::ErrorRuntime(err.as_ref().to_string().into(), err.as_ref().to_string().into())
    }
}

impl<T: AsRef<str>> From<T> for Box<EvalAltResult> {
    #[cold]
    #[inline(always)]
    fn from(err: T) -> Self {
        Into::<EvalAltResult>::into(err).into()
    }
}

impl EvalAltResult {
    /// Is this a pseudo error?  A pseudo error is one that does not occur naturally.
    ///
    /// [`LoopBreak`][EvalAltResult::LoopBreak], [`Return`][EvalAltResult::Return] and [`Exit`][EvalAltResult::Exit] are pseudo errors.
    // #[cold]
    // #[inline(never)]
    // #[must_use]
    // pub const fn is_pseudo_error(&self) -> bool {
    //     matches!(
    //         self,
    //         Self::LoopBreak(..) | Self::Return(..) | Self::Exit(..)
    //     )
    // }
    /// Can this error be caught?
    #[cold]
    #[inline(never)]
    #[must_use]
    pub const fn is_catchable(&self) -> bool {
        match self {
            Self::ErrorSystem(..) => false,
            Self::ErrorRuntime(..) => true,

        }
    }

}
