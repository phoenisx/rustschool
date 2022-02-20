#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// If I work with two pointers to track current location in array, it might work:
/// Since the sequence needs to be in order, the solution can be kept simple.
/// Following works if I don't have duplicates.
/// For eg. [1,2,3], [1,1,2,3], even though we have a subset, the loop will break at index 1
///     We can fix this, if I do not break and compare second array pointer with
///     first arrays' previous pointer.
///
/// counter -> 0, counter should be >0 for Comparison to be either sublist, superlist or equal
/// i, j are two pointers starting from 0, 0 for first_list and second_list respectively
///
/// loop till i < fl.len && j < sl.len
///     if (fl[i] = sl[j])
///         i +=1, j+=1, counter += 1
///     else
///         if counter = 0 && fl.len > sl.len
///             i += 1
///         else if (counter = 0 && fl.len < sl.len)
///             j += 1
///         else if (counter = 0)
///             i += 1, j += 1
///         else
///             if (fl.len > sl.len && fl[i] = sl[j-1])
///                 j -= 1, counter -> 0
///             else if (fl.len < sl.len && fl[i] = sl[j-1])
///                 i -= 1, counter -> 0
///             else
///                 // We got a comparison before, loop hasn't ended, that means there's no common seq
///                 break
///
/// if (counter = fl.len && counter = sl.len)
///     // equal
/// else if (counter = fl.len)
///     // subset
/// else if (counter = sl.len)
///     // superset
/// else
///     // unequal
///
/// Above impl fails for duplicates
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut counter = 0;
    let mut i = 0;
    let mut j = 0;

    while i < _first_list.len() && j < _second_list.len() {
        if _first_list[i] == _second_list[j] {
            i += 1;
            j += 1;
            counter += 1;
        } else {
            if counter == 0 && _first_list.len() > _second_list.len() {
                i += 1;
            } else if counter == 0 && _first_list.len() < _second_list.len() {
                j += 1
            } else if counter == 0 {
                i += 1;
                j += 1;
            } else {
                if _first_list.len() > _second_list.len() && _first_list[i] == _second_list[j - 1] {
                    j -= 1;
                    counter = 0;
                } else if _first_list.len() < _second_list.len() && _first_list[i] == _second_list[j - 1] {
                    i -= 1;
                    counter = 0;
                } else {
                    // We got a comparison before, loop hasn't ended, that means there's no common seq
                    break;
                }
            }
        }
    }

    if counter == _first_list.len() && counter == _second_list.len() {
        return Comparison::Equal;
    } else if counter == _first_list.len() {
        return Comparison::Sublist;
    } else if counter == _second_list.len() {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}
