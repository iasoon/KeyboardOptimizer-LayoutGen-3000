use std::collections::HashMap;

pub type Keymap = HashMap<String, KeyMapping>;

pub type KeyMapping = HashMap<String, String>;
