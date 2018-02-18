pub use failure::ResultExt;
pub use failure::Error as HatchError;

pub type HatchResult<T> = Result<T, HatchError>;

#[derive(Fail, Debug)]
#[fail(display = "No content in Hatch.yml")]
pub struct EmptyConfigError;

#[derive(Fail, Debug)]
#[fail(display = "No build field found in Hatch.yml")]
pub struct MissingBuildError;

#[derive(Fail, Debug)]
#[fail(display = "No name field found in Hatch.yml")]
pub struct MissingNameError;

#[derive(Fail, Debug)]
#[fail(display = "No version field found in Hatch.yml")]
pub struct MissingVersionError;

#[derive(Fail, Debug)]
#[fail(display = "Invalid path")]
pub struct InvalidPathError;

#[derive(Fail, Debug)]
#[fail(display = "")]
pub struct NullError;
