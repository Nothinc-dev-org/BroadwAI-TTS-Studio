use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dialogue_tts_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub dialogue_node_id: String,
    pub tag: String,
    pub position: String,
    pub order_index: i32,
    pub source: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dialogue_node::Entity",
        from = "Column::DialogueNodeId",
        to = "super::dialogue_node::Column::Id",
        on_delete = "Cascade"
    )]
    DialogueNode,
}

impl Related<super::dialogue_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DialogueNode.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
