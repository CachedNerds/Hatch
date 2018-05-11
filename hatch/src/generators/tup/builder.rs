use constants::CATCH_HEADER_URL;
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
use generators::tup::catch_header::catch_header_filename;
use generators::project_asset::ProjectAsset;
use generators::tup::tup_kind::TupKind;
use generators::platform_kind::PlatformKind;

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
            TupKind::Config => self.add_tup_config(),
            TupKind::TestTupfile => self.add_test_tupfile(),
            TupKind::Tuprules => self.add_tuprules(),
            TupKind::Tupfile => self.add_tupfile(),
            TupKind::TupfileIni => self.add_tupfile_ini(),
        };

        self.assets.push(asset);
    }

    pub fn platform(&mut self, asset_kind: &PlatformKind) {
        let asset = match *asset_kind {
            PlatformKind::Linux => self.add_linux_platform_tup_file(),
            PlatformKind::MacOS => self.add_macos_platform_tup_file(),
            PlatformKind::Windows => self.add_windows_platform_tup_file(),
        };

        self.assets.push(asset);
    }

    pub fn add_tup_config(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let project = &self.project;
        let contents = make_tup_config_string(project.name(), project.kind());
        ProjectAsset::new(project_path, String::from("config.tup"), contents)
    }

    pub fn add_test_tupfile(&self) -> ProjectAsset {
        let asset_path = self.project_path.join("test");
        let contents = make_test_tupfile_string();
        ProjectAsset::new(asset_path, String::from("test_tupfile"), contents)
    }

    pub fn add_tuprules(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = make_tuprules_string(self.project);
        ProjectAsset::new(project_path, String::from("tuprules"), contents)
    }

    pub fn add_tupfile(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let project = &self.project;
        let contents = make_tupfile_string(project.kind());
        ProjectAsset::new(project_path, String::from("tupfile"), contents)
    }

    pub fn add_tupfile_ini(&self) -> ProjectAsset {
        let contents = TupfileIni::new().to_string();
        let project_path = self.project_path.clone();
        ProjectAsset::new(project_path, String::from("tup.ini"), contents)
    }

    pub fn add_linux_platform_tup_file(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = linux::to_string();
        ProjectAsset::new(project_path, String::from("linux.tup"), contents)
    }

    pub fn add_macos_platform_tup_file(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = mac_os::to_string();
        ProjectAsset::new(project_path, String::from("macosx.tup"), contents)
    }

    pub fn add_windows_platform_tup_file(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = windows::to_string();
        ProjectAsset::new(project_path, String::from("win32.tup"), contents)
    }

    pub fn add_catch_header(&self) -> HatchResult<ProjectAsset> {
        let test_src_path = self.project_path.join("test/src");
        if !test_src_path.join(catch_header_filename()).exists() {
            let res = (|| -> HatchResult<ProjectAsset> {
                let mut resp = reqwest::get(CATCH_HEADER_URL)?;
                let content = resp.text()?;

                Ok(ProjectAsset::new(
                    test_src_path,
                    catch_header_filename(),
                    content,
                ))
            })().with_context(|e| format!("failed to generate catch.hpp : {}", e))?;

            Ok(res)
        } else {
            Err(NullError)?
        }
    }

    pub fn add_catch_definition(&self) -> ProjectAsset {
        let test_src_path = self.project_path.join("test/src");
        let contents = make_catch_definition_string();

        ProjectAsset::new(test_src_path, String::from("catch.cpp"), contents)
    }
}
