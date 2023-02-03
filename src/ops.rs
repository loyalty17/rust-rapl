use super::*;
use std::ops::*;

impl<T, const R: usize, const N: usize> Add for Ndarr<T, N, R>
where
    T: Add<Output = T> + Copy + Clone + Debug + Default,
    [T; N]: Default,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        //this is temporary, util we att projection por rank polymorphic operations
        if self.shape != other.shape {
            panic!("Shape missmatch")
        } else {
            self.bimap(other, |x, y| *x + *y)
        }
    }
}

impl<T, const R: usize, const N: usize> Sub for Ndarr<T, N, R>
where
    T: Sub<Output = T> + Copy + Clone + Debug + Default,
    [T; N]: Default,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        //this is temporary, util we att projection por rank polymorphic operations
        if self.shape != other.shape {
            panic!("Shape missmatch")
        } else {
            self.bimap(other, |x, y| *x - *y)
        }
    }
}

impl<T, const R: usize, const N: usize> Mul for Ndarr<T, N, R>
where
    T: Mul<Output = T> + Copy + Clone + Debug + Default,
    [T; N]: Default,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        //this is temporary, util we att projection por rank polymorphic operations
        if self.shape != other.shape {
            panic!("Shape missmatch")
        } else {
            self.bimap(other, |x, y| *x * *y)
        }
    }
}

impl<T, const R: usize, const N: usize> Rem for Ndarr<T, N, R>
where
    T: Rem<Output = T> + Copy + Clone + Debug + Default,
    [T; N]: Default,
{
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        //this is temporary, util we att projection por rank polymorphic operations
        if self.shape != other.shape {
            panic!("Shape missmatch")
        } else {
            self.bimap(other, |x, y| *x % *y)
        }
    }
}

impl<T, const R: usize, const N: usize> Neg for Ndarr<T, N, R>
where
    T: Neg<Output = T> + Copy + Clone + Debug + Default,
    [T; N]: Default,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.map(|x| -*x)
    }

}