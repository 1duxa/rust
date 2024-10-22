#[allow(dead_code)]
pub mod sublist{

    #[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if first_list.is_empty() {
        return Comparison::Sublist;
    }
    if second_list.is_empty() {
        return Comparison::Superlist;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(smaller: &[T], larger: &[T]) -> bool {
    smaller.is_empty() || larger.windows(smaller.len()).any(|window| window == smaller)
}
}