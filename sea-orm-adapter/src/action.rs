use casbin::{error::AdapterError, Error as CasbinError, Filter, Result};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect,
};

use crate::entity::{self, ActiveModel, Column, Entity};

#[derive(Debug, Default)]
pub(crate) struct Rule<'a> {
    pub(crate) v0: &'a str,
    pub(crate) v1: &'a str,
    pub(crate) v2: &'a str,
    pub(crate) v3: &'a str,
    pub(crate) v4: &'a str,
    pub(crate) v5: &'a str,
}

impl<'a> Rule<'a> {
    pub(crate) fn from_str(value: &'a [&'a str]) -> Self {
        #[allow(clippy::get_first)]
        Rule {
            v0: value.get(0).map_or("", |x| x),
            v1: value.get(1).map_or("", |x| x),
            v2: value.get(2).map_or("", |x| x),
            v3: value.get(3).map_or("", |x| x),
            v4: value.get(4).map_or("", |x| x),
            v5: value.get(5).map_or("", |x| x),
        }
    }

    pub(crate) fn from_string(value: &'a [String]) -> Self {
        #[allow(clippy::get_first)]
        Rule {
            v0: value.get(0).map_or("", |x| x),
            v1: value.get(1).map_or("", |x| x),
            v2: value.get(2).map_or("", |x| x),
            v3: value.get(3).map_or("", |x| x),
            v4: value.get(4).map_or("", |x| x),
            v5: value.get(5).map_or("", |x| x),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct RuleWithType<'a> {
    pub(crate) ptype: &'a str,
    pub(crate) v0: &'a str,
    pub(crate) v1: &'a str,
    pub(crate) v2: &'a str,
    pub(crate) v3: &'a str,
    pub(crate) v4: &'a str,
    pub(crate) v5: &'a str,
}

impl<'a> RuleWithType<'a> {
    pub(crate) fn from_rule(ptype: &'a str, rule: Rule<'a>) -> Self {
        RuleWithType {
            ptype,
            v0: rule.v0,
            v1: rule.v1,
            v2: rule.v2,
            v3: rule.v3,
            v4: rule.v4,
            v5: rule.v5,
        }
    }
}

pub(crate) async fn remove_policy<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    ptype: &'rule str,
    rule: Rule<'rule>,
) -> Result<bool> {
    Entity::delete_many()
        .filter(Column::Ptype.eq(ptype))
        .filter(Column::V0.eq(rule.v0))
        .filter(Column::V1.eq(rule.v1))
        .filter(Column::V2.eq(rule.v2))
        .filter(Column::V3.eq(rule.v3))
        .filter(Column::V4.eq(rule.v4))
        .filter(Column::V5.eq(rule.v5))
        .exec(conn)
        .await
        .map(|count| count.rows_affected == 1)
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

pub(crate) async fn remove_policies<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    ptype: &'rule str,
    rules: Vec<Rule<'rule>>,
) -> Result<bool> {
    for rule in rules {
        remove_policy(conn, ptype, rule).await?;
    }
    Ok(true)
}

pub(crate) async fn remove_filtered_policy<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    ptype: &'rule str,
    index_of_match_start: usize,
    rule: Rule<'rule>,
) -> Result<bool> {
    let columns = [
        Column::V0,
        Column::V1,
        Column::V2,
        Column::V3,
        Column::V4,
        Column::V5,
    ];

    let values = [rule.v0, rule.v1, rule.v2, rule.v3, rule.v4, rule.v5];

    let base_condition = Condition::all().add(Column::Ptype.eq(ptype));

    let conditions = values
        .iter()
        .zip(&columns[index_of_match_start..])
        .filter(|(value, _)| !value.is_empty())
        .fold(base_condition, |acc, (value, column)| acc.add(column.eq(*value)));

    Entity::delete_many()
        .filter(conditions)
        .exec(conn)
        .await
        .map(|count| count.rows_affected >= 1)
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

pub(crate) async fn load_policy<C: ConnectionTrait>(conn: &C) -> Result<Vec<entity::Model>> {
    Entity::find()
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

pub(crate) async fn load_filtered_policy<'conn, 'filter, C: ConnectionTrait>(
    conn: &'conn C,
    filter: &'filter Filter<'filter>,
) -> Result<Vec<entity::Model>> {
    let g_filter = Rule::from_str(&filter.g);
    let p_filter = Rule::from_str(&filter.p);

    let g_condition = create_condition_from_rule("g", &g_filter);
    let p_condition = create_condition_from_rule("p", &p_filter);

    Entity::find()
        .filter(Condition::any().add(g_condition).add(p_condition))
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

fn create_condition_from_rule(prefix: &str, rule: &Rule) -> Condition {
    let mut condition = Condition::all().add(Column::Ptype.starts_with(prefix));

    let columns = [
        Column::V0,
        Column::V1,
        Column::V2,
        Column::V3,
        Column::V4,
        Column::V5,
    ];

    let values = [rule.v0, rule.v1, rule.v2, rule.v3, rule.v4, rule.v5];

    for (column, value) in columns.iter().zip(values.iter()) {
        if !value.is_empty() {
            condition = condition.add(column.eq(*value));
        }
    }

    condition
}

pub(crate) async fn save_policy<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rule: RuleWithType<'rule>,
) -> Result<i32> {
    let id: Option<i32> = Entity::find()
        .select_only()
        .column(Column::Id)
        .filter(Column::Ptype.eq(rule.ptype))
        .filter(Column::V0.eq(rule.v0))
        .filter(Column::V1.eq(rule.v1))
        .filter(Column::V2.eq(rule.v2))
        .filter(Column::V3.eq(rule.v3))
        .filter(Column::V4.eq(rule.v4))
        .filter(Column::V5.eq(rule.v5))
        .into_tuple()
        .one(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;

    if let Some(id) = id {
        return Ok(id);
    }

    let insert_result = Entity::insert(build_model(rule))
        .exec(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;

    Ok(insert_result.last_insert_id)
}

fn build_model(rule: RuleWithType) -> ActiveModel {
    ActiveModel {
        id: NotSet,
        ptype: Set(rule.ptype.to_string()),
        v0: Set(rule.v0.to_string()),
        v1: Set(rule.v1.to_string()),
        v2: Set(rule.v2.to_string()),
        v3: Set(rule.v3.to_string()),
        v4: Set(rule.v4.to_string()),
        v5: Set(rule.v5.to_string()),
    }
}

pub(crate) async fn save_policies<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rules: Vec<RuleWithType<'rule>>,
) -> Result<()> {
    let mut ids = Vec::with_capacity(rules.len());

    for rule in rules {
        ids.push(save_policy(conn, rule).await?);
    }

    Entity::delete_many()
        .filter(Condition::all().add_option((!ids.is_empty()).then(|| Column::Id.is_not_in(ids))))
        .exec(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;

    Ok(())
}

pub(crate) async fn add_policy<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rule: RuleWithType<'rule>,
) -> Result<()> {
    build_model(rule)
        .insert(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;

    Ok(())
}

pub(crate) async fn add_policies<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rules: Vec<RuleWithType<'rule>>,
) -> Result<()> {
    for rule in rules {
        add_policy(conn, rule).await?;
    }

    Ok(())
}

pub(crate) async fn clear_policy<C: ConnectionTrait>(conn: &C) -> Result<()> {
    Entity::delete_many()
        .exec(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;

    Ok(())
}
