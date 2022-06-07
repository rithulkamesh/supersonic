use diesel::Queryable;

#[derive(Queryable)]
pub struct Config {
    pub property: String,
    pub val: String
}