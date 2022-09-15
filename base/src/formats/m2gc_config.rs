// Standard USes

// Crate Uses

// External Uses


pub enum M2Script<'a> {
    Object(Vec<(&'a str, M2Script<'a>)>),
    Group(Vec<(&'a str, M2Script<'a>)>),
    List(Vec<M2Script<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
}

