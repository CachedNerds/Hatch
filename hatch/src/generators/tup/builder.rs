use constants::CATCH_HEADER_URL;
use generators::platform_kind::PlatformKind;
use generators::project_asset::ProjectAsset;
use generators::tup::platform::{linux, mac_os, windows};
use generators::tup::tup_kind::TupKind;
use generators::tup::catch_definition;
use generators::tup::catch_header;
use generators::tup::test_tupfile;
use generators::tup::tup_config;
use generators::tup::tupfile;
use generators::tup::tupfile_ini;
use generators::tup::tuprules;
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
        let contents = tup_config::make_string(project.name(), project.kind());
        ProjectAsset::new(project_path, tup_config::file_name(), contents)
    }

    pub fn add_test_tupfile(&self) -> ProjectAsset {
        let asset_path = self.project_path.join("test/");
        let contents = test_tupfile::make_string();
        ProjectAsset::new(asset_path, test_tupfile::file_name(), contents)
    }

    pub fn add_tuprules(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = tuprules::make_string(self.project);
        ProjectAsset::new(project_path, tuprules::file_name(), contents)
    }

    pub fn add_tupfile(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let project = &self.project;
        let contents = tupfile::make_string(project.kind());
        ProjectAsset::new(project_path, tupfile::file_name(), contents)
    }

    pub fn add_tupfile_ini(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = tupfile_ini::make_string();
        ProjectAsset::new(project_path, tupfile_ini::file_name(), contents)
    }

    pub fn add_linux_platform_tup_file(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = linux::make_string();
        ProjectAsset::new(project_path, linux::file_name(), contents)
    }

    pub fn add_macos_platform_tup_file(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = mac_os::make_string();
        ProjectAsset::new(project_path, mac_os::file_name(), contents)
    }

    pub fn add_windows_platform_tup_file(&self) -> ProjectAsset {
        let project_path = self.project_path.clone();
        let contents = windows::make_string();
        ProjectAsset::new(project_path, windows::file_name(), contents)
    }

    pub fn add_catch_header(&self) -> HatchResult<ProjectAsset> {
        let test_src_path = self.project_path.join("test/src");
        if !test_src_path.join(catch_header::file_name()).exists() {
            let res = (|| -> HatchResult<ProjectAsset> {
                let mut resp = reqwest::get(CATCH_HEADER_URL)?;
                let content = resp.text()?;

                Ok(ProjectAsset::new(
                    test_src_path,
                    catch_header::file_name(),
                    content,
                ))
            })().with_context(|e| format!("failed to generate {} : {}", catch_header::file_name(), e))?;

            Ok(res)
        } else {
            Err(NullError)?
        }
    }

    pub fn add_catch_definition(&self) -> ProjectAsset {
        let test_src_path = self.project_path.join("test/src");
        let contents = catch_definition::make_string();

        ProjectAsset::new(test_src_path, catch_definition::file_name(), contents)
    }
}
