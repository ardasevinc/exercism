use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let list_lens = (first_list.len(), second_list.len());

    match list_lens.0.cmp(&list_lens.1) {
        Ordering::Equal => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        // Check if first list is a sublist of the second
        Ordering::Less => {
            if let 0 = list_lens.0 {
                return Comparison::Sublist;
            }

            let mut second_list_window = second_list.windows(list_lens.0);

            let is_sublist = second_list_window.any(|sublist| sublist == first_list);

            // Why does `is_sublist || list_lens.0 == 0` doesn't work?
            if is_sublist {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }

        Ordering::Greater => {
            if let 0 = list_lens.1 {
                return Comparison::Superlist;
            }

            let mut first_list_window = first_list.windows(list_lens.1);

            let is_superlist = first_list_window.any(|sublist| sublist == second_list);

            if is_superlist {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}
