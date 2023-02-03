#[cfg(test)]
mod list_test {
    use crate::lib::list::List;

    #[test]
    fn list_empty() {
        let list: List<i32> = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn list_len_3() {
        let mut list: List<i32> = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert!(!list.is_empty());
        assert!(list.len() == 3);
    }

    #[test]
    fn list_from_vec() {
        let list: List<i32> = List::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert!(!list.is_empty());
        assert!(list.len() == 9);
    }
    #[test]
    fn list_display_items() {
        let list: List<i32> = List::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            list.to_string(),
            "(1) <-> (2) <-> (3) <-> (4) <-> (5) <-> (6) <-> (7) <-> (8) <-> (9)"
        );
    }
    #[test]
    fn empty() {
        let list: List<i32> = List::from(vec![]);
        assert_eq!(list.to_string(), "()");
    }

    #[test]
    fn list_push_back_front() {
        let mut list: List<i32> = List::from(vec![1, 2, 3]);
        list.push_front(-2);
        list.push_front(-3);
        list.push_back(10);
        list.push_back(100);
        assert_eq!(
            list.to_string(),
            "(-3) <-> (-2) <-> (1) <-> (2) <-> (3) <-> (10) <-> (100)"
        );
    }

    #[test]
    fn list_insert_and_push() {
        let mut list: List<i32> = List::from(vec![1, 2, 3]);

        assert_eq!(list.len(), 3);

        list.push_front(-10);
        list.push_back(10);

        assert_eq!(list.len(), 5);

        fn insert(item: &i32, current_item: &i32) -> (bool, bool) {
            let is_before = true;
            if item < current_item {
                return (true, is_before);
            }
            return (false, false);
        }
        list.insert(0, insert);
        list.insert(5, insert);
        list.insert(11, insert);
        list.insert(-12, insert);

        assert_eq!(list.len(), 9);
        assert_eq!(
            list.to_string(),
            "(-12) <-> (-10) <-> (0) <-> (1) <-> (2) <-> (3) <-> (5) <-> (10) <-> (11)"
        );

        list.pop_front();
        list.pop_back();
        assert_eq!(list.len(), 7);
        assert_eq!(
            list.to_string(),
            "(-10) <-> (0) <-> (1) <-> (2) <-> (3) <-> (5) <-> (10)"
        );
    }

    #[test]
    fn list_push_and_pop() {
        let mut list: List<i32> = List::from(vec![]);
        assert!(list.is_empty());
        list.push_front(-2);
        list.push_front(-3);
        list.push_back(10);
        list.push_back(100);
        assert!(list.len() == 4);
        assert_eq!(list.to_string(), "(-3) <-> (-2) <-> (10) <-> (100)");
        list.pop_front();
        list.pop_front();
        assert_eq!(list.to_string(), "(10) <-> (100)");
        list.pop_back();
        assert_eq!(list.to_string(), "(10)");
        list.pop_back();
        assert_eq!(list.to_string(), "()");
        assert!(list.is_empty())
    }
    #[test]
    fn list_delete_any() {
        let mut list: List<i32> = List::from(vec![]);
        assert!(list.is_empty());
        list.push_front(-2);
        list.push_back(10);
        list.push_back(100);
        assert!(list.len() == 3);
        assert_eq!(list.to_string(), "(-2) <-> (10) <-> (100)");
        list.delete_any(|node| node.borrow().item == 100);
        assert!(list.len() == 2);
        assert_eq!(list.to_string(), "(-2) <-> (10)");
        list.delete_any(|node| node.borrow().item == 10);
        list.pop_back();
        assert!(list.len() == 0);
        list.push_front(1000);
        list.delete_any(|node| node.borrow().item == 1000);
        assert!(list.is_empty());
    }
}
