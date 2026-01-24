use sea_orm::entity::prelude::*;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}

pub type User = Model;
