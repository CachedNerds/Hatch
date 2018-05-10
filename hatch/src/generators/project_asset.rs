use std::fs;
use hatch_error::HatchResult;
use std::path::Path;
use std::path::PathBuf;
use std::fmt;
use std::cmp;
use failure::ResultExt;
use std::io::Write;

pub struct ProjectAsset {
    path: PathBuf,
    name: String,
    contents: String,
}

impl ProjectAsset {
    pub fn new(path: PathBuf, name: String, contents: String) -> ProjectAsset {
        ProjectAsset {
            path,
            name,
            contents,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn path(& self) -> &Path {
        self.path.as_path()
    }

    pub fn write(&self) -> HatchResult<()> {
        let path = self.path();
        fs::create_dir_all(path)
            .with_context(|e| format!("Failed to create directory: `{}` : {}", path.display(), e))?;

        let file_path = path.join(&self.name);
        let mut file = fs::File::create(&file_path)
            .with_context(|e| format!("Failed to create file: `{}` : {}", file_path.display(), e))?;

        file.write_all(self.contents.as_bytes()).with_context(|e| {
            format!(
                "Failed to write contents to file: `{}` : {}",
                file_path.display(),
                e
            )
        })?;

        Ok(())
    }
}

impl fmt::Debug for ProjectAsset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path: {}, name: {}, contents: {}", self.path.display(), self.name, self.contents)
    }
}

impl cmp::PartialEq for ProjectAsset {
    fn eq(&self, other: &ProjectAsset) -> bool {
        self.name == other.name && self.contents == other.contents
    }
}