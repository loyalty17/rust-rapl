use std::cmp::max;
use std::ops;

use crate::errors::DimError;

pub(crate) fn multiply_list<T>(list: &[T], init: T) -> T
where
    T: ops::MulAssign + Copy,
{
    let mut result: T = init;
    for x in list {
        result *= *x
    }
    result
}

// Given and index n of a flat array, calculate the indexes of such element in an N rank array of shape: shape
pub(crate) fn get_indexes<const N: usize>(n: &usize, shape: &[usize; N]) -> [usize; N] {
    let mut ind: [usize; N] = [0; N];
    for i in (0..N).rev() {
        let n_block = multiply_list(&shape[i + 1..], 1 as usize);
        ind[i] = ((n - (n % n_block)) / n_block) % shape[i]
    }
    ind
}



// given the indexes of an element in a N rank array of shape shape return the position of such element if the array was flattened
pub(crate) fn get_flat_pos<const R: usize>(
    indexes: &[usize; R],
    shape: &[usize; R],
) -> Result<usize, String> {
    let mut ind: usize = 0;
    for i in 0..R {
        if indexes[i] >= shape[i] {
            return Err("Error: Index out of bounds".into());
        }
        ind += indexes[R - i - 1] * multiply_list(&shape[R - i..], 1);
    }
    Ok(ind)
}



pub(crate) fn format_vla(val: String, size: &usize) -> String {
    let mut s = val.clone();
    let l = val.len();
    s += ",";
    s += &" ".repeat(size - l);
    s
}

pub fn remove_element<T: Copy, const N: usize>(arr: [T; N], index: usize) -> [T; N - 1] {
    assert!(index < N);
    let mut result = [arr[0]; N - 1];
    let mut j = 0;
        for (i, &item) in arr.iter().enumerate().take(N) {
        if i != index {
            result[j] = item;
            j += 1;
        }
    }
    result
}


pub fn insert_element<T: Copy + Default, const N: usize>(arr: [T; N], index: usize, element: T) -> [T; N + 1] {
    let mut result = [T::default(); N + 1];
    let mut inserted = false;
    for i in 0..result.len(){
        if ! inserted && i == index{
            result[i] = element;
            inserted = true
        }else if ! inserted{
            result[i] = arr[i]
        }else{
            result[i] = arr[i-1]
        }
    }
    return  result;
}

// paths a shape of rank N with ones in the left until is rank M
pub fn path_shape<const N: usize, const M: usize>(
    shape: &[usize; N],
) -> Result<[usize; M], DimError> {
    if N > M {
        return Err(DimError::new(&format!(
            "Can not path shape {:?} of rank {} to rank {}.",
            shape, N, M
        )));
    }
    let mut out: [usize; M] = [1; M];
    for i in 0..N {
        out[M - i - 1] = shape[N - i - 1]
    }
    Ok(out)
}

// Given a indexes of an element in broadcasted broadcasted array and a shape of the array before broadcasting, 
// returns the position of the element in the array before broadcasting
pub fn rev_cast_pos<const N: usize, const M: usize>(
    small_shape: &[usize; N],
    indexes: &[usize; M],
) -> Result<usize, DimError> {
    let padded: [usize; M] = path_shape(small_shape)?;
    let mut indexes = *indexes;
    let mut rev_casted_ind: [usize; N] = [0; N];
    for i in 0..M {
        if padded[i] <= indexes[i] {
            indexes[i] = padded[i] - 1
        }
    }
    for i in 0..N {
        rev_casted_ind[N - i - 1] = indexes[M - i - 1]
    }
    let pos = get_flat_pos(&rev_casted_ind, small_shape).unwrap();
    Ok(pos)
}

fn index_or(arr: &[usize], index: usize, or: usize) -> usize {
    if index >= arr.len() {
        or
    } else {
        arr[index]
    }
}

pub const fn const_max(x: usize, y: usize) -> usize {
    if x >= y {
        x
    } else {
        y
    }
}

pub fn broadcast_shape<const N: usize, const M: usize>(
    shape1: &[usize; N],
    shape2: &[usize; M],
) -> Result<[usize;  const_max(N, M) ], DimError> {
    let mut out_shape: [usize;  const_max(N, M) ] = [0; { const_max(N, M) }];
    let mut sh1 = shape1.to_vec();
    let mut sh2 = shape2.to_vec();
    sh1.reverse();
    sh2.reverse();

    let l = max(N, M);
    for i in 0..l {
        let size1 = index_or(&sh1, i, 1);
        let size2 = index_or(&sh2, i, 1);
        if size1 != 1 && size2 != 1 && size1 != size2 {
            return Err(DimError::new(&format!(
                "Error arrays with shape {:?} and {:?} can not be broadcasted.",
                shape1, shape2
            )));
        }
        out_shape[l - i - 1] = max(size1, size2)
    }
    Ok(out_shape)
}
