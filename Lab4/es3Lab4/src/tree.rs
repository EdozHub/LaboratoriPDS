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
    fn add_child(mut self: Tree, father: String)->Self {
        unimplemented!()
    }
    fn remove_child(mut self: Tree, child: String)->Self {
        unimplemented!()
    }
}
