use core::cmp::PartialOrd;
use std::{
    fmt::{Debug, Display},
    ops::Add,
    rc::Rc,
};

use crate::lib::list::List;

#[derive(PartialEq, Debug)]
pub enum ResultAppList<T> {
    Insecure(T),
    Secure,
    EmptyList,
    Missing,
}

pub struct AppList<T: Clone + Display> {
    pub sorted_list: List<T>,
    pub safe_list: List<T>,
    pub insecure_list: List<T>,
}

fn insert_before<U: PartialOrd>(item_to_insert: &U, current_item: &U) -> (bool, bool) {
    let is_before = true;
    if item_to_insert < current_item {
        return (true, is_before);
    }
    return (false, !is_before);
}

impl<T: PartialOrd + Clone + Display + Ord + Add + Copy + Debug> AppList<T>
where
    <T as Add>::Output: PartialOrd<T>,
{
    pub fn new() -> Self {
        Self {
            sorted_list: List::new(),
            safe_list: List::new(),
            insecure_list: List::new(),
        }
    }
    pub fn add_sorted_list(&mut self, mut list: Vec<T>) {
        list.sort();
        self.sorted_list = List::from(list);
    }

    pub fn add_safe_list(&mut self, list: Vec<T>) {
        self.safe_list = List::from(list);
    }

    pub fn add_insecure_list(&mut self, list: Vec<T>) {
        self.insecure_list = List::from(list);
    }

    pub fn run(&mut self) -> (u128, ResultAppList<T>) {
        let mut node_count = 0;
        if self.sorted_list.len() == 1 {
            return (0, ResultAppList::Missing);
        }
        if self.sorted_list.len() == 0 {
            return (0, ResultAppList::EmptyList);
        }
        while let Some(item) = self.insecure_list.pop_front() {
            let is_secure = self.has_two_sum(item);
            if !is_secure {
                return (node_count + 1, ResultAppList::Insecure(item));
            }
            // The number is secure
            self.safe_list.push_back(item);
            let first_node = self.safe_list.pop_front();

            if let Some(n) = first_node {
                // Insert sorted in sorted list
                self.sorted_list.insert(item, insert_before);
                //Delete the `pop_front` item
                self.sorted_list.delete_any(|node| node.borrow().item == n)
            } else {
                return (node_count, ResultAppList::EmptyList);
            }
            node_count += 1;
        }
        return (node_count, ResultAppList::Secure);
    }
    pub fn has_two_sum(&self, target: T) -> bool {
        if self.sorted_list.get_head().is_none()
            || self.sorted_list.get_head().unwrap().borrow().next.is_none()
        {
            return false;
        }

        let mut init_node = Rc::clone(self.sorted_list.get_head().unwrap());
        let mut last_node = Rc::clone(self.sorted_list.get_last().unwrap());
        let mut left = 0;
        let mut right = self.sorted_list.len() - 1;

        while left < right {
            let sum = init_node.borrow().item + last_node.borrow().item;

            if sum == target {
                return true;
            } else if sum < target {
                left += 1;
                let first_node = Rc::clone(&init_node);
                init_node = Rc::clone(first_node.borrow().next.as_ref().unwrap());
            } else {
                right -= 1;
                let last_node_copy = Rc::clone(&last_node);
                last_node = Rc::clone(last_node_copy.borrow().prev.as_ref().unwrap());
            }
        }
        false
    }
}

#[cfg(test)]
mod custom_list {
    use super::{AppList, ResultAppList};
    fn fill_app_list(numbers: Vec<u128>, safe_max: usize) -> AppList<u128> {
        let mut numbers_init: Vec<u128> =
            numbers[0..safe_max].iter().map(|n| n.to_owned()).collect();

        let mut app_list = AppList::new();
        app_list.add_safe_list(numbers_init.clone());
        numbers_init.sort(); // Sort the numbers to use `Two Pointers algorithm`
        app_list.add_sorted_list(numbers_init);
        app_list.add_insecure_list(numbers[safe_max..].iter().map(|t| t.to_owned()).collect());
        app_list
    }

    #[test]
    fn app_list_5() {
        let safe_max = 5;
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let mut app_list = fill_app_list(numbers, safe_max);

        let app_result = app_list.run();
        assert_eq!(app_result, (10, ResultAppList::Insecure(127)))
    }
    #[test]
    fn app_list_3() {
        let safe_max = 3;
        let numbers = vec![2, 7, 6, 9, 13, 19, 32, 46, 51, 78, 96];
        let mut app_list = fill_app_list(numbers, safe_max);

        let app_result = app_list.run();
        assert_eq!(app_result, (5, ResultAppList::Insecure(46)))
    }
    #[test]
    fn app_list_empty() {
        let mut app_list: AppList<u32> = AppList::new();

        let app_result = app_list.run();
        assert_eq!(app_result, (0, ResultAppList::EmptyList))
    }
    #[test]
    fn app_list_other_5() {
        let safe_max = 5;
        let numbers = vec![3, 7, 2, 5, 1, 4, 3, 7, 6, 4, 8, 9, 10, 10, 11, 20, 19, 31];
        let mut app_list = fill_app_list(numbers, safe_max);

        let app_result = app_list.run();
        assert_eq!(app_result, (10, ResultAppList::Insecure(11)))
    }
}
