//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_role_menu")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub role_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub menu_id: i32,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub domain: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_menu::Entity",
        from = "Column::MenuId",
        to = "super::sys_menu::Column::Id"
    )]
    SysMenu,
    #[sea_orm(
        belongs_to = "super::sys_role::Entity",
        from = "Column::RoleId",
        to = "super::sys_role::Column::Id"
    )]
    SysRole,
}

impl Related<super::sys_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysMenu.def()
    }
}

impl Related<super::sys_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
