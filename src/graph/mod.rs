//! 用于建立存储图的数据结构的module
pub mod edge;

use edge::*;
use core::ops::Add;
use core::ops::Sub;
// use std::collections::HashMap;
use std::marker::PhantomData;

/// 存储图的数据结构
/// 
/// 其中L为点的标签(label)的类型
/// 
/// T为边的容量的类型
/// 
/// E为边的费用的类型
/// 
/// M用于实现容量和费用的相乘
pub struct Graph<L, T, E, M : super::costtype::MulTE<T, E> = super::costtype::MulTEDefaultType> 
    where 
        L : Clone,
        E : Default + Add<E> + Sub<E>,
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
        E : Clone + Default + Add<E> + Sub<E>,
        T : Clone + Default + Add<T>,
        M : super::costtype::MulTE<T, E> {
    /// 初始化一个图， 将原有的边和点全部去除
    pub fn init_graph(&mut self) {
        self.edges = vec![];
        self.labels = vec![];
        self.first = vec![];
    }

    /// 创建一个初始为空的图
    pub fn new() -> Self {
        Self {
            labels : vec![],
            edges : vec![],
            first : vec![],
            m : PhantomData
        }
    }

    /// 使用一些点的标签来创建一个图，按照在vector中的顺序赋予其编号
    pub fn create_graph(nodes : &Vec<L>) -> Self {
        Self {
            labels : copy_nodes(nodes),
            edges : vec![],
            first : empty_edges(nodes.len()),
            m : PhantomData
        }
    }

    /// 在最后添加一个新的点
    pub fn add_node(&mut self, label : &L) {
        self.labels.push(label.clone());
        self.first.push(Edge::empty_edge(self.labels.len() - 1));        
    }

    /// 添加一条从from到to的边，容量为weight，费用为默认值
    pub fn add_edge(&mut self, from : usize, to : usize, weight : &T) {
        let mut edge = Edge::create_edge(
            from, to, self.first[from].next_edge, 0, weight.clone(), E::default());
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

    /// 添加一条从from到to的边，容量为weight，费用为cost
    pub fn add_edge2(&mut self, from : usize, to : usize, weight : &T, cost : &E) {
        let mut edge = Edge::create_edge(
            from, to, self.first[from].next_edge, 0, weight.clone(), cost.clone());
        let mut edge2 = Edge::create_edge(
            from, to, self.first[to].next_edge, 0, T::default(), cost.clone());
        edge.opp_edge = self.edges.len() + 1;
        edge2.opp_edge = self.edges.len();
        edge2.reversed = true;
        self.first[from].next_edge = self.edges.len();
        self.first[to].next_edge = self.edges.len() + 1;
        self.edges.push(edge);
        self.edges.push(edge2);
    }

    /// 获得从index指出的第一条边
    pub fn first_edge(&self, index : usize) -> Option<&Edge<T, E>> {
        if self.first[index].next_edge == usize::MAX {
            None
        }
        else {
            Some(&self.edges[self.first[index].next_edge])
        }
    }

    /// 获得和now从同一个点指出的下一条边
    /// 
    /// 配合first_edge可以这样使用：
    /// 
    /// ```rust
    /// let mut temp = graph.first_edge(0);
    /// while let Some(edge) = temp {
    ///     // do something
    ///     temp = graph.next_edge(edge);
    /// }
    /// ```
    pub fn next_edge(&self, now : &Edge<T, E>) -> Option<&Edge<T, E>> {
        if now.next_edge == usize::MAX {
            None
        }
        else {
            Some(&self.edges[now.next_edge])
        }
    }

    /// 获得与index相邻的所有点
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
