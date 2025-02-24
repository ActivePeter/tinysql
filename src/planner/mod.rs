use std::sync::Arc;

use crate::{
    binder::{statement::BoundStatement, table_ref::BoundTableRef},
    catalog::schema::{self, Schema},
};

use self::{logical_plan::LogicalPlan, operator::LogicalOperator};

pub mod logical_plan;
pub mod operator;
pub mod plan_insert;
pub mod plan_select;

pub struct Planner {}
impl Planner {
    // 根据BoundStatement生成逻辑计划
    pub fn plan(&mut self, statement: BoundStatement) -> LogicalPlan {
        match statement {
            BoundStatement::Insert(stmt) => self.plan_insert(stmt),
            BoundStatement::CreateTable(stmt) => {
                let schema = Schema::new(stmt.columns);
                LogicalPlan {
                    operator: LogicalOperator::new_create_table_operator(stmt.table_name, schema),
                    children: Vec::new(),
                }
            }
            BoundStatement::Select(stmt) => self.plan_select(stmt),
            _ => unimplemented!(),
        }
    }

    fn plan_table_ref(&mut self, table_ref: BoundTableRef) -> LogicalPlan {
        match table_ref {
            BoundTableRef::BaseTable(table) => LogicalPlan {
                operator: LogicalOperator::new_scan_operator(table.oid, table.schema.columns),
                children: Vec::new(),
            },
            BoundTableRef::Join(join) => {
                let left_plan = self.plan_table_ref(*join.left);
                let right_plan = self.plan_table_ref(*join.right);
                let join_plan = LogicalPlan {
                    operator: LogicalOperator::new_join_operator(join.join_type, join.condition),
                    children: vec![Arc::new(left_plan), Arc::new(right_plan)],
                };
                join_plan
            }
            _ => unimplemented!(),
        }
    }
}
