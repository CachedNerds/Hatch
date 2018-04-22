use assets::{PlatformKind, ProjectAsset, TupKind};
use assets::config::Config;
use assets::tuprules::Tuprules;
use assets::test_tupfile::Tupfile as TestTupfile;
use assets::tupfile::Tupfile;
use assets::platform::{Linux, MacOS, Windows};
use assets::tupfile_ini::TupfileIni;
use assets::catch_header::CatchHeader;
use assets::catch_definition::CatchDefinition;
use hatch_error::{HatchResult, NullError, ResultExt};
use project::Project;
use platform::os;
use reqwest;

static CATCH_HEADER_URL: &str =
    "https://github.com/catchorg/Catch2/releases/download/v2.1.1/catch.hpp";

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

        let platform_type = os::platform_type();
        builder.platform(&platform_type, project);

        if let Ok(catch_header) = builder.catch_header(project) {
            builder.add_asset(catch_header);
        }

        let catch_definition = builder.catch_definition(project);
        builder.add_asset(catch_definition);

        builder
    }

    pub fn assets(&self) -> &Vec<ProjectAsset> {
        &self.assets.as_ref()
    }

    pub fn add_asset(&mut self, asset: ProjectAsset) {
        self.assets.push(asset);
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
            PlatformKind::Windows => self.windows(project),
        };

        self.assets.push(asset);
    }

    pub fn config(&self, project: &Project) -> ProjectAsset {
        let project_path = project.path();
        let contents = Config::new(project.name(), project.config().kind()).to_string();

        ProjectAsset::new(project_path.to_path_buf(), Config::name(), contents)
    }

    pub fn test_tupfile(&self, project: &Project) -> ProjectAsset {
        let asset_path = project.path().join("test");
        let contents = TestTupfile::new().to_string();

        ProjectAsset::new(asset_path, TestTupfile::name(), contents)
    }

    pub fn tuprules(&self, project: &Project) -> ProjectAsset {
        let project_path = project.path();
        let config = project.config();
        let contents = Tuprules::new(config).to_string();

        ProjectAsset::new(project_path.to_path_buf(), Tuprules::name(), contents)
    }

    pub fn tupfile(&self, project: &Project) -> ProjectAsset {
        let project_path = project.path();
        let contents = Tupfile::new(project.config().kind()).to_string();

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

    pub fn catch_header(&self, project: &Project) -> HatchResult<ProjectAsset> {
        let test_src_path = project.path().join("test/src");
        let file_name = CatchHeader::name();
        if !test_src_path.join(file_name).exists() {
            let res = (|| -> HatchResult<ProjectAsset> {
                let mut resp = reqwest::get(CATCH_HEADER_URL)?;
                let content = resp.text()?;

                Ok(ProjectAsset::new(
                    test_src_path,
                    CatchHeader::name(),
                    content,
                ))
            })()
                .with_context(|e| format!("failed to generate catch.hpp : {}", e))?;

            Ok(res)
        } else {
            Err(NullError)?
        }
    }

    pub fn catch_definition(&self, project: &Project) -> ProjectAsset {
        let test_src_path = project.path().join("test/src");
        let contents = CatchDefinition::new().to_string();

        ProjectAsset::new(test_src_path, CatchDefinition::name(), contents)
    }
}
