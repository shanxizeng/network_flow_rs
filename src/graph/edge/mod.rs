pub(crate) struct Edge<T, E> {
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
}