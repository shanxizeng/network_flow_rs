//! 存储图中边的信息的数据结构

/// T 为边上容量的类型
/// 
/// E 为边上费用的类型
#[derive(Debug)]
pub struct Edge<T, E> {
    pub(crate) from : usize,
    pub(crate) to : usize,
    pub(crate) next_edge : usize,
    pub(crate) opp_edge : usize,
    pub(crate) weight : T,
    pub(crate) cost : E,
    pub(crate) reversed : bool
}

impl<T, E> Edge<T, E> 
    where
        T : Default,
        E : Default{
    pub(crate) fn empty_edge(i : usize) -> Edge<T, E> {
        Edge::<T, E> {
            from : i,
            to : i,
            next_edge : usize::MAX,
            opp_edge : usize::MAX,
            weight : T::default(),
            cost : E::default(),
            reversed : false
        }
    }

    pub(crate) fn create_edge(from : usize, to : usize, next_edge : usize, opp_edge : usize, 
        weight : T, cost : E) -> Edge<T, E> {
        Edge::<T, E> {
            from, to, next_edge, opp_edge, weight, cost, reversed : false
        }
    }

    pub fn is_reversed(&self) -> bool {
        self.reversed
    }

    pub fn get_to(&self) -> usize {
        self.to
    }
}
impl <T : Default + PartialEq, E> Edge<T, E> {
    pub fn is_full(&self) -> bool {
        self.weight == T::default()
    }
}