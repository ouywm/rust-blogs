# Rust-OY-Blogs

🚀 一个基于 Rust + Axum 的高性能 RBAC 权限管理系统

[![Rust Version][rustc-image]][rustc-url]
[![License][license-image]][license-url]

[rustc-image]: https://img.shields.io/badge/rustc-1.70+-blue.svg
[rustc-url]: https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: LICENSE

## 项目说明

本项目是一个基于 Rust + Axum 构建的后端 API 服务。

使用的的前端项目基于 [Art Design Pro](https://github.com/Daymychen/art-design-pro) 开发,这是一个基于 Vue3、TypeScript、Vite 和 Element-Plus 的优秀开源后台管理系统模板。

- 仓库地址: [Art Design Pro](https://github.com/Daymychen/art-design-pro)
- 技术栈: Vue3 + TypeScript + Vite + Element-Plus
- 特点: 专注于用户体验和视觉设计的后台管理系统模板
- 

### 前端项目
- 1231231

## 🌟 项目特点

### 高性能设计
- 基于 Rust 语言开发,享受零成本抽象和内存安全
- 使用 Tokio 异步运行时,高效处理并发请求
- Rbatis ORM 支持连接池和异步查询

### 完善的 RBAC 权限控制
- 基于 JWT 的无状态认证
- 细粒度的接口权限控制
- 多角色分配,权限灵活组合

## 🔨 技术栈

### 核心框架
- **Web框架**: Axum 0.8.1 - 轻量级、模块化、异步优先
- **ORM**: Rbatis 4.5.50 - 高性能异步 ORM
- **运行时**: Tokio 1.43.0 - 异步运行时
- **数据库**: MySQL 8.0 - 关系型数据库

### 认证授权
- JWT (jsonwebtoken 9.2.0) - 无状态认证
- bcrypt 0.17.0 - 密码加密

### 工具库
- serde 1.0.217 - 序列化/反序列化
- chrono 0.4 - 时间处理
- fast_log 1.6 - 日志记录
- dashmap 7.0.0 - 并发安全的 HashMap

## 📁 项目结构

```
src/
├── auth/           # JWT认证与权限验证
├── config/         # 配置管理
├── controller/     # 控制器(请求处理)
├── domain/         # 领域模型
│   ├── dto/       # 数据传输对象
│   ├── entity/    # 数据库实体
│   └── vo/        # 视图对象
├── middleware/     # 中间件(认证)
├── route/          # 路由配置
├── service/        # 业务逻辑
└── util/           # 工具函数
```



## 演示图

### 菜单
![menu](./img/menu.jpg)



### 角色分配权限
![role_p](./img/role_p.jpg)



### 文章列表
![artilelist](./img/artilelist.jpg)

## ⚙️ 配置说明

配置文件位于 `config/dev.toml`:

```toml
# 服务器配置
host = "127.0.0.1"    # 服务器地址
port = 3000           # 服务端口
name = "web"          # 服务名称
debug = true          # 调试模式

# 数据库配置
url = "mysql://user:pass@localhost/dbname"  # 数据库连接URL
pool_timeout = 30     # 连接超时时间(秒)
pool_size = 20        # 连接池大小

# JWT配置
secret = "your-jwt-secret"     # JWT密钥
expiration = 86400             # Token过期时间(秒)
algorithm = "HS256"            # 加密算法
issuer = "rust-blog"           # Token签发者

# OSS配置
access_key_id = ""            # OSS访问密钥ID
access_key_secret = ""        # OSS访问密钥密码
bucket = ""                   # OSS存储空间名称
endpoint = ""                 # OSS访问域名
bucket_domain = ""           # OSS自定义域名

# 日志配置
level = "info"               # 日志级别
```

## 🚀 快速开始

1. 克隆项目
2. 复制并修改配置文件
3. 初始化数据库
4. 运行项目



本人也是刚刚学习rust，有哪里写的不行，望各位大佬多多指导，小弟感激不尽！！

很多增删改查都是cursor写的
