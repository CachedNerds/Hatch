use assets::{ TupKind, PlatformKind, ProjectAsset };
use assets::config::Config;
use assets::tuprules::Tuprules;
use assets::test_tupfile::Tupfile as TestTupfile;
use assets::tupfile::Tupfile;
use assets::platform::{ Linux, MacOS, Windows };
use assets::tupfile_ini::TupfileIni;
use project::{ Project, ProjectKind, Arch };
use task;

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
    builder.project(&TupKind::TupfileIni, project);

    let platform_type = task::platform_type();
    builder.platform(&platform_type, project);

    builder
  }

  pub fn assets(&mut self) -> &Vec<ProjectAsset> {
    &self.assets.as_ref()
  }

  pub fn project(&mut self, asset_kind: &TupKind, project: &Project) {
    let asset = match *asset_kind {
      TupKind::Config => self.config(project),
      TupKind::TestTupfile => self.test_tupfile(project),
      TupKind::Tuprules => self.tuprules(project),
      TupKind::Tupfile => self.tupfile(project),
      TupKind::TupfileIni => self.tupfile_ini(project),
    };

    self.assets.push(asset);
  }

  pub fn platform(&mut self, asset_kind: &PlatformKind, project: &Project) {
    let asset = match *asset_kind {
      PlatformKind::Linux => self.linux(project),
      PlatformKind::MacOS => self.macos(project),
      PlatformKind::Windows => self.windows(project)
    };

    self.assets.push(asset);
  }

  pub fn config(&self, project: &Project) -> ProjectAsset {
    let project_path = project.path();
    let contents = Config::new(project.name(), project.kind()).to_string();

    ProjectAsset::new(project_path.to_path_buf(), Config::name(), contents)
  }

  pub fn test_tupfile(&self, project: &Project) -> ProjectAsset {
    let asset_path = project.path().join("test");
    let contents = TestTupfile::new().to_string();

    ProjectAsset::new(asset_path, TestTupfile::name(), contents)
  }

  pub fn tuprules(&self, project: &Project) -> ProjectAsset {
    let project_path = project.path();
    let project_kind = project.kind();
    let contents = match *project_kind {
      ProjectKind::Library(ref lib_type) => {
        Tuprules::new(String::from("g++"), false, Arch::X64, String::from("c++1z"), lib_type).to_string()
      },
      _ => String::new()
    };

    ProjectAsset::new(project_path.to_path_buf(), Tuprules::name(), contents)
  }

  pub fn tupfile(&self, project: &Project) -> ProjectAsset {
    let project_path = project.path();
    let contents = Tupfile::new().to_string();

    ProjectAsset::new(project_path.to_path_buf(), Tupfile::name(), contents)
  }

  pub fn tupfile_ini(&self, project: &Project) -> ProjectAsset {
    let project_path = project.path();
    let contents = TupfileIni::new().to_string();

    ProjectAsset::new(project_path.to_path_buf(), TupfileIni::name(), contents)
  }

  pub fn linux(&self, project: &Project) -> ProjectAsset {
    let project_path = project.path();
    let contents = Linux::new().to_string();

    ProjectAsset::new(project_path.to_path_buf(), Linux::name(), contents)
  }

  pub fn macos(&self, project: &Project) -> ProjectAsset {
    let project_path = project.path();
    let contents = MacOS::new().to_string();

    ProjectAsset::new(project_path.to_path_buf(), MacOS::name(), contents)
  }

  pub fn windows(&self, project: &Project) -> ProjectAsset {
    let project_path = project.path();
    let contents = Windows::new().to_string();

    ProjectAsset::new(project_path.to_path_buf(), Windows::name(), contents)
  }
}
