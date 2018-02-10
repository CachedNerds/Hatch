use std::path::{ Path, PathBuf };

pub fn modules_path(base: &Path) -> PathBuf {
  base.join("hatch_modules")
}

pub fn hatchfile_path(base: &Path) -> PathBuf {
  base.join("Hatch.yml")
}

