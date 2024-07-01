use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult)]
pub struct UserWithDomainAndOrgOutput {
    pub id: i64,
    pub domain_id: i64,
    pub org_id: Option<i64>,
    pub username: String,
    pub password: String,
    pub nick_name: String,
    pub avatar: Option<String>,
    pub domain_code: String,
    pub domain_name: String,
    pub organization_id: Option<i64>,
    pub organization_name: Option<String>,
}
