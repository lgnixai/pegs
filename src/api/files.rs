
use crate::{Engine, RhaiResultOf, ERR, Scope};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use crate::types::scope;

impl Engine {
    /// Read the contents of a file into a string.
    fn read_file(path: impl AsRef<Path>) -> RhaiResultOf<String> {
        let path = path.as_ref();

        let mut f = File::open(path).map_err(|err| {
            ERR::ErrorSystem(
                format!("Cannot open script file '{}'", path.to_string_lossy()),
                err.into(),
            )
        })?;

        let mut contents = String::new();

        f.read_to_string(&mut contents).map_err(|err| {
            ERR::ErrorSystem(
                format!("Cannot read script file '{}'", path.to_string_lossy()),
                err.into(),
            )
        })?;

        if contents.starts_with("#!") {
            // Remove shebang
            match contents.find('\n') {
                Some(n) => {
                    contents.drain(0..n).count();
                }
                None => contents.clear(),
            }
        };

        Ok(contents)
    }
    #[inline]
    pub fn run_file(&mut self, path: PathBuf) -> RhaiResultOf<()> {
        Self::read_file(path).and_then(|contents| self.run(&*contents))
    }
    #[inline]
    pub fn run_file_scope(&mut self, path: PathBuf, scope:&mut Scope) -> RhaiResultOf<()> {
        Self::read_file(path).and_then(|contents| self.run_scope(&*contents,scope))
    }


}