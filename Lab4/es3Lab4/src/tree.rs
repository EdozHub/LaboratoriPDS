use std::collections::HashMap;

struct Tree{
    children: HashMap<String, Vec<String>>,
    father: HashMap<String, String>,
    switches: HashMap<String, bool>,
}

impl Tree{
    fn new() -> Self{
        Tree{
            children: HashMap::from([("root".to_string(), Vec::new())]),
            father: HashMap::from([("root".to_string(), "root".to_string())]),
            switches: HashMap::from([("root".to_string(), false)]),
        }
    }
    fn add(&mut self, father: &str, node: &str) {
        self.children
            .entry(father.to_string().clone())
            .or_insert_with(Vec::new)
            .push(node.to_string());
        self.father
            .insert(node.to_string(), father.to_string());
        self.switches
            .insert(father.to_string(), true);
        self.switches
            .insert(node.to_string(), false);
    }

    fn find_relatives_recursive(nodes: Vec<String>, father: &str, found: &bool) -> Vec<String>{
        unimplemented!()
    }

    fn remove(&self: Tree, node: &str){
        let mut nodes: Vec<String>=Vec::new();
        nodes.push(node.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;
    fn build_tree() -> Tree{
        let mut tree = Tree::new();
        tree.add("root", "A");
        tree.add("A", "B");
        tree.add("A", "C");
        tree.add("B", "D");
        tree.add("B", "E");
        tree.add("C", "F");
        tree.add("C", "G");
        tree
    }
    #[test]
    fn test_build_tree() {
        let tree = build_tree();
        assert_eq!(tree.children.get("root").unwrap(), &vec!["A"]);
        assert_eq!(tree.children.get("A").unwrap(), &vec!["B", "C"]);
        assert_eq!(tree.children.get("B").unwrap(), &vec!["D", "E"]);
        assert_eq!(tree.children.get("C").unwrap(), &vec!["F", "G"]);
    }
}