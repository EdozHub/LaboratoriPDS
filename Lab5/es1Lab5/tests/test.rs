#[cfg(test)]
pub mod test_list1 {
    use es1Lab5::list1::List;

    #[test]
    pub fn test_new(){
        let new_list = List::<i32>::new();
        assert_eq!(new_list.peek(), None);
    }

    #[test]
    pub fn test_push(){
        let mut new_list = List::<i32>::new();
        new_list.push(1);
        assert_eq!(new_list.peek(), Some(&1));
        new_list.push(2);
        assert_eq!(new_list.peek(), Some(&2));
        new_list.push(3);
        assert_eq!(new_list.peek(), Some(&3));
    }

    #[test]
    pub fn test_pop(){
        let mut new_list = List::<i32>::new();
        new_list.push(1);
        new_list.push(2);
        new_list.push(3);
        let dato = new_list.pop();
        assert_eq!(dato, Some(3));
        assert_eq!(new_list.peek(), Some(&2));
        let dato1 = new_list.pop();
        assert_eq!(dato1, Some(2));
        assert_eq!(new_list.peek(), Some(&1));
    }

    #[test]
    fn test_take_basic() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3); // lista: 3 → 2 → 1

        let mut taken = list.take(2); // deve prendere 3 → 2

        // Controlliamo che la lista originale sia rimasta con 1
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        // Controlliamo che `taken` abbia 3 → 2
        assert_eq!(taken.pop(), Some(3));
        assert_eq!(taken.pop(), Some(2));
        assert_eq!(taken.pop(), None);
    }

    #[test]
    fn test_list_iterator() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3); // La lista è: 3 → 2 → 1

        let collected: Vec<_> = list.iter().cloned().collect(); // da [&T] a T

        assert_eq!(collected, vec![3, 2, 1]);
    }
}

pub mod test_list2 {
    use es1Lab5::list2::List;

    #[test]
    pub fn test_new(){
        let list=List::<i32>::new;
    }

    #[test]
    pub fn test_push(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let dato = list.peek().unwrap();
        assert_eq!(*dato, 3);
    }
}