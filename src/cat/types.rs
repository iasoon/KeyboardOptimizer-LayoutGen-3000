use std::marker::PhantomData;
use cat::composed::Composed;
use cat::table::Table;

pub trait Domain {
    type Type;
}

pub trait FiniteDomain: Domain {}

pub trait Mapping<T>
{
    type Result;

    fn apply(&self, elem: T) -> Self::Result;
}

/// A mapping that stores its values
/// A Dict is total; it has a value for each member of its domain.
pub trait Dict<K, T: ?Sized> {
    fn get<'t>(&'t self, elem: K) -> &'t T;
    fn get_mut<'t>(&'t mut self, elem: K) -> &'t mut T;

    fn compose<M>(self, mapping: M) -> Composed<M, Self>
        where Self: Sized
    {
        Composed::new(mapping, self)
    }

    fn into_mapping(self) -> DictMapping<Self, T>
        where Self: Sized,
              T: Sized + Copy
    {
        DictMapping {
            phantom_t: PhantomData,
            dict: self,
        }
    }

    fn borrow_mapping<'t>(&'t self) -> BorrowedDict<'t, Self, T>
        where Self: Sized,
              T: Sized
    {
        BorrowedDict {
            phantom_t: PhantomData,
            dict: self,
        }
    }
}

pub struct DictMapping<D, T> {
    phantom_t: PhantomData<T>,
    dict: D,
}

impl<D, K, T> Mapping<K> for DictMapping<D, T>
    where D: Dict<K, T>,
          T: Sized + Copy
{
    type Result = T;

    fn apply(&self, key: K) -> T {
        *self.dict.get(key)
    }
}

pub type TableMapping<D, T> = DictMapping<Table<D, T>, T>;

pub struct BorrowedDict<'t, D: 't, T> {
    dict: &'t D,
    phantom_t: PhantomData<T>,
}

impl<'t, D, K, T: 't> Mapping<K> for BorrowedDict<'t, D, T>
    where D: Dict<K, T>
{
    type Result = &'t T;

    fn apply(&self, key: K) -> &'t T {
        self.dict.get(key)
    }
}

pub type BorrowedTable<'t, D, T> = BorrowedDict<'t, Table<D, T>, T>;
