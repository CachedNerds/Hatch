use assets::catch_definition::CatchDefinition;
use assets::catch_header::CatchHeader;
use assets::config::Config;
use assets::platform::{Linux, MacOS, Windows};
use assets::test_tupfile::Tupfile as TestTupfile;
use assets::tupfile::Tupfile;
use assets::tupfile_ini::TupfileIni;
use assets::tuprules::Tuprules;
use assets::{PlatformKind, ProjectAsset, TupKind};
use constants::CATCH_HEADER_URL;
use hatch_error::{HatchResult, NullError, ResultExt};
use project::Project;
use reqwest;
use std::path::PathBuf;

pub struct Builder<'builder> {
    project_path: PathBuf,
    project: &'builder Project,
    pub assets: Vec<ProjectAsset>,
}

impl<'builder> Builder<'builder> {
    pub fn new(project_path: PathBuf, project: &'builder Project) -> Builder<'builder> {
        Builder {
            project_path,
            project,
            assets: Vec::new(),
        }
    }

    pub fn collect_assets(builder: Builder) -> Vec<ProjectAsset> {
        builder.assets
    }

    pub fn add_asset(&mut self, asset: ProjectAsset) {
        self.assets.push(asset);
    }

    pub fn project(&mut self, asset_kind: TupKind) {
        let asset = match asset_kind {
            TupKind::Config => self.config(),
            TupKind::TestTupfile => self.test_tupfile(),
            TupKind::Tuprules => self.tuprules(),
            TupKind::Tupfile => self.tupfile(),
            TupKind::TupfileIni => self.tupfile_ini(),
        };

        self.assets.push(asset);
    }

    pub fn platform(&mut self, asset_kind: &PlatformKind) {
        let asset = match *asset_kind {
            PlatformKind::Linux => self.linux(),
            PlatformKind::MacOS => self.macos(),
            PlatformKind::Windows => self.windows(),
        };

        self.assets.push(asset);
    }

    pub fn config(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let project = &self.project;
        let contents = Config::new(project.name(), project.kind()).to_string();
        ProjectAsset::new(project_path, Config::name(), contents)
    }

    pub fn test_tupfile(&self) -> ProjectAsset {
        let asset_path = self.project_path.join("test");
        let contents = TestTupfile::new().to_string();
        ProjectAsset::new(asset_path, TestTupfile::name(), contents)
    }

    pub fn tuprules(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let project = &self.project;
        let tuprules = Tuprules {};
        let contents = tuprules.to_string(&project);
        ProjectAsset::new(project_path, Tuprules::name(), contents)
    }

    pub fn tupfile(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let project = &self.project;
        let contents = Tupfile::new(project.kind()).to_string();
        ProjectAsset::new(project_path, Tupfile::name(), contents)
    }

    pub fn tupfile_ini(&self) -> ProjectAsset {
        let contents = TupfileIni::new().to_string();
        let project_path = self.project_path.clone();
        ProjectAsset::new(project_path, TupfileIni::name(), contents)
    }

    pub fn linux(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = Linux::new().to_string();
        ProjectAsset::new(project_path, Linux::name(), contents)
    }

    pub fn macos(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = MacOS::new().to_string();
        ProjectAsset::new(project_path, MacOS::name(), contents)
    }

    pub fn windows(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = Windows::new().to_string();
        ProjectAsset::new(project_path, Windows::name(), contents)
    }

    pub fn catch_header(&self) -> HatchResult<ProjectAsset> {
        let test_src_path = self.project_path.join("test/src");
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
            })().with_context(|e| format!("failed to generate catch.hpp : {}", e))?;

            Ok(res)
        } else {
            Err(NullError)?
        }
    }

    pub fn catch_definition(&self) -> ProjectAsset {
        let test_src_path = self.project_path.join("test/src");
        let contents = CatchDefinition::new().to_string();

        ProjectAsset::new(test_src_path, CatchDefinition::name(), contents)
    }
}
