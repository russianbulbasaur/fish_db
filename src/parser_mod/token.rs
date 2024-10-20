#[allow(unused)]
pub enum Reserved{
    SELECT,
    FROM,
    COUNT
}

#[allow(unused)]
pub fn is_reserved(token:&String) -> Option<Reserved>{
    match token.as_str() {
        "SELECT" => Some(Reserved::SELECT),
        "FROM" => Some(Reserved::FROM),
        "COUNT" => Some(Reserved::COUNT),
        _ => None
    }
}
