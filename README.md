# Aethaum

[简体中文](README.md) | [English](README_en.md)

**重要**：此项目仍处于初期开发阶段，目前没有可用版本



一个用于编写AI智能体可交互世界的声明式框架

Aethaum 让你能够通过简单的 TOML 配置来定义复杂的游戏世界和AI交互环境，无需编写复杂的Rust代码。

## 🎯 核心理念

- **声明式配置**：使用 TOML 定义 ECS 结构
- **逻辑分离**：配置与逻辑分离，Lua 处理运行时逻辑
- **自动转译**：TOML 配置自动转译为高性能 Rust 代码
- **灵活扩展**：生成的 Rust 项目可直接扩展和定制

## 🚀 特性

- 📝 **基于 TOML 的 ECS 配置** - 简洁易读的配置语法
- 🐍 **Lua 脚本逻辑** - 灵活的运行时逻辑编写
- 🦀 **Bevy ECS 集成** - 强大的 ECS 引擎支持
- 🔥 **热重载支持** - 开发时实时更新
- 🎮 **AI 友好** - 专为 AI 智能体交互设计

## 🏗️ 项目架构

world/
├── world.toml                 # 世界配置文件
├── systems/                   # 系统定义目录
├── components/                # 组件定义目录
├── events/                    # 事件定义目录
└── entity_protos/             # 实体原型目录
## 📋 配置详解

### 🌍 world.toml - 世界配置

```toml
[world]
name = "MyAIWorld"
version = "0.1.0"
author = "Your Name"

[includes]
systems = [
    "movement.toml",
    "combat.toml"
]
components = [
    "health.toml",
    "position.toml"
]
events = [
    "damage.toml",
    "death.toml"
]
entity_protos = [
    "player.toml",
    "enemy.toml"
]

[build]
output_dir = "generated"

[cargo]
# 标准 Cargo 配置
```

### ⚙️ systems/*.toml - 系统定义

```toml
#一个toml文件中，只能定义一个系统
[normal]
name = "HealthSystem"
description = "处理实体健康值更新"
category = "combat"
priority = 100

# 组件查询定义
[[queries]]
name = "living_entities"
components = ["Health", "Position"]
description = "查询所有存活实体"

[[queries]]
name = "damaged_entities"
components = ["Health", "Damage"]
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
```

### 🧩 components/*.toml - 组件定义

```toml
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
```

### ⚡ events/*.toml - 事件定义

```toml
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
```

### 🏗️ entity_protos/*.toml - 实体原型

```toml
#在一个toml文件中可以定义多个实体原型，normal字段不会参与转译，作为注释和元信息提供
[normal]
tags = ["characters", "npcs"]
description = "角色实体原型"

# 玩家原型
[[entity_protos]]
name = "Player"
components = ["Health", "Position", "PlayerControlled"]
description = "玩家角色"

# 敌人原型
[[entity_protos]]
name = "Enemy"
components = ["Health", "Position", "AIControlled"]
description = "敌人角色"

# 物品原型
[[entity_protos]]
name = "HealthPotion"
components = ["Item", "Consumable"]
description = "治疗药水"
```

## 🚀 快速开始

- 正在开发中，目前不处于可用阶段

## 🛠️ 开发流程

1. **定义组件** - 在 `components/` 目录下创建 TOML 文件
2. **创建系统** - 在 `systems/` 目录下定义系统逻辑
3. **设计事件** - 在 `events/` 目录下定义事件结构
4. **构建原型** - 在 `entity_protos/` 目录下创建实体模板
5. **配置世界** - 编辑 `world.toml` 包含所有定义
6. **构建运行** - 使用 CLI 工具构建和运行

## 🎯 适用场景

- 🤖 **AI 训练环境** - 为 AI 智能体提供简单交互世界
- 🤖 **AI 角色扮演** - 通过与世界交互，为 AI 角色提供可以转化为记忆的经历
- 🎮 **原型开发** - 快速构建小游戏原型
- 🔬 **模拟实验** - 复杂系统行为模拟

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT