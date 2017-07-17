pub trait Map<S, T, R> {
    fn map<'s, F>(&'s self, fun: F) -> R
        where F: FnMut(&'s S) -> T,
              S: 's;
}

pub trait MapMut<T> {
    fn map_mut<'t, F>(&'t mut self, fun: F)
        where F: FnMut(&'t mut T),
              T: 't;
}

pub trait MapMutWithKey<K, T> {
    fn map_mut_with_key<'t, F>(&'t mut self, fun: F)
        where F: FnMut(K, &'t mut T),
              T: 't;
}

pub trait MapInto<S, T, R> {
    fn map_into<F>(self, fun: F) -> R
        where F: FnMut(S) -> T;
}
