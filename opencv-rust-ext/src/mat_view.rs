use anyhow::Result;
use opencv::prelude::*;
use std::ops::*;

pub struct MatView<'a, T> {
    rows: usize,
    cols: usize,
    data: &'a mut [T],
}

impl<'a, T> MatView<'a, T> {
    pub fn new(rows: usize, cols: usize, data: &'a mut [T]) -> Self {
        assert_eq!(rows * cols, data.len());
        MatView { rows, cols, data }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<'a, T: DataType> MatView<'a, T> {
    pub fn from_mat(mat: &'a mut Mat) -> Result<Self> {
        let size = mat.size()?;
        Ok(Self::new(
            size.height as usize,
            size.width as usize,
            mat.data_typed_mut::<T>()?,
        ))
    }
}

impl<T> MatView<'_, T> {
    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T {
        self.data.get_unchecked(self.calc_index_unchecked(i, j))
    }

    pub unsafe fn get_unchecked_mut(&mut self, i: usize, j: usize) -> &mut T {
        let idx = self.calc_index_unchecked(i, j);
        self.data.get_unchecked_mut(idx)
    }
}

impl<T> MatView<'_, T> {
    fn calc_index(&self, i: usize, j: usize) -> usize {
        assert!(i < self.rows);
        assert!(j < self.cols);
        self.calc_index_unchecked(i, j)
    }

    fn calc_index_unchecked(&self, i: usize, j: usize) -> usize {
        i * self.cols + j
    }
}

impl<T> Index<(usize, usize)> for MatView<'_, T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[self.calc_index(i, j)]
    }
}

impl<T> IndexMut<(usize, usize)> for MatView<'_, T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let idx = self.calc_index(i, j);
        &mut self.data[idx]
    }
}
