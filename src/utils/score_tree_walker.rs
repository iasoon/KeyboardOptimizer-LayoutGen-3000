use data::score_tree::{Elem, Group, Children};
use errors::*;

pub trait ScoreTreeWalker<E, G> {
    fn visit_elem(&mut self, elem: &Elem) -> Result<E>;
    fn visit_group(&mut self, group: &Group) -> Result<G>;
}

pub fn map_children(walker: &mut ScoreTreeWalker<Elem, Group>,
                    children: &Children)
                    -> Result<Children>
{
    match *children {
        Children::Elems(ref elems) => {
            Ok(Children::Elems(collect_results(elems, |elem| walker.visit_elem(elem))?))
        }
        Children::Groups(ref groups) => {
            Ok(Children::Groups(collect_results(groups, |group| walker.visit_group(group))?))
        }
    }
}

pub fn visit_children(walker: &mut ScoreTreeWalker<(), ()>, children: &Children) -> Result<()> {
    match *children {
        Children::Elems(ref elems) => {
            for elem in elems.iter() {
                try!(walker.visit_elem(elem));
            }
        },
        Children::Groups(ref groups) => {
            for group in groups.iter() {
                try!(walker.visit_group(group));
            }
        },
    }
    Ok(())
}

fn collect_results<F, A, B>(vec: &Vec<A>, mut fun: F) -> Result<Vec<B>>
    where F: FnMut(&A) -> Result<B>
{
    let mut res = Vec::with_capacity(vec.capacity());
    for item in vec.iter() {
        res.push(fun(item)?);
    }
    Ok(res)
}
