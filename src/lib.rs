mod map;

pub use map::ArenaMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub struct Idx<T> {
    raw: RawIdx,
    _marker: PhantomData<fn() -> T>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Arena<T> {
    data: Vec<T>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawIdx(u32);

impl<T> Idx<T> {
    pub const DUMMY: Self = Idx {
        raw: RawIdx(0),
        _marker: PhantomData,
    };

    #[inline]
    pub fn from_raw(raw: RawIdx) -> Self {
        Idx {
            raw,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn into_raw(self) -> RawIdx {
        self.raw
    }
}

impl<T> Arena<T> {
    #[inline]
    pub const fn new() -> Self {
        Arena { data: Vec::new() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn next_idx(&self) -> Idx<T> {
        Idx::from_raw(RawIdx(self.data.len() as u32))
    }

    pub fn alloc(&mut self, value: T) -> Idx<T> {
        let id = self.next_idx();

        self.data.push(value);
        id
    }

    pub fn iter(&self) -> impl Iterator<Item = (Idx<T>, &T)> + ExactSizeIterator + DoubleEndedIterator {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, value)| (Idx::from_raw(RawIdx(idx as u32)), value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Idx<T>, &mut T)> + ExactSizeIterator + DoubleEndedIterator {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(idx, value)| (Idx::from_raw(RawIdx(idx as u32)), value))
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Arena { data: Vec::new() }
    }
}

impl<T> Index<Idx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, index: Idx<T>) -> &Self::Output {
        let idx = index.into_raw().0 as usize;

        &self.data[idx]
    }
}

impl<T> IndexMut<Idx<T>> for Arena<T> {
    fn index_mut(&mut self, index: Idx<T>) -> &mut Self::Output {
        let idx = index.into_raw().0 as usize;

        &mut self.data[idx]
    }
}

impl<T> FromIterator<T> for Arena<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Arena {
            data: Vec::from_iter(iter),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Arena<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Arena")
            .field("len", &self.len())
            .field("data", &self.data)
            .finish()
    }
}

impl<T> Clone for Idx<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Idx<T> {
}

impl<T> PartialEq for Idx<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for Idx<T> {
}

impl<T> Hash for Idx<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> fmt::Debug for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut type_name = std::any::type_name::<T>();

        if let Some(idx) = type_name.rfind(':') {
            type_name = &type_name[idx + 1..];
        }

        write!(f, "Idx<{}>({})", type_name, self.raw)
    }
}

impl From<RawIdx> for u32 {
    fn from(raw: RawIdx) -> Self {
        raw.0
    }
}

impl From<u32> for RawIdx {
    fn from(id: u32) -> Self {
        RawIdx(id)
    }
}

impl fmt::Debug for RawIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for RawIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
