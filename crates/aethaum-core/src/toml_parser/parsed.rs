use crate::toml_parser::raw::{RawComponent, RawComponentField, RawComponentFile, RawEntityProto, RawEntityProtoFile, RawEvent, RawEventField, RawEventFile, RawSystem, RawSystemEventHandler, RawSystemFile, RawSystemNormal, RawSystemQuery, RawSystemUpdate, RawTomlCodeFile};
use smart_string::SmartString;
use std::time::Duration;
use anyhow::Error;
use itertools::Itertools;
use one_or_many::OneOrMany;
use serde::Deserialize;

type LuaCode = SmartString; //TODO: 后续细化这一类型实现
pub trait TomlCode: Sized { //标记Trait, 用于约束Parser泛型
    type RawFile: RawTomlCodeFile + for<'de> serde::Deserialize<'de>;
    fn from_raw_file(raw: <Self::RawFile as RawTomlCodeFile>::RawPieces) -> Result<OneOrMany<Self>, anyhow::Error>;

}

//Type Definition
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PrimitiveType {
    Float,
    Int,
    Bool,
    Str,
    //TODO: 添加更多类型
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AethaumType {
    Primitive(PrimitiveType),
    Custom(SmartString)
}
impl AethaumType {
    pub fn is_primitive(&self) -> bool {
        matches!(self, AethaumType::Primitive(_))
    }
    pub fn is_custom(&self) -> bool {
        matches!(self, AethaumType::Custom(_))
    }
    pub fn from_type_str(type_str: &str) -> AethaumType {
        match type_str {
            "float" => AethaumType::Primitive(PrimitiveType::Float),
            "int" => AethaumType::Primitive(PrimitiveType::Int),
            "bool" => AethaumType::Primitive(PrimitiveType::Bool),
            "str" => AethaumType::Primitive(PrimitiveType::Str),

            _ => AethaumType::Custom(type_str.into()),
        }
    }
    pub fn to_rust_type(&self) -> String {
        todo!("to_rust_type")
    }
}

//Component
#[derive(Debug,PartialEq, Clone)]
pub struct ComponentField {
    pub name: SmartString,
    pub type_spec : AethaumType,
    pub default_value: Option<toml::Value>,
    pub description: Option<SmartString>
}
#[derive(Debug,PartialEq, Clone)]
pub struct Component {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub fields: Option<Vec<ComponentField>>
}
#[derive(Debug,PartialEq,Clone,Eq,Hash)]
pub struct ComponentRef {
    pub name: SmartString,
}
impl ComponentRef {
    pub fn new(name: impl Into<SmartString>) -> Self {
        Self { name: name.into() }
    }
    pub fn as_str(&self) -> &str {
        &self.name
    }
}
impl From<&str> for ComponentRef {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}
impl From<SmartString> for ComponentRef {
    fn from(s: SmartString) -> Self {
        Self::new(s)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ComponentConstraint {
    include: Option<Vec<ComponentRef>>, //必须包含的组件
    exclude: Option<Vec<ComponentRef>>, //必须不包含的组件
}
impl From<(Option<Vec<SmartString>>, Option<Vec<SmartString>>)> for ComponentConstraint {
    fn from(
        (include, exclude): (Option<Vec<SmartString>>, Option<Vec<SmartString>>),
    ) -> Self {
        Self {
            include: include.map(|v| v.into_iter().map(|s| s.into()).collect()),
            exclude: exclude.map(|v| v.into_iter().map(|s| s.into()).collect()),
        }
    }
}
//Event
#[derive(Debug,PartialEq,Clone)]
pub struct EventField {
    pub name: SmartString,
    pub type_spec : AethaumType,
    pub description: Option<SmartString>
}
#[derive(Debug,PartialEq,Clone)]
pub struct Event {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub fields: Option<Vec<EventField>>
}
//Entity Protos
#[derive(Debug,PartialEq,Clone)]
pub struct EntityProto {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub components: Vec<ComponentRef>
}
pub type SystemNormal = RawSystemNormal;
#[derive(Debug,PartialEq,Clone)]
pub struct SystemQuery {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub component_constraint: ComponentConstraint
}
#[derive(Debug,PartialEq,Clone)]
pub struct SystemEventHandler {
    pub watch_for: SmartString,
    pub priority: u32,
    pub logic: Option<LuaCode>
}
#[derive(Debug,PartialEq,Clone)]
pub struct SystemUpdate {
    pub interval: Duration,
    pub condition: Option<LuaCode>,
    pub logic: Option<LuaCode>
}
#[derive(Debug,PartialEq,Clone)]
pub struct System {
    pub normal: SystemNormal,
    pub queries: Vec<SystemQuery>,
    pub update: Option<SystemUpdate>,
    pub event_handlers: Vec<SystemEventHandler>
}
//TomlCode Mark
impl TomlCode for Component {
    type RawFile = RawComponentFile;
    fn from_raw_file(raw: <Self::RawFile as RawTomlCodeFile>::RawPieces) -> Result<OneOrMany<Self>, Error> {
        Ok(OneOrMany::from_iter(
            raw.into_iter().map(|x| x.into())
        ))
    }
}
impl TomlCode for Event {
    type RawFile = RawEventFile;
    fn from_raw_file(raw: <Self::RawFile as RawTomlCodeFile>::RawPieces) -> Result<OneOrMany<Self>, Error> {
        Ok(OneOrMany::from_iter(
            raw.into_iter().map(|x| x.into())
        ))
    }
}
impl TomlCode for EntityProto {
    type RawFile = RawEntityProtoFile;
    fn from_raw_file(raw: <Self::RawFile as RawTomlCodeFile>::RawPieces) -> Result<OneOrMany<Self>, Error> {
        Ok(OneOrMany::from_iter(
            raw.into_iter().map(|x| x.into())
        ))
    }
}
impl TomlCode for System {
    type RawFile = RawSystemFile;
    fn from_raw_file(raw: <Self::RawFile as RawTomlCodeFile>::RawPieces) -> Result<OneOrMany<Self>, Error> {
        Ok(OneOrMany::One(Box::new(raw.into_pieces().try_into()?)))
    }
}


//Raw Transformation
impl From<RawComponentField> for ComponentField {
    fn from(value: RawComponentField) -> Self {
        ComponentField {
            name: value.name,
            type_spec: AethaumType::from_type_str(&value.type_spec),
            default_value: value.default,
            description: value.description,
        }
    }
}
impl From<RawComponent> for Component {
    fn from(value: RawComponent) -> Self {
        Component {
            name: value.name,
            fields: match value.fields {
                None => None,
                Some(fields) => Some(fields.into_iter().map(|x| x.into()).collect())
            },
            description: value.description,
        }
    }
}
impl From<RawEventField> for EventField {
    fn from(value: RawEventField) -> Self {
        EventField {
            name: value.name,
            type_spec: AethaumType::from_type_str(&value.type_spec),
            description: value.description,
        }
    }
}
impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        Event {
            name: value.name,
            fields: match value.fields {
                None => None,
                Some(fields) => Some(fields.into_iter().map(|x| x.into()).collect())
            },
            description: value.description,
        }
    }
}
impl From<RawEntityProto> for EntityProto {
    fn from(value: RawEntityProto) -> Self {
        EntityProto {
            name: value.name,
            description: value.description,
            components: value.components.into_iter().map(|x| x.into()).collect(),
        }
    }
}
impl From<RawSystemQuery> for SystemQuery {
    fn from(value: RawSystemQuery) -> Self {
        SystemQuery {
            name: value.name,
            description: value.description,
            component_constraint: ComponentConstraint::from(
                (value.components_include, value.components_exclude)
            )
        }
    }
}
impl TryFrom<RawSystemEventHandler> for SystemEventHandler {
    type Error = anyhow::Error; //TODO: better error type further
    fn try_from(value: RawSystemEventHandler) -> Result<Self, Self::Error> {
        Ok(SystemEventHandler {
            watch_for: value.watch_for,
            priority: match value.priority {
                Some(priority) => {
                   match priority {
                       toml::Value::Integer(i) => {
                           if i < 0 {
                               return Err(anyhow::anyhow!("Priority must be positive"));
                           }
                           i as u32
                       },
                       _ => return Err(anyhow::anyhow!("Priority must be an integer")),
                   }
                },
                None => 0
            },
            logic: value.logic.into() // For Further Version, LuaCode type might not be SmartString
        })
    }
}
impl TryFrom<RawSystemUpdate> for SystemUpdate {
    type Error = anyhow::Error; //TODO: better error type further
    fn try_from(value: RawSystemUpdate) -> Result<Self, Self::Error> {
        Ok(SystemUpdate {
            interval: match value.interval {
                toml::Value::Integer(i) => {
                    if i <= 0 {
                        return Err(anyhow::anyhow!("Interval must be positive"));
                    }
                    Duration::from_secs(i as u64)
                }
                toml::Value::Float(f) => {
                    if f <= 0.0 {
                        return Err(anyhow::anyhow!("Interval must be positive"));
                    }
                    Duration::from_secs_f64(f)
                }
                _ => return Err(anyhow::anyhow!("Interval must be a number")),
            },
            condition: value.condition.into(), // For Further Version, LuaCode type might not be SmartString
            logic: value.logic.into(), // For Further Version, LuaCode type might not be SmartString
        })
    }
}
impl TryFrom<RawSystem> for System {
    type Error = anyhow::Error; //TODO: better error type further
    fn try_from(value: RawSystem) -> Result<Self, Self::Error> {
        Ok(System {
            normal: value.normal.into(),
            queries: value.queries.into_iter().map(|q| q.into()).collect(),
            update: value.update.map(TryInto::try_into).transpose()?,
            event_handlers: value.event_handlers
                .into_iter()
                .map(|h| h.try_into())
                .try_collect()?,
        })
    }
}

