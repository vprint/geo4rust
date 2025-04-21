/// Retrieves elements from the first vector that are present in the second.
///
/// # Arguments
///
/// * `vector_a` - Source vector from which to extract elements.
/// * `vector_b` - Comparison vector.
///
/// # Returns
///
/// A vector of references to elements from `vector_a` that are also present in `vector_b`.
///
/// # Example
///
/// ```
/// let fruits_a = vec!["🍎", "🍌", "🍊", "🍓"];
/// let fruits_b = vec!["🍌", "🥝", "🍓", "🍍"];
///
/// let present = get_present(&fruits_a, &fruits_b);
/// assert_eq!(present, vec![&"🍌", &"🍓"]);
/// ```
pub fn get_present<'a, T>(vector_a: &'a Vec<T>, vector_b: &'a Vec<T>) -> Vec<&'a T> where T: Eq {
    vector_a
        .iter()
        .filter(|item| vector_b.contains(item))
        .collect()
}

/// Retrieves elements from the first vector that are absent from the second.
///
/// # Arguments
///
/// * `vector_a` - Source vector from which to extract elements.
/// * `vector_b` - Comparison vector.
///
/// # Returns
///
/// A vector of references to elements from `vector_a` that are not present in `vector_b`.
///
/// # Example
///
/// ```
/// let fruits_a = vec!["🍎", "🍌", "🍊", "🍓"];
/// let fruits_b = vec!["🍌", "🥝", "🍓", "🍍"];
///
/// let absent = get_absent(&fruits_a, &fruits_b);
/// assert_eq!(absent, vec![&"🍎", &"🍊"]);
/// ```
pub fn get_absent<'a, T>(vector_a: &'a Vec<T>, vector_b: &'a Vec<T>) -> Vec<&'a T> where T: Eq {
    vector_a
        .iter()
        .filter(|item| !vector_b.contains(item))
        .collect()
}

pub fn descendant_sort<T: Ord>(vector: &mut Vec<T>) {
    vector.sort_by(|a, b| b.cmp(a));
}

pub fn ascendant_sort<T: Ord>(vector: &mut Vec<T>) {
    vector.sort();
}
