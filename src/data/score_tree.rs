pub type ScoreTree = Group;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub label: String,
    pub weight: f64,
    pub children: Children,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Children {
    Groups(Vec<Group>),
    Elems(Vec<Elem>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Elem {
    pub path: Vec<Loc>,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loc {
    pub key: String,
    pub layer: String,
}
