use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Error{
    Duplicate,
    NotFound,
    Forbidden,
}

struct Tree{
    children: HashMap<String, Vec<String>>,
    father: HashMap<String, String>,
    is_node: HashMap<String, bool>,
    switches: HashMap<String, bool>,
}

impl Tree{
    fn new() -> Self{
        Tree{
            children: HashMap::from([("root".to_string(), Vec::new())]),
            father: HashMap::from([("root".to_string(), "root".to_string())]),
            is_node: HashMap::from([("root".to_string(), false)]),
            switches: HashMap::from([("root".to_string(), false)]),
        }
    }
    fn add(&mut self, father: &str, node: &str){
        self.children
            .entry(father.to_string().clone())
            .or_insert_with(Vec::new)
            .push(node.to_string());
        self.father
            .insert(node.to_string(), father.to_string());
        self.is_node
            .insert(father.to_string(), true);
        self.is_node
            .insert(node.to_string(), false);
        self.switches.insert(node.to_string(), false);
    }

    fn remove(self: &mut Tree, node: &str) -> Result<(), Error> {
        if node == "root" {
            return Err(Error::Forbidden);
        }
        let children = self.children.get(node).cloned().unwrap_or_default(); // Copia i figli
        let father = self.father.get(node).cloned(); // Copia il padre
        for child in children {
            let res=self.remove(&child)?; // Rimuove ricorsivamente
        }
        if let Some(father) = father.as_ref() {
            if let Some(father_children) = self.children.get_mut(father) {
                father_children.retain(|child| child != node);
            }
            if self.children.get(father).map_or(true, |v| v.is_empty()) {
                if let Some(value) = self.is_node.get_mut(father) {
                    *value = false;
                }
            }
        }
        self.father.remove(node);
        self.is_node.remove(node);
        Ok(())
    }

    fn toggle(&mut self, node: &str) -> bool{
        self.switches.get_mut(node).map(|v| *v = !*v);
        self.switches.get(node).unwrap().clone()
    }

    fn peek(&self, node: &str) -> bool{
        self.switches.get(node).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;
    fn build_tree() -> Tree{
        let mut tree = Tree::new();
        tree.add("root", "A");
        tree.add("root", "B");
        tree.add("A", "C");
        tree.add("A", "D");
        tree.add("A", "E");
        tree.add("B", "F");
        tree.add("B", "G");
        tree.add("C", "H");
        tree.add("C", "I");
        tree.add("D", "J");
        tree.add("F", "K");
        tree.add("F", "L");
        tree.add("F", "M");
        tree.add("G", "N");
        tree.add("G", "O");
        tree
    }
    #[test]
    fn test_build_tree() {
        let tree = build_tree();
        assert_eq!(tree.children.get("root").unwrap(), &vec!["A", "B"]);
        assert_eq!(tree.children.get("A").unwrap(), &vec!["C", "D", "E"]);
        assert_eq!(tree.children.get("B").unwrap(), &vec!["F", "G"]);
        assert_eq!(tree.children.get("C").unwrap(), &vec!["H", "I"]);
        assert_eq!(tree.children.get("D").unwrap(), &vec!["J"]);
        assert_eq!(tree.children.get("F").unwrap(), &vec!["K", "L", "M"]);
        assert_eq!(tree.children.get("G").unwrap(), &vec!["N", "O"]);

        assert_eq!(tree.father.get("root").unwrap().to_string(), "root".to_string());
        assert_eq!(tree.father.get("A").unwrap().to_string(), "root".to_string());
        assert_eq!(tree.father.get("B").unwrap().to_string(), "root".to_string());
        assert_eq!(tree.father.get("C").unwrap().to_string(), "A".to_string());
        assert_eq!(tree.father.get("D").unwrap().to_string(), "A".to_string());
        assert_eq!(tree.father.get("E").unwrap().to_string(), "A".to_string());
        assert_eq!(tree.father.get("F").unwrap().to_string(), "B".to_string());
        assert_eq!(tree.father.get("G").unwrap().to_string(), "B".to_string());
        assert_eq!(tree.father.get("H").unwrap().to_string(), "C".to_string());
        assert_eq!(tree.father.get("I").unwrap().to_string(), "C".to_string());
        assert_eq!(tree.father.get("J").unwrap().to_string(), "D".to_string());
        assert_eq!(tree.father.get("K").unwrap().to_string(), "F".to_string());
        assert_eq!(tree.father.get("L").unwrap().to_string(), "F".to_string());
        assert_eq!(tree.father.get("M").unwrap().to_string(), "F".to_string());
        assert_eq!(tree.father.get("N").unwrap().to_string(), "G".to_string());
        assert_eq!(tree.father.get("O").unwrap().to_string(), "G".to_string());

        assert_eq!(tree.is_node.get("root").unwrap(), &true);
        assert_eq!(tree.is_node.get("A").unwrap(), &true);
        assert_eq!(tree.is_node.get("B").unwrap(), &true);
        assert_eq!(tree.is_node.get("C").unwrap(), &true);
        assert_eq!(tree.is_node.get("D").unwrap(), &true);
        assert_eq!(tree.is_node.get("E").unwrap(), &false);
        assert_eq!(tree.is_node.get("F").unwrap(), &true);
        assert_eq!(tree.is_node.get("G").unwrap(), &true);
        assert_eq!(tree.is_node.get("H").unwrap(), &false);
        assert_eq!(tree.is_node.get("I").unwrap(), &false);
        assert_eq!(tree.is_node.get("J").unwrap(), &false);
        assert_eq!(tree.is_node.get("K").unwrap(), &false);
        assert_eq!(tree.is_node.get("L").unwrap(), &false);
        assert_eq!(tree.is_node.get("M").unwrap(), &false);
        assert_eq!(tree.is_node.get("N").unwrap(), &false);
        assert_eq!(tree.is_node.get("O").unwrap(), &false);
    }

    #[test]
    fn test_remove(){
        let mut tree = build_tree();
        let res = tree.remove("C");
        assert_eq!(res, Ok(()));
        assert_eq!(tree.children.get("A").unwrap(), &vec!["D", "E"]);
        assert_eq!(tree.is_node.get("A").unwrap(), &true);
        let res = tree.remove("J");
        assert_eq!(tree.is_node.get("D").unwrap(), &false);
    }

    #[test]
    fn test_toggle_peek(){
        let mut tree = build_tree();
        let res = tree.toggle("A");
        assert_eq!(tree.switches.get("B").unwrap(), &false);
        assert_eq!(tree.switches.get("A").unwrap(), &true);
        assert_eq!(res, true);
        let res1 = tree.toggle("A");
        assert_eq!(tree.switches.get("A").unwrap(), &false);
        assert_eq!(res1, false);

        let res2 = tree.toggle("B");
        assert_eq!(tree.peek("B"), true);
        let res3 = tree.toggle("B");
        assert_eq!(tree.peek("B"), false);
    }
}