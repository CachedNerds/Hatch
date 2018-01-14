use assets::{ TupKind, PlatformKind, Arch, ProjectAsset };
use assets::config::Config;
use assets::tuprules::Tuprules;
use assets::test::Tupfile;
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
      TupKind::TestTupfile => Self::test_tupfile(project),
      TupKind::Tuprules => Self::tuprules(project),
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

  fn tuprules(project: &Project) -> ProjectAsset {
    let file_path = "C++/libs/".to_owned() + "Tuprules.tup";
    let project_kind = project.kind();
    let file_contents = match *project_kind {
      ProjectKind::Library(ref lib_type) =>  {
        Tuprules::new("g++".to_owned(), false, Arch::X64, "c++1z".to_owned(), lib_type).to_string()
      },
      _ => String::new()
    };

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