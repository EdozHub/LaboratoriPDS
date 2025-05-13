use std::collections::HashMap;

pub enum Error{
    Duplicate,
    NotFound,
    Forbidden,
}

struct Tree{
    children: HashMap<String, Vec<String>>,
    father: HashMap<String, String>,
    switches: HashMap<String, bool>,
}

impl Clone for Tree{
    fn clone(&self) -> Tree{
        return self.clone();
    }
}

impl Tree{
    fn new() -> Self{
        Tree{
            children: HashMap::from([("root".to_string(), Vec::new())]),
            father: HashMap::from([("root".to_string(), "root".to_string())]),
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
        self.switches
            .insert(father.to_string(), true);
        self.switches
            .insert(node.to_string(), false);
    }

    fn remove(&mut self: Tree, node: &str)->Result<(),Error>{
        if node == "root" {
            return Err(Error::Forbidden);
        }

        if !self.switches.contains_key(node) {
            return Err(Error::NotFound);
        }
        let children = match self.children.get(node){
            Some(children)=> children.clone(),
            None => vec![]
        };
        for child in children{
           self.clone().remove(node);
        }
        Ok(())
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

        assert_eq!(tree.switches.get("root").unwrap(), &true);
        assert_eq!(tree.switches.get("A").unwrap(), &true);
        assert_eq!(tree.switches.get("B").unwrap(), &true);
        assert_eq!(tree.switches.get("C").unwrap(), &true);
        assert_eq!(tree.switches.get("D").unwrap(), &true);
        assert_eq!(tree.switches.get("E").unwrap(), &false);
        assert_eq!(tree.switches.get("F").unwrap(), &true);
        assert_eq!(tree.switches.get("G").unwrap(), &true);
        assert_eq!(tree.switches.get("H").unwrap(), &false);
        assert_eq!(tree.switches.get("I").unwrap(), &false);
        assert_eq!(tree.switches.get("J").unwrap(), &false);
        assert_eq!(tree.switches.get("K").unwrap(), &false);
        assert_eq!(tree.switches.get("L").unwrap(), &false);
        assert_eq!(tree.switches.get("M").unwrap(), &false);
        assert_eq!(tree.switches.get("N").unwrap(), &false);
        assert_eq!(tree.switches.get("O").unwrap(), &false);
    }
}