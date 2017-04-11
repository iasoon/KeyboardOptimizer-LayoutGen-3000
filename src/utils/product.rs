use utils::countable::Countable;

impl<Fst, Snd> Countable for (Fst, Snd)
    where Fst: Countable,
          Snd: Countable
{
    type Data = (Fst::Data, Snd::Data);

    fn from_num(data: &Self::Data, num: usize) -> Self {
        let fst_num = num / Snd::count(&data.1);
        let snd_num = num % Snd::count(&data.1);
        (Fst::from_num(&data.0, fst_num), Snd::from_num(&data.1, snd_num))
    }

    fn to_num(&self, data: &Self::Data) -> usize {
        self.0.to_num(&data.0) * Snd::count(&data.1) + self.1.to_num(&data.1)
    }

    fn count(data: &Self::Data) -> usize {
        Fst::count(&data.0) * Snd::count(&data.1)
    }
}
