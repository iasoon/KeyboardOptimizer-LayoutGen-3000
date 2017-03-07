use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct KbConf {
    pub tokens: Vec<String>,
    pub keys: Vec<String>,
    pub layers: Vec<String>,
}
