use super::schema::chatters;

#[derive(Queryable, Clone, Debug)]
pub struct Chatter {
    pub id: i64,
    pub name: String,
    pub type_: i32,
    pub avatar_key: String,
    pub update_time: i64,
    pub name_pinyin: String,
    pub creator_id: Option<i64>,
    pub is_resigned: Option<bool>
}

#[derive(Insertable, Debug)]
#[table_name = "chatters"]
pub struct NewChatter<'a> {
    pub id: i64,
    pub name: &'a str,
    pub type_: i32,
    pub avatar_key: &'a str,
    pub update_time: i64,
    pub name_pinyin: &'a str,
    pub creator_id: Option<i64>,
    pub is_resigned: Option<bool>
}
