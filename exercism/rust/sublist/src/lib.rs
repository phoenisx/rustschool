#![allow(unused_doc_comments)]

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/**
 * List can have duplicates
 */
pub fn sublist<T: PartialEq + Clone>(fl: &[T], sl: &[T]) -> Comparison {
    /**
     * - find which list is smaller
     *  - if both are of equal length, take any for the loop
     * - find if every element from smaller list exist in sequence to the second one
     */

    if fl.len() < sl.len() {
        if is_subset(fl, sl) {
            return Comparison::Sublist;
        }
    } else {
        if is_subset(fl, sl) {
            if fl.len() == sl.len() {
                return Comparison::Equal;
            }
            return Comparison::Sublist;
        }
    }

    return Comparison::Unequal;
}

fn is_subset<T: PartialEq + Clone> (list1: &[T], list2: &[T]) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut counter = 0;
    let l1 = remove_duplicates(list1);
    let l2  = remove_duplicates(list2);

    while i < l1.len() && j < l2.len() {
        if l1[i] == l2[j] {
            counter += 1;
        } else {
            // Restart
            i = 0;
            counter = 0;
            continue;
        }

        i += 1;
        j += 1;
    }

    return counter == l1.len();
}

fn remove_duplicates<T: PartialEq + Clone> (list: &[T]) -> Vec<T> {
    let mut i = 0;
    let mut without_dups: Vec<T> = Vec::with_capacity(list.len());

    // Best and simplest way is to reduce all consecutive duplicates
    // and reduce the size before comparison happens, from both the lists
    while i < list.len() {
        // Since I just need to compare later, even if a reference
        // is copied, that's fine here
        if i == 0 {
            // The only issue with this code is that, I need
            // to clone the entire Item for push to work.
            // Not sure how much this affects the performance
            without_dups.push(list[i].clone());
        } else {
            if list[i-1] != list[i] {
                without_dups.push(list[i].clone());
            }
        }
        i += 1;
    }

    return without_dups;
}
