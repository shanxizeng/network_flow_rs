mod edge;

use edge::*;
use core::ops::Add;
use core::ops::Mul;
use core::ops::Sub;
use std::collections::HashMap;
use std::marker::PhantomData;

pub(crate) struct Graph<L, T, E, M> 
    where 
        L : Clone,
        E : Default + Add<E> + Mul<T> + Sub<E>,
        T : Default + Add<T> {
    labels : Vec<L>,
    edges : Vec<Edge<T, E>>,
    first : Vec<Edge<T, E>>,
    m : PhantomData<M>
}

fn copy_nodes<L : Clone>(nodes : &Vec<L>) -> Vec<L> {
    let mut res = vec![];
    for i in nodes {
        res.push(i.clone());
    }
    res
}

fn empty_edges<T : Default, E : Default>(len : usize) -> Vec<Edge<T, E>> {
    let mut res = vec![];
    for i in 0..len {
        res.push(Edge::empty_edge(i));
    }
    res
}

impl<L, T, E, M> Graph<L, T, E, M> 
    where 
        L : Clone,
        E : Default + Add<E> + Mul<T> + Sub<E>,
        T : Default + Add<T> {
    pub fn init_graph(&mut self) {
        self.edges = vec![];
        self.labels = vec![];
        self.first = vec![];
    }

    pub fn new() -> Self {
        Self {
            labels : vec![],
            edges : vec![],
            first : vec![],
            m : PhantomData
        }
    }

    pub fn create_graph(nodes : &Vec<L>) -> Self {
        Self {
            labels : copy_nodes(nodes),
            edges : vec![],
            first : empty_edges(nodes.len()),
            m : PhantomData
        }
    }

    pub fn add_node(&mut self, label : L) {
        self.labels.push(label);
        self.first.push(Edge::empty_edge(self.labels.len() - 1));        
    }

    pub fn add_edge(&mut self, from : usize, to : usize, weight : T) {
        let mut edge = Edge::create_edge(
            from, to, self.first[from].next_edge, 0, weight, E::default());
        let mut edge2 = Edge::create_edge(
            from, to, self.first[to].next_edge, 0, T::default(), E::default());
        edge.opp_edge = self.edges.len() + 1;
        edge2.opp_edge = self.edges.len();
        edge2.reversed = true;
        self.first[from].next_edge = self.edges.len();
        self.first[to].next_edge = self.edges.len() + 1;
        self.edges.push(edge);
        self.edges.push(edge2);
    }

    pub fn first_edge(&self, index : usize) -> Option<&Edge<T, E>> {
        if self.first[index].next_edge == usize::MAX {
            None
        }
        else {
            Some(&self.edges[self.first[index].next_edge])
        }
    }

    pub fn next_edge(&self, now : &Edge<T, E>) -> Option<&Edge<T, E>> {
        if now.next_edge == usize::MAX {
            None
        }
        else {
            Some(&self.edges[now.next_edge])
        }
    }

    pub fn get_neighbor(&self, index : usize) -> Vec<usize> {
        let mut res = vec![];
        let mut edge = self.first_edge(index);
        while let Some(x) = edge {
            res.push(x.to);
            edge = self.next_edge(x);
        }
        res
    }
}
