use assets::{ TupKind, PlatformKind, Arch, ProjectAsset };
use assets::config::Config;
use assets::tuprules::Tuprules;
use assets::test_tupfile::Tupfile as TestTupfile;
use assets::tupfile::Tupfile;
use assets::platform::{ Linux, Darwin, Windows};
use project::{ Project, ProjectKind };

pub struct Builder {
  assets: Vec<ProjectAsset>,
}

impl Builder {
  pub fn from(project: &Project) -> Builder {
    let mut builder = Builder { assets: Vec::new() };
    builder.project(&TupKind::Config, project);
    builder.project(&TupKind::TestTupfile, project);
    builder.project(&TupKind::Tuprules, project);
    builder.project(&TupKind::Tupfile, project);
    builder.platform(&PlatformKind::Linux);
    builder.platform(&PlatformKind::Darwin);
    builder.platform(&PlatformKind::Windows);

    builder
  }

  pub fn assets(&mut self) -> &Vec<ProjectAsset> {
    &self.assets.as_ref()
  }

  pub fn project(&mut self, asset_kind: &TupKind, project: &Project) {
    let asset = match *asset_kind {
      TupKind::Config => Self::config(project),
      TupKind::TestTupfile => Self::test_tupfile(),
      TupKind::Tuprules => Self::tuprules(project),
      TupKind::Tupfile => Self::tupfile(project),
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
    let path = String::from("./");
    let contents = Config::new(project.name(), project.kind()).to_string();

    ProjectAsset::new(path, Config::name(), contents)
  }

  fn test_tupfile() -> ProjectAsset {
    let path = String::from("./test/");
    let contents = TestTupfile::new().to_string();

    ProjectAsset::new(path, TestTupfile::name(), contents)
  }

  fn tuprules(project: &Project) -> ProjectAsset {
    let path = String::from("./");
    let project_kind = project.kind();
    let contents = match *project_kind {
      ProjectKind::Library(ref lib_type) =>  {
        Tuprules::new(String::from("g++"), false, Arch::X64, String::from("c++1z"), lib_type).to_string()
      },
      _ => String::new()
    };

    ProjectAsset::new(path, Tuprules::name(), contents)
  }

  fn tupfile(project: &Project) -> ProjectAsset {
    let path = String::from("./");
    let contents = Tupfile::new().to_string();

    ProjectAsset::new(path, Tupfile::name(), contents)
  }

  fn linux() -> ProjectAsset {
    let path = String::from("./");
    let contents = Linux::new().to_string();

    ProjectAsset::new(path, Linux::name(), contents)
  }

  fn darwin() -> ProjectAsset {
    let path = String::from("./");
    let contents = Darwin::new().to_string();

    ProjectAsset::new(path, Darwin::name(), contents)
  }

  fn windows() -> ProjectAsset {
    let path = String::from("./");
    let contents = Windows::new().to_string();

    ProjectAsset::new(path, Windows::name(), contents)
  }
}