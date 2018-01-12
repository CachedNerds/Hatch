use project::Project;
use dtl::tup::Asset;
use dtl::tup::ProjectAsset;
use dtl::tup::config::Config;
use dtl::tup::platform::{ Linux, Darwin, Windows };
use dtl::tup::test::Tupfile;

pub fn print_file_path<T>(asset: T) where T: Asset {
  println!("{}", asset.path());
}

pub fn print_file_contents<T>(asset: T) where T: Asset {
  println!("{}", asset.contents());
}

#[derive(Debug)]
pub enum TupKind { Tuprules, Config, Tupfile, TestTupfile }

#[derive(Debug)]
pub enum PlatformKind { Linux, Darwin, Windows }

#[derive(Debug)]
pub enum AssetKind { Os(PlatformKind), Tup(TupKind) }

#[derive(Debug)]
pub struct ProjectAssetBuilder {
  assets: Vec<ProjectAsset>,
}

impl ProjectAssetBuilder {
  pub fn from(project: &Project) -> ProjectAssetBuilder {
    let mut asset_builder = ProjectAssetBuilder { assets: Vec::new() };
    asset_builder.project(&TupKind::Config, project);
    asset_builder.platform(&PlatformKind::Darwin);

    asset_builder
  }

  pub fn assets(&mut self) -> &Vec<ProjectAsset> {
    &self.assets.as_ref()
  }

  pub fn project(&mut self, asset_kind: &TupKind, project: &Project) {
    let asset = match *asset_kind {
      TupKind::Config => Self::config(project),
      TupKind::TestTupfile => Self::test_tupfile(project),
      _ => ProjectAsset::new(String::new(), String::new())
    };

    self.assets.push(asset);
  }

  pub fn platform(&mut self, asset_kind: &PlatformKind) {
    let asset = match *asset_kind {
      PlatformKind::Linux => Self::linux(),
      PlatformKind::Darwin => Self::darwin(),
      PlatformKind::Windows => Self::windows()
    };

    self.assets.push(asset);
  }

  fn config(project: &Project) -> ProjectAsset {
    let file_path = "C++/libs/".to_owned() + project.name() + "/config.tup";
    let file_contents = Config::new(project.name(), project.kind()).to_string();

    ProjectAsset::new(file_path, file_contents)
  }

  fn test_tupfile(project: &Project) -> ProjectAsset {
    let file_path = "C++/libs/".to_owned() + project.name() + "/test/Tupfile";
    let file_contents = Tupfile::new().to_string();

    ProjectAsset::new(file_path, file_contents)
  }

  fn linux() -> ProjectAsset {
    let file_path = "C++/libs/linux.tup".to_owned();
    let file_contents = Linux::new().to_string();

    ProjectAsset::new(file_path, file_contents)
  }

  fn darwin() -> ProjectAsset {
    let file_path = "C++/libs/macosx.tup".to_owned();
    let file_contents = Darwin::new().to_string();

    ProjectAsset::new(file_path, file_contents)
  }

  fn windows() -> ProjectAsset {
    let file_path = "C++/libs/win32.tup".to_owned();
    let file_contents = Windows::new().to_string();

    ProjectAsset::new(file_path, file_contents)
  }
}
