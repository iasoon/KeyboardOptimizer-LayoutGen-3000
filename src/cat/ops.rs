pub trait Map<S, T, R> {
    fn map<'s, F>(&'s self, fun: F) -> R
        where F: FnMut(&'s S) -> T,
              S: 's;
}
