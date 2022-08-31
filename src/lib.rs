use std::cmp::Ordering;

pub fn bin_search<T: Ord>(array: &[T], elem: &T) -> Result<usize, usize>
{
    let mut l = 0;
    let mut r = array.len();
    while l < r {
        let i = (l + r) / 2;
        match &array[i].cmp(elem) {
            Ordering::Less => l = i + 1,
            Ordering::Equal => return Ok(i),
            Ordering::Greater => r = i,
        }
    }
    Err(r)
}
