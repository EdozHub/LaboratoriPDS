use std::collections::HashMap;

struct Tree{
    children: HashMap<String, Vec<String>>,
    father: HashMap<String, String>,
    switches: HashMap<String, bool>,
}

impl Tree{
    fn new() -> Self{
        Tree{
            children: HashMap::new(),
            father: HashMap::new(),
            switches: HashMap::new()
        }
    }
    fn add(mut self, father: &str, node: &str) {
        self.children
            .entry(father.to_string())
            .or_insert_with(Vec::new)
            .push(node.to_string());
        self.father
            .insert(node.to_string(), father.to_string());
        self.switches
            .insert(father.to_string(), true);
        self.switches
            .insert(node.to_string(), false);
    }

    fn remove(&self: Tree, node: &str)->Self {
        unimplemented!()
    }
}
