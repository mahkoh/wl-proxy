use std::{
    array,
    cell::UnsafeCell,
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

#[cfg(test)]
mod tests;

type Seg = usize;
const SEG_SIZE: usize = Seg::BITS as usize;

pub(crate) struct FreeList<T, const N: usize> {
    levels: UnsafeCell<[Vec<Seg>; N]>,
    _phantom: PhantomData<T>,
}

impl<T, const N: usize> Default for FreeList<T, N> {
    fn default() -> Self {
        Self {
            levels: UnsafeCell::new(array::from_fn(|_| Vec::new())),
            _phantom: Default::default(),
        }
    }
}

impl<T, const N: usize> Debug for FreeList<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let levels = unsafe { &mut *self.levels.get() };
        f.debug_struct("FreeList").field("levels", levels).finish()
    }
}

impl<T, const N: usize> FreeList<T, N> {
    pub(crate) fn release(&self, n: T)
    where
        T: Into<u32>,
    {
        let mut ext = n.into() as usize;
        let mut int;
        let levels = unsafe { &mut *self.levels.get() };
        assert!(ext / SEG_SIZE < levels[0].len());
        for level in levels {
            int = ext % SEG_SIZE;
            ext /= SEG_SIZE;
            unsafe {
                *level.get_unchecked_mut(ext) |= 1 << int;
            }
        }
    }

    pub(crate) fn acquire(&self) -> T
    where
        u32: Into<T>,
    {
        let levels = unsafe { &mut *self.levels.get() };
        let mut ext = 'last: {
            let level = &mut levels[N - 1];
            for (idx, &seg) in level.iter().enumerate() {
                if seg != 0 {
                    break 'last idx;
                }
            }
            level.len()
        };
        for level in levels.iter_mut().rev() {
            if ext == level.len() {
                level.push(!0);
            }
            let seg = unsafe { level.get_unchecked(ext) };
            ext = SEG_SIZE * ext + seg.trailing_zeros() as usize;
        }
        let id = ext as u32;
        for level in levels.iter_mut() {
            let int = ext % SEG_SIZE;
            ext /= SEG_SIZE;
            let seg = unsafe { level.get_unchecked_mut(ext) };
            *seg &= !(1 << int);
            if *seg != 0 {
                break;
            }
        }
        id.into()
    }
}
