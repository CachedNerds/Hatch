use assets::{PlatformKind, ProjectAsset, TupKind};
use constants::CATCH_HEADER_URL;
use generators::tup::catch_header::CatchHeader;
use generators::tup::tupfile_ini::TupfileIni;
use hatch_error::{HatchResult, NullError, ResultExt};
use project::Project;
use reqwest;
use std::path::PathBuf;
use generators::tup::tuprules::make_tuprules_string;
use generators::tup::test_tupfile::make_test_tupfile_string;
use generators::tup::tup_config::make_tup_config_string;
use generators::tup::catch_definition::make_catch_definition_string;
use generators::tup::tupfile::make_tupfile_string;
use generators::tup::platform::{mac_os, linux, windows};

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
        //    let z= if let Some(path) = self.project_path { path };

        //    let asset = match *asset_kind {
        //      TupKind::Config => self.config(project_path, project),
        //      TupKind::TestTupfile => self.test_tupfile(project),
        //      TupKind::Tuprules => self.tuprules(project),
        //      TupKind::Tupfile => self.tupfile(project),
        //      TupKind::TupfileIni => self.tupfile_ini(project),
        //    };
        //
        //    self.assets.push(asset);
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
        let contents = make_tup_config_string(project.name(), project.kind());
        ProjectAsset::new(project_path, String::from("config.tup"), contents)
    }

    pub fn test_tupfile(&self) -> ProjectAsset {
        let asset_path = self.project_path.join("test");
        let contents = make_test_tupfile_string();
        ProjectAsset::new(asset_path, String::from("test_tupfile"), contents)
    }

    pub fn tuprules(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = make_tuprules_string(self.project);
        ProjectAsset::new(project_path, String::from("tuprules"), contents)
    }

    pub fn tupfile(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let project = &self.project;
        let contents = make_tupfile_string(project.kind());
        ProjectAsset::new(project_path, String::from("tupfile"), contents)
    }

    pub fn tupfile_ini(&self) -> ProjectAsset {
        let contents = TupfileIni::new().to_string();
        let project_path = self.project_path.clone();
        ProjectAsset::new(project_path, String::from("tup.ini"), contents)
    }

    pub fn linux(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = linux::to_string();
        ProjectAsset::new(project_path, String::from("linux.tup"), contents)
    }

    pub fn macos(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = mac_os::to_string();
        ProjectAsset::new(project_path, String::from("macos.tup"), contents)
    }

    pub fn windows(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = windows::to_string();
        ProjectAsset::new(project_path, String::from("win32.tup"), contents)
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
        let contents = make_catch_definition_string();

        ProjectAsset::new(test_src_path, String::from("catch.cpp"), contents)
    }
}
