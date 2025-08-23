use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use smart_string::SmartString;
use crate::ecs::module::{AethaumProject, EcsModule, EcsModuleTree};
use crate::toml_parser::parsed::{Component, EntityProto, Event, System, World};
use crate::toml_parser::raw::{RawComponent, RawComponentFile, RawEntityProto, RawEntityProtoFile, RawEvent, RawEventFile, RawSystem, RawSystemFile, RawTomlCodeFile, RawWorld};
use itertools::Itertools;
use one_or_many::OneOrMany;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum ModuleFileLoaderError {
    #[error("Failed to load module file, {0}")]
    IoError(#[from] std::io::Error),
    #[error("Illegal toml file, {0}")]
    IllegalToml(#[from] toml::de::Error),
    #[error("fail to convert: {0}")]
    ConversionError(#[from] anyhow::Error),
    #[error("Multiple errors occurred during loading:\n{}",
        .errors.iter().map(|e| format!("  - {}", e)).collect::<Vec<_>>().join("\n"))]
    Multiple {
        errors: Vec<ModuleFileLoaderError>
    }
}
impl ModuleFileLoaderError {
    pub fn raise_multiple(errors: Vec<ModuleFileLoaderError>) -> ModuleFileLoaderError {
        ModuleFileLoaderError::Multiple { errors }
    }
}
#[derive(Debug, Error)]
pub enum ProjectLoaderError {
    #[error("world.toml not found in the project.")]
    MissingWorld,
    #[error("module '{0}' not found")]
    MissingModule(SmartString),
    #[error("{0}")]
    FileError(#[from] ModuleFileLoaderError)
}
impl From<std::io::Error> for ProjectLoaderError {
    fn from(error: std::io::Error) -> Self {
        ProjectLoaderError::FileError(ModuleFileLoaderError::IoError(error))
    }
}
impl From<toml::de::Error> for ProjectLoaderError {
    fn from(error: toml::de::Error) -> Self {
        ProjectLoaderError::FileError(ModuleFileLoaderError::IllegalToml(error))
    }
}
impl From<anyhow::Error> for ProjectLoaderError {
    fn from(error: anyhow::Error) -> Self {
        ProjectLoaderError::FileError(ModuleFileLoaderError::ConversionError(error))
    }
}
impl ProjectLoaderError {
    pub fn raise_missing_world() -> Self {
        ProjectLoaderError::MissingWorld
    }
    pub fn raise_missing_module(module_name: SmartString) -> Self {
        ProjectLoaderError::MissingModule(module_name)
    }
    pub fn raise_file_error(error: ModuleFileLoaderError) -> Self {
        ProjectLoaderError::FileError(error)
    }
}

trait Loadable: Sized {
    type RawFile: for<'de> Deserialize<'de>;
    type RawType: for<'de> Deserialize<'de>;
    type Error: Into<ProjectLoaderError>;
    fn try_load(path: &Path) -> Result<OneOrMany<Self>, Self::Error>;
}
impl Loadable for System {
    type RawFile = RawSystemFile;
    type RawType = RawSystem;
    type Error = ModuleFileLoaderError;
    fn try_load(path: &Path) -> Result<OneOrMany<Self>, Self::Error> {
        let file_content = load_file(path)?;
        let raw_system_file: RawSystemFile = toml::from_str(&file_content)?;
        let raw_system = raw_system_file.into_pieces();
        let system = System::try_from(raw_system)?;
        Ok(OneOrMany::One(Box::new(system)))
    }
}
impl Loadable for Component {
    type RawFile = RawComponentFile;
    type RawType = RawComponent;
    type Error = ModuleFileLoaderError;
    fn try_load(path: &Path) -> Result<OneOrMany<Self>, Self::Error> {
        let file_content = load_file(path)?;
        let raw_component_file: RawComponentFile = toml::from_str(&file_content)?;
        let raw_components = raw_component_file.into_pieces();
        Ok(OneOrMany::Many(
            raw_components.into_iter().map(|raw_component| Component::from(raw_component)).collect()
        ))
    }
}
impl Loadable for Event {
    type RawFile = RawEventFile;
    type RawType = RawEvent;
    type Error = ModuleFileLoaderError;
    fn try_load(path: &Path) -> Result<OneOrMany<Self>, Self::Error> {
        let file_content = load_file(path)?;
        let raw_event_file: RawEventFile = toml::from_str(&file_content)?;
        let raw_events = raw_event_file.into_pieces();
        Ok(OneOrMany::Many(
            raw_events.into_iter().map(|raw_event| Event::from(raw_event)).collect()
        ))
    }
}
impl Loadable for EntityProto {
    type RawFile = RawEntityProtoFile;
    type RawType = RawEntityProto;
    type Error = ModuleFileLoaderError;
    fn try_load(path: &Path) -> Result<OneOrMany<Self>, Self::Error> {
        let file_content = load_file(path)?;
        let raw_entity_proto_file: RawEntityProtoFile = toml::from_str(&file_content)?;
        let raw_entity_protos = raw_entity_proto_file.into_pieces();

        let mut errors = Vec::new();
        let mut entity_protos = Vec::new();
        for raw_entity_proto in raw_entity_protos {
            match EntityProto::try_from(raw_entity_proto) {
                Ok(entity_proto) => entity_protos.push(entity_proto),
                Err(error) => errors.push(error.into())
            }
        }
        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors.pop().unwrap());
            }else {
                return Err(ModuleFileLoaderError::raise_multiple(errors));
            }
        }
        Ok(OneOrMany::Many(entity_protos))
    }
}
impl Loadable for World {
    type RawFile = RawWorld;
    type RawType = RawWorld;
    type Error = ProjectLoaderError;
    fn try_load(path: &Path) -> Result<OneOrMany<Self>, Self::Error> {
        let file_content = load_file(path)?;
        let raw_world: RawWorld = toml::from_str(&file_content)?;
        let world = World::from(raw_world);
        Ok(OneOrMany::One(Box::new(world)))
    }
}

