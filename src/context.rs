use crate::config::{ApplicationConfig, JwtConfig};
use crate::service::sys_menu_service::SysMenuService;
use crate::service::sys_role_service::SysRoleService;
use crate::service::sys_user_service::SysUserService;
use chrono::{DateTime, Local};
use dashmap::DashMap;
use fast_log::plugin::console::ConsoleAppender;
use fast_log::plugin::file_split::{FileSplitAppender, Packer, RollingType};
use log::{log, LevelFilter};
use rbatis::intercept_log::LogInterceptor;
use rbatis::RBatis;
use std::collections::HashSet;
use std::fs;
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use uuid::Uuid;
use crate::service::article_service::ArticleService;
use crate::service::category_service::CategoryService;
use crate::service::comment_service::CommentService;
use crate::service::oss_service::OssService;

/// 学习别人的 原项目: https://github.com/rbatis/abs_admin
/// Service CONTEXT
pub static CONTEXT: LazyLock<ServiceContext> = LazyLock::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::context::CONTEXT.rb
    };
}
#[derive(Clone)]
pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: RBatis,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_menu_service: SysMenuService,
    pub oss_service: OssService,
    pub category_service: CategoryService,
    pub article_service: ArticleService,
    pub comment_service: CommentService
}


impl ServiceContext {
    pub async fn init_database(&self) {
        log::debug!("数据库池初始化 ({})...", self.config.db_url);
        self.rb
            .link(rbdc_mysql::driver::MysqlDriver {}, &self.config.db_url)
            .await
            .expect("rbatis pool init fail!");
        let pool = self.rb.get_pool().unwrap();
        self.rb
            .get_intercept::<LogInterceptor>()
            .expect("rbatis LogInterceptor init fail!")
            .set_level_filter(log::max_level());
        pool.set_max_open_conns(self.config.db_pool_len).await;
        pool.set_timeout(Some(Duration::from_secs(self.config.db_pool_timeout)))
            .await;
        log::debug!(
            "rabtis 池初始化成功！池状态 = {}",
            self.rb.get_pool().expect("pool not init!").state().await
        );
    }

    pub async fn initialize(&self) -> ServiceContext {
        self.init_database().await;
        Self {
            rb: self.rb.clone(),
            config: self.config.clone(),
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_menu_service: SysMenuService {},
            oss_service: OssService {},
            category_service: CategoryService{},
            article_service: ArticleService{},
            comment_service: CommentService{},
        }
    }

    pub async fn get_jwt_config(&self) -> JwtConfig {
        JwtConfig {
            secret: self.config.secret.clone(),
            expiration: self.config.expiration,
            algorithm: self.config.algorithm.clone(),
            issuer: self.config.issuer.clone(),
        }
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let mut config = ApplicationConfig::init();
        ServiceContext {
            rb: {
                let rb = RBatis::new();

                if cfg!(debug_assertions) == false && config.debug.eq(&true) {
                    config.debug = false;
                }
                rb
            },
            config,
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_menu_service: SysMenuService {},
            oss_service: OssService{},
            category_service: CategoryService{},
            article_service: ArticleService{},
            comment_service: CommentService{},
        }
    }
}

