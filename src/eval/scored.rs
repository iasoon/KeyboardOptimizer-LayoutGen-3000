#[derive(Clone)]
pub struct Scored<T> {
    pub value: T,
    pub score: f64,
}

impl<T> Scored<T> {
    pub fn map<R, F>(self, fun: F) -> Scored<R>
        where F: FnOnce(T) -> R
    {
        Scored {
            value: fun(self.value),
            score: self.score,
        }
    }
}