pub struct ModuleFileLoader {
    base_path: PathBuf,
    module_name: SmartString
}
impl ModuleFileLoader {
    pub fn new(base_path: PathBuf, module_name: SmartString) -> Self {
        ModuleFileLoader {
            base_path,
            module_name
        }
    }
    pub fn load(self) -> Result<EcsModule, ModuleFileLoaderError> {
        let systems = self.load_parts(self.base_path.join("systems"))?;
        let components = self.load_parts(self.base_path.join("components"))?;
        let events = self.load_parts(self.base_path.join("events"))?;
        let entity_protos = self.load_parts(self.base_path.join("entity_protos"))?;
        Ok(
            EcsModule::new_empty(self.module_name)
                .with_option_components(components)
                .with_option_events(events)
                .with_option_entity_protos(entity_protos)
                .with_option_systems(systems)
        )
    }
    fn load_parts<T: Loadable<Error = ModuleFileLoaderError>>(&self, dir_path: impl AsRef<Path>) -> Result<Option<Vec<T>>, <T as Loadable>::Error>
    {
        if !dir_path.as_ref().exists() {
            return Ok(None); //TODO: better distinguish the None and the error
        }
        let paths = list_dir(dir_path)?;
        if paths.is_empty() {
            return Ok(None);
        }
        let mut parts = Vec::new();
        let mut errors = Vec::new();

        for path in paths {
            match T::try_load(path.as_path()) {
                Ok(part) => parts.push(part),
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors.pop().unwrap());//ROBUST: there must be one element in the errors
            } else {
                return Err(ModuleFileLoaderError::Multiple { errors });
            }
        }

        Ok(Some(parts.into_iter().flatten().collect())) //TODO: try reduce the collect call
    }
}
fn load_file(path: impl AsRef<Path>) -> Result<String,std::io::Error> {
    if path.as_ref().exists() {
        std::fs::read_to_string(path)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}
fn list_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>,std::io::Error> {
    match fs::read_dir(path) {
        Ok(entries) => {
            entries.into_iter()
                .map(|entry| {
                    match entry {
                        Ok(entry) => Ok(entry.path()),
                        Err(error) => Err(error)
                    }
                })
                .try_collect()
        }
        Err(error) => Err(error)
    }
}
pub struct ProjectLoader {
    base_path: PathBuf
}
impl ProjectLoader {
    pub fn new(base_path: PathBuf) -> Self {
        ProjectLoader {
            base_path
        }
    }
    pub fn load(self) -> Result<AethaumProject, ProjectLoaderError> {
        let world_toml = World::try_load(self.base_path.join("world.toml").as_path())?;
        let world_toml = match world_toml {
            OneOrMany::One(world_toml) => world_toml,
            _ => unreachable!("World::try_load always yield OneOrMany::One"),
        };

        let mut errors = Vec::new();
        let mut modules = Vec::new();
        for (module_name, module_base_path) in world_toml.modules.modules.iter() {
            match ModuleFileLoader::new(self.base_path.join(module_base_path), module_name.clone()).load() {
                Ok(module) => modules.push(module),
                Err(error) => errors.push(error)
            }
        }
        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors.pop().unwrap().into());//ROBUST: there must be one element in the errors
            } else {
                return Err(ModuleFileLoaderError::raise_multiple(errors).into());
            }
        }
        Ok(AethaumProject::new(self.base_path, *world_toml, EcsModuleTree::new_empty().with_modules(modules)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;
    #[test]
    fn test_load_file() {
        let string = load_file(Path::new(r#"D:\Aethaum\test_project\world.toml"#)).unwrap();
        println!("{}", string)
    }
    #[test]
    fn test_list_dir() {
        let base_path = Path::new(r#"D:\Aethaum\test_project\modules\explore"#);
        let paths = list_dir(base_path.join("components")).unwrap();
        for path in paths {
            println!("{}", path.display());
        }
    }
    #[test]
    fn test_load_single_module() {
        let base_path = Path::new(r#"D:\Aethaum\test_project\modules\explore"#);
        let module = ModuleFileLoader::new(base_path.to_path_buf(), "explore".into()).load().unwrap();
        assert_eq!(module.name, "explore")
    }
    #[test]
    fn test_load_project() {
        let base_path = Path::new(r#"D:\Aethaum\test_project"#);
        let project = ProjectLoader::new(base_path.to_path_buf()).load().unwrap();
        assert_eq!(project.world.normal.name, "MyAIWorld");
    }
}