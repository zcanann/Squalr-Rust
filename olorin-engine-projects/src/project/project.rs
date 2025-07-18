use olorin_engine_api::structures::{
    processes::process_icon::ProcessIcon,
    projects::{
        project_info::ProjectInfo,
        project_items::{built_in_types::project_item_type_directory::ProjectItemTypeDirectory, project_item::ProjectItem},
        project_manifest::ProjectManifest,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::Path,
};

use super::serialization::serializable_project_file::SerializableProjectFile;

#[derive(Serialize, Deserialize)]
pub struct Project {
    project_info: ProjectInfo,

    #[serde(rename = "project")]
    project_root: ProjectItem,
}

impl Project {
    pub const PROJECT_FILE: &'static str = "project.json";
    pub const PROJECT_DIR: &'static str = "project";

    pub fn new(
        project_info: ProjectInfo,
        project_root: ProjectItem,
    ) -> Self {
        Self { project_info, project_root }
    }

    /// Creates a new project and writes it to disk.
    pub fn create_new_project(path: &Path) -> anyhow::Result<Self> {
        if path.exists() && path.read_dir()?.next().is_some() {
            anyhow::bail!("Cannot create project: directory already contains files.");
        }

        fs::create_dir_all(path)?;

        let project_info = ProjectInfo::new(path.to_path_buf(), None, ProjectManifest::default());

        let project_root = ProjectItemTypeDirectory::new_project_item(path);
        let mut project = Self { project_info, project_root };

        project.save_to_path(path, true)?;

        Ok(project)
    }

    pub fn save(
        &mut self,
        save_even_if_unchanged: bool,
    ) -> anyhow::Result<()> {
        let project_path = self.project_info.get_path().to_owned();

        self.save_to_path(&project_path, save_even_if_unchanged)
    }

    pub fn get_name(&self) -> &str {
        self.project_info.get_name()
    }

    pub fn get_project_manifest(&self) -> &ProjectManifest {
        &self.project_info.get_project_manifest()
    }

    pub fn get_project_info(&self) -> &ProjectInfo {
        &self.project_info
    }

    pub fn get_project_info_mut(&mut self) -> &mut ProjectInfo {
        &mut self.project_info
    }

    pub fn set_project_icon(
        &mut self,
        project_icon: Option<ProcessIcon>,
    ) {
        self.project_info.set_project_icon(project_icon);
    }

    pub fn get_project_root(&self) -> &ProjectItem {
        &self.project_root
    }

    pub fn get_project_root_mut(&mut self) -> &mut ProjectItem {
        &mut self.project_root
    }
}
