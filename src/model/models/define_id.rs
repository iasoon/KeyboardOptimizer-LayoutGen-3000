macro_rules! define_id {
    ($model: ident, $id: ident) => {
        use utils::{HasId, Countable, ElemCount};

        impl HasId for $model {
            type Id = $id;
        }

        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $id(usize);

        impl Countable for $id {
            type Data = ElemCount<$model>;

            fn from_num(_: &Self::Data, num: usize) -> Self {
                $id(num)
            }

            fn to_num(&self, _: &Self::Data) -> usize {
                let &$id(num) = self;
                num
            }

            fn count(data: &Self::Data) -> usize {
                data.count()
            }
        }
    }
}
