use assets::{ TupKind, PlatformKind, Arch, ProjectAsset };
use assets::config::Config;
use assets::tuprules::Tuprules;
use assets::test_tupfile::Tupfile as TestTupfile;
use assets::tupfile::Tupfile;
use assets::platform::{ Linux, MacOS, Windows};
use project::{ Project, ProjectKind };

pub struct Builder {
  assets: Vec<ProjectAsset>,
}

impl Builder {
  pub fn new() -> Builder {
    Builder { assets: Vec::new() }
  }

  pub fn from(project: &Project) -> Builder {
    let mut builder = Builder::new();
    builder.project(&TupKind::Config, project);
    builder.project(&TupKind::TestTupfile, project);
    builder.project(&TupKind::Tuprules, project);
    builder.project(&TupKind::Tupfile, project);
    builder.platform(&PlatformKind::Linux);
    builder.platform(&PlatformKind::MacOS);
    builder.platform(&PlatformKind::Windows);

    builder
  }

  pub fn assets(&mut self) -> &Vec<ProjectAsset> {
    &self.assets.as_ref()
  }

  pub fn project(&mut self, asset_kind: &TupKind, project: &Project) {
    let asset = match *asset_kind {
      TupKind::Config => self.config(project),
      TupKind::TestTupfile => self.test_tupfile(),
      TupKind::Tuprules => self.tuprules(project),
      TupKind::Tupfile => self.tupfile(project),
    };

    self.assets.push(asset);
  }

  pub fn platform(&mut self, asset_kind: &PlatformKind) {
    let asset = match *asset_kind {
      PlatformKind::Linux => self.linux(),
      PlatformKind::MacOS => self.macos(),
      PlatformKind::Windows => self.windows()
    };

    self.assets.push(asset);
  }

  pub fn config(&self, project: &Project) -> ProjectAsset {
    let path = String::from("./");
    let contents = Config::new(project.name(), project.kind()).to_string();

    ProjectAsset::new(path, Config::name(), contents)
  }

  pub fn test_tupfile(&self) -> ProjectAsset {
    let path = String::from("./test/");
    let contents = TestTupfile::new().to_string();

    ProjectAsset::new(path, TestTupfile::name(), contents)
  }

  pub fn tuprules(&self, project: &Project) -> ProjectAsset {
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

  pub fn tupfile(&self, _project: &Project) -> ProjectAsset {
    let path = String::from("./");
    let contents = Tupfile::new().to_string();

    ProjectAsset::new(path, Tupfile::name(), contents)
  }

  pub fn linux(&self) -> ProjectAsset {
    let path = String::from("./");
    let contents = Linux::new().to_string();

    ProjectAsset::new(path, Linux::name(), contents)
  }

  pub fn macos(&self) -> ProjectAsset {
    let path = String::from("./");
    let contents = MacOS::new().to_string();

    ProjectAsset::new(path, MacOS::name(), contents)
  }

  pub fn windows(&self) -> ProjectAsset {
    let path = String::from("./");
    let contents = Windows::new().to_string();

    ProjectAsset::new(path, Windows::name(), contents)
  }
}