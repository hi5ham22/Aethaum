use std::marker::PhantomData;
use one_or_many::OneOrMany;
use crate::toml_parser::parsed::TomlCode;
use anyhow::Result;
use crate::toml_parser;
use crate::toml_parser::raw::RawTomlCodeFile;

mod raw;
pub mod parsed;
pub mod error;

pub struct Parser<T> {
    raw_content: String,
    is_parsed: bool,
    _type_marker: PhantomData<T>
}
impl<T: TomlCode> Parser<T> {
    pub fn new(raw_content: String) -> Self {
        Self {
            raw_content,
            is_parsed: false,
            _type_marker: PhantomData
        }
    }
    pub fn parse(&mut self) -> Result<OneOrMany<T>>
    {
        // 解析 TOML 文件为 RawFile 类型
        let raw_file: T::RawFile = toml::from_str(&self.raw_content)?;
        // 提取我们关心的部分
        let pieces = raw_file.into_pieces();
        // 转换为最终类型
        let result = T::from_raw_file(pieces)?;
        self.is_parsed = true;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;
    use crate::toml_parser::parsed::{Component, Event, EntityProto, System};

    /// 测试Parser::new函数
    /// 验证构造函数正确初始化所有字段
    #[test]
    fn test_parser_new() {
        let content = "test content".to_string();
        let parser: Parser<Component> = Parser::new(content.clone());

        assert_eq!(parser.raw_content, content);
    }

    /// 测试空字符串创建Parser
    /// 验证边界情况处理
    #[test]
    fn test_parser_new_empty_content() {
        let content = "".to_string();
        let parser: Parser<Component> = Parser::new(content);

        assert_eq!(parser.raw_content, ""); // 成功创建即为正确
    }

    /// 测试成功解析Component TOML内容
    /// 验证正常流程：TOML解析 -> into_pieces -> from_raw_file
    #[test]
    fn test_parse_component_success() {
        let toml_content = r#"
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
            type = "float"
            default = 100.0
            description = "当前健康值"

            [[components.fields]]
            name = "max_value"
            type = "float"
            default = 100.0
            description = "最大健康值"

            # 位置组件
            [[components]]
            name = "Position"
            description = "实体位置"

            [[components.fields]]
            name = "x"
            type = "float"
            default = 0.0
            description = "X坐标"

            [[components.fields]]
            name = "y"
            type = "float"
            default = 0.0
            description = "Y坐标"
        "#.to_string();

        let mut parser: Parser<Component> = Parser::new(toml_content);
        let result = parser.parse();

        assert!(result.is_ok());

        let components = result.unwrap();
        match components {
            OneOrMany::Many(vec) => {
                assert_eq!(vec.len(), 2);
                assert_eq!(vec[0].name, "Health");
                assert!(vec[0].fields.is_some());
                let fields = vec[0].fields.as_ref().unwrap();
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "value");
                assert_eq!(fields[1].name, "max_value");
            },
            _ => panic!("Expected Many variant"),
        }
    }

    /// 测试成功解析Event TOML内容
    #[test]
    fn test_parse_event_success() {
        let toml_content = r#"
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
            type = "float"
            description = "伤害值"

            [[events.fields]]
            name = "attacker"
            type = "EntityId" #原生类型
            description = "攻击者ID"

            # 治疗事件
            [[events]]
            name = "EntityHealed"
            description = "实体被治疗"

            [[events.fields]]
            name = "amount"
            type = "float"
            description = "治疗量"

            [[events.fields]]
            name = "healer"
            type = "EntityId"
            description = "治疗者ID"
        "#.to_string();

        let mut parser: Parser<Event> = Parser::new(toml_content);
        let result = parser.parse();

        assert!(result.is_ok());

        let events = result.unwrap();
        match events {
            OneOrMany::Many(vec) => {
                assert_eq!(vec.len(), 2);
                assert_eq!(vec[0].name, "EntityDamaged");
                assert!(vec[0].fields.is_some());
                let fields = vec[0].fields.as_ref().unwrap();
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "damage");
            },
            _ => panic!("Expected Many variant"),
        }
    }

    /// 测试成功解析EntityProto TOML内容
    #[test]
    fn test_parse_entity_proto_success() {
        let toml_content = r#"
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
        "#.to_string();

        let mut parser: Parser<EntityProto> = Parser::new(toml_content);
        let result = parser.parse();

        assert!(result.is_ok());

        let entity_protos = result.unwrap();
        match entity_protos {
            OneOrMany::Many(vec) => {
                assert_eq!(vec.len(), 3);
                assert_eq!(vec[0].name, "Player");
                assert_eq!(vec[0].components.len(), 3);
            },
            _ => panic!("Expected Many variant"),
        }
    }

    /// 测试成功解析System TOML内容
    #[test]
    fn test_parse_system_success() {
        let toml_content = r#"
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
        "#.to_string();

        let mut parser: Parser<System> = Parser::new(toml_content);
        let result = parser.parse();

        assert!(result.is_ok());

        let systems = result.unwrap();
        match systems {
            OneOrMany::One(sys) => {
                assert_eq!(sys.queries.len(),2);
                assert_eq!(sys.event_handlers.len(),2);
                assert_eq!(sys.update.unwrap().interval, Duration::from_secs_f64(0.1));
            },
            _ => panic!("Expected One variant"),
        }
    }

    /// 测试无效TOML格式解析
    /// 验证错误处理：TOML解析失败情况
    #[test]
    fn test_parse_invalid_toml() {
        let invalid_toml = "invalid [toml content".to_string();
        let mut parser: Parser<Component> = Parser::new(invalid_toml);
        let result = parser.parse();

        assert!(result.is_err());
    }

    /// 测试空内容解析
    /// 验证边界情况处理
    #[test]
    fn test_parse_empty_content() {
        let empty_content = "".to_string();
        let mut parser: Parser<Component> = Parser::new(empty_content);
        let result = parser.parse();

        assert!(result.is_err());
    }

    /// 测试解析不完整TOML
    #[test]
    fn test_parse_incomplete_toml() {
        let incomplete_toml = r#"
            [[components]]
            name = "Health"
            # 缺少fields
        "#.to_string();

        let mut parser: Parser<Component> = Parser::new(incomplete_toml);
        let result = parser.parse();

        assert!(result.is_ok()); // 即使没有fields也应该解析成功
    }
}

