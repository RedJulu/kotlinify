/// Executes an action a specified number of times.
///
/// The action receives the zero-based iteration index.
///
/// # Examples
///
/// ```
/// use kotlinify::repeat;
///
/// let mut indices = Vec::new();
///
/// repeat(3, |index| indices.push(index));
///
/// assert_eq!(indices, vec![0, 1, 2]);
/// ```
pub fn repeat(times: usize, mut action: impl FnMut(usize)) {
    for index in 0..times {
        action(index);
    }
}
