use std::collections::HashMap;
///原始toml解析，类型，模块路径在后续处理
use serde::{Deserialize, Serialize};
use smart_string::SmartString;

pub trait RawTomlCodeFile: Sized {
    type RawPieces: for<'de> Deserialize<'de>;
    fn into_pieces(self) -> Self::RawPieces;
}

#[derive(Debug,Serialize,Deserialize)]
pub struct RawNormal {
    pub tags: Option<Vec<SmartString>>,
    pub description: Option<SmartString>,
}

//Component
#[derive(Debug,Serialize,Deserialize)]
pub struct RawComponentField {
    pub name: SmartString,
    #[serde(rename = "type")]
    pub type_spec: SmartString,
    pub default: Option<toml::Value>,
    pub description: Option<SmartString>,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawComponent {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub fields: Option<Vec<RawComponentField>>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct RawComponentFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<RawNormal>,
    #[serde(rename = "components")]
    pub component_list: Vec<RawComponent>,
}
impl RawTomlCodeFile for RawComponentFile {
    type RawPieces = Vec<RawComponent>;
    fn into_pieces(self) -> Self::RawPieces {
        self.component_list
    }
}
//Event
#[derive(Debug,Serialize,Deserialize)]
pub struct RawEventField {
    pub name: SmartString,
    #[serde(rename = "type")]
    pub type_spec: SmartString,
    pub description: Option<SmartString>,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawEvent {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub fields: Option<Vec<RawEventField>>,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawEventFile {
    pub normal: Option<RawNormal>,
    #[serde(rename = "events")]
    pub event_list: Vec<RawEvent>
}
impl RawTomlCodeFile for RawEventFile {
    type RawPieces = Vec<RawEvent>;
    fn into_pieces(self) -> Self::RawPieces {
        self.event_list
    }
}
//EntityProto
#[derive(Debug,Serialize,Deserialize)]
pub struct RawEntityProto {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub components: Vec<SmartString>,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawEntityProtoFile {
    pub normal: Option<RawNormal>,
    #[serde(rename = "entity_protos")]
    pub entity_proto_list: Vec<RawEntityProto>
}
impl RawTomlCodeFile for RawEntityProtoFile {
    type RawPieces = Vec<RawEntityProto>;
    fn into_pieces(self) -> Self::RawPieces {
        self.entity_proto_list
    }
}

//System
#[derive(Debug,Serialize,Deserialize)]
pub struct RawSystemQuery {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub components_include: Option<Vec<SmartString>>,
    pub components_exclude: Option<Vec<SmartString>>,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawSystemEventHandler {
    pub watch_for: SmartString,
    pub priority: Option<toml::Value>,
    pub logic: Option<SmartString>
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawSystemUpdate {
    pub interval: toml::Value,
    pub condition: Option<SmartString>,
    pub logic: Option<SmartString>
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RawSystemNormal {
    pub name: SmartString,
    pub description: Option<SmartString>,
    pub category: Option<SmartString>,
    pub priority: Option<toml::Value>,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawSystem {
    pub normal: RawSystemNormal,
    pub queries: Vec<RawSystemQuery>,
    pub update: Option<RawSystemUpdate>,
    pub event_handlers: Vec<RawSystemEventHandler>,
}
pub type RawSystemFile = RawSystem;
impl RawTomlCodeFile for RawSystemFile {
    type RawPieces = RawSystem;
    fn into_pieces(self) -> Self::RawPieces {
        self
    }
}
//World
#[derive(Debug, Serialize, Deserialize)]
pub struct RawWorldNormal {
    pub name: SmartString,
    pub version: SmartString,
    pub author:  SmartString
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawWorldModules {
    pub modules: HashMap<SmartString, SmartString>,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawWorldBuild {
    pub output_dir: SmartString,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawWorldCargo {

}
#[derive(Debug,Serialize,Deserialize)]
pub struct RawWorld {
    #[serde(rename = "world")]
    pub normal: RawWorldNormal,
    #[serde(flatten)]
    pub modules: RawWorldModules,
    pub build: Option<RawWorldBuild>,
    pub cargo: Option<RawWorldCargo>,
}
type RawWorldFile = RawWorld;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_component_file() {
        let toml_str = r#"
        #在一个toml文件中可以定义多个组件，normal字段不会参与转译，作为注释和元信息提供
        [normal]
        tags = ["combat", "stats"]
        description = "战斗相关组件"

        # 健康组件
        [[components]]
        name = "Health"
        description = "实体健康值"

        [[components.fields]]
        name = "value"
        type = "f32"
        default = 100.0
        description = "当前健康值"

        [[components.fields]]
        name = "max_value"
        type = "f32"
        default = 100.0
        description = "最大健康值"

        # 位置组件
        [[components]]
        name = "Position"
        description = "实体位置"

        [[components.fields]]
        name = "x"
        type = "f32"
        default = 0.0
        description = "X坐标"

        [[components.fields]]
        name = "y"
        type = "f32"
        default = 0.0
        description = "Y坐标"
        "#;
        let raw_component: RawComponentFile = toml::from_str(toml_str).unwrap();
        assert_eq!(raw_component.component_list.len(), 2)
    }
    #[test]
    fn test_parse_event_file() {
        let toml_str = r#"
        #在一个toml文件中可以定义多个事件，normal字段不会参与转译，作为注释和元信息提供
        [normal]
        tags = ["combat", "interaction"]
        description = "战斗和交互事件"

        # 伤害事件
        [[events]]
        name = "EntityDamaged"
        description = "实体受到伤害"

        [[events.fields]]
        name = "damage"
        type = "f32"
        description = "伤害值"

        [[events.fields]]
        name = "attacker"
        type = "EntityId"
        description = "攻击者ID"

        # 治疗事件
        [[events]]
        name = "EntityHealed"
        description = "实体被治疗"

        [[events.fields]]
        name = "amount"
        type = "f32"
        description = "治疗量"

        [[events.fields]]
        name = "healer"
        type = "EntityId"
        description = "治疗者ID"
        "#;
        let raw_event : RawEventFile = toml::from_str(toml_str).unwrap();
        assert_eq!(raw_event.event_list[0].name, "EntityDamaged");
        assert_eq!(raw_event.event_list.len(), 2);
    }
    #[test]
    fn test_parse_entity_proto_file() {
        let toml_str = r#"
        #在一个toml文件中可以定义多个实体原型，normal字段不会参与转译，作为注释和元信息提供
        [normal]
        tags = ["characters", "npcs"]
        description = "角色实体原型"

        # 玩家原型
        [[entity_protos]]
        name = "Player"
        components = ["combat::Health", "Position", "PlayerControlled"]
        description = "玩家角色"

        # 敌人原型
        [[entity_protos]]
        name = "Enemy"
        components = ["combat::Health", "Position", "AIControlled"]
        description = "敌人角色"

        # 物品原型
        [[entity_protos]]
        name = "HealthPotion"
        components = ["Item", "Consumable"]
        description = "治疗药水"
        "#;
        let raw_entity_proto : RawEntityProtoFile = toml::from_str(toml_str).unwrap();
        assert_eq!(raw_entity_proto.entity_proto_list.len(), 3);
    }
    #[test]
    fn test_parse_system_file() {
        let toml_str = r#"
        #一个toml文件中，只能定义一个系统
        [normal]
        name = "HealthSystem"
        description = "处理实体健康值更新"
        category = "combat"
        priority = 100

        # 组件查询定义
        [[queries]]
        name = "living_entities"
        components_include = ["combat::Health", "Position"]
        components_exclude = ["Test"]
        description = "查询所有存活实体"

        [[queries]]
        name = "damaged_entities"
        components_include = ["combat::Health", "Damage"]
        description = "查询受伤实体"

        [update]
        interval = 0.1  # 更新间隔(秒)

        # 更新条件 (Lua)
        condition = '''
        return entity.health.value > 0
        '''

        # 更新逻辑 (Lua)
        logic = '''
        entity.health.value = entity.health.value - entity.damage.amount
        entity.damage.amount = 0
        '''

        # 事件处理器
        [[event_handlers]]
        watch_for = "EntityDamaged"
        priority = 10
        logic = '''
        entity.health.value = entity.health.value - event.damage
        '''

        [[event_handlers]]
        watch_for = "EntityHealed"
        priority = 20
        logic = '''
        entity.health.value = math.min(
            entity.health.value + event.amount,
            entity.health.max_value
        )
        '''
        "#;
        let raw_system : RawSystemFile = toml::from_str(toml_str).unwrap();
        assert_eq!(raw_system.event_handlers.len(), 2);
        assert_eq!(raw_system.queries.len(),2);
        assert_eq!(raw_system.event_handlers[0].watch_for, "EntityDamaged");
        assert_eq!(raw_system.event_handlers[1].watch_for, "EntityHealed");
        assert_eq!(raw_system.queries[0].name, "living_entities");
        assert_eq!(raw_system.queries[1].name, "damaged_entities");
    }
    #[test]
    fn test_parse_world_file() {
        let toml_str = r#"
        [world]
        name = "MyAIWorld"
        version = "0.1.0"
        author = "Your Name"

        [modules]
        combat = "modules/combat"
        explore = "modules/explore"

        [build]
        output_dir = "generated"

        [cargo]
        # 标准 Cargo 配置
        "#;
        let raw_world : RawWorldFile = toml::from_str(toml_str).unwrap();
        assert_eq!(raw_world.normal.name, "MyAIWorld");
        assert_eq!(raw_world.normal.version, "0.1.0");
        assert_eq!(raw_world.normal.author, "Your Name");
        assert_eq!(raw_world.build.unwrap().output_dir, "generated");
        println!("{:?}", raw_world.modules)
    }
}