//! 用于建立存储图的数据结构的module
pub mod edge;

use edge::*;
use core::ops::Add;
use core::ops::Sub;
use core::mem::size_of;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;
use std::io::Write;
// use std::collections::HashMap;
use std::marker::PhantomData;
use std::fs::File;
use std::io::Error;

/// 存储图的数据结构
/// 
/// 其中L为点的标签(label)的类型
/// 
/// T为边的容量的类型
/// 
/// E为边的费用的类型
/// 
/// M用于实现容量和费用的相乘
#[derive(Debug)]
pub struct Graph<L : Hash, T, E = (), M : super::costtype::MulTE<T, E> = super::costtype::MulTEDefaultType> {
    labels : Vec<L>,
    pub edges : Vec<Edge<T, E>>,
    first : Vec<Edge<T, E>>,
    m : PhantomData<M>,
    hs : HashMap<L, usize>
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

fn make_hash<L : Clone + Hash + Eq>(nodes : &Vec<L>) -> HashMap<L, usize> {
    let mut res = HashMap::new();
    for i in 0..nodes.len() {
        res.insert(nodes[i].clone(), i);
    }
    res
}

impl<L, T, E, M> Graph<L, T, E, M> 
    where 
        L : Clone + Hash + Eq,
        E : Default,
        T : Default,
        M : super::costtype::MulTE<T, E> {

    /// 获得某一个label对应的编号
    pub fn get_index(&self, label : &L) -> Option<usize> {
        self.hs.get(label).map(|x| *x)
    }

    /// 获得某一个编号对应的label
    pub fn get_label(&self, index : usize) -> Option<&L> {
        if index >= self.labels.len() {
            None
        }
        else {
            Some(&self.labels[index])
        }
    }

    /// 初始化一个图， 将原有的边和点全部去除
    pub fn init_graph(&mut self) {
        self.edges = vec![];
        self.labels = vec![];
        self.first = vec![];
        self.hs.clear();
    }

    /// 创建一个初始为空的图
    pub fn new() -> Self {
        Self {
            labels : vec![],
            edges : vec![],
            first : vec![],
            m : PhantomData,
            hs : HashMap::<L, usize>::new()
        }
    }

    /// 使用一些点的标签来创建一个图，按照在vector中的顺序赋予其编号
    pub fn create_graph(nodes : &Vec<L>) -> Self {
        Self {
            labels : copy_nodes(nodes),
            edges : vec![],
            first : empty_edges(nodes.len()),
            m : PhantomData,
            hs : make_hash(nodes)
        }
    }

    /// 在最后添加一个新的点
    pub fn add_node(&mut self, label : &L) {
        self.labels.push(label.clone());
        self.first.push(Edge::empty_edge(self.labels.len() - 1));
        self.hs.insert(label.clone(), self.labels.len() - 1);
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

    fn first_edge_mut(&mut self, index : usize) -> Option<&mut Edge<T, E>> {
        if self.first[index].next_edge == usize::MAX {
            None
        }
        else {
            Some(&mut self.edges[self.first[index].next_edge])
        }
    }

    /// 获得和now从同一个点指出的下一条边
    /// 
    /// 配合first_edge可以这样使用：
    /// 
    /// ```ignore
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
    
    fn next_edge_mut(&mut self, now : &mut Edge<T, E>) -> Option<&mut Edge<T, E>> {
        if now.next_edge == usize::MAX {
            None
        }
        else {
            Some(&mut self.edges[now.next_edge])
        }
    }

    /// 使用first_edge和next_edge函数得到从index出发的所有边及其编号
    pub fn get_all_edges(&self, index : usize) -> Vec<(&Edge<T, E>, usize)> {
        let mut res = vec![];
        let mut temp = self.first_edge(index);
        let mut no = self.first[index].next_edge;
        while let Some(edge) = temp {
            res.push((edge, no));
            no = edge.next_edge;
            temp = self.next_edge(edge);
        }
        res
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

impl<L, T, E, M> Graph<L, T, E, M> 
    where 
        L : Clone + Hash + Eq,
        E : Clone + Default + Add<Output = E> + Sub<Output = E> + PartialEq + PartialOrd,
        T : Clone + Default + Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd,
        M : super::costtype::MulTE<T, E> {
    

    /// 添加一条从from到to的边，容量为weight，费用为默认值
    pub fn add_edge(&mut self, from : usize, to : usize, weight : &T) {
        let mut edge = Edge::create_edge(
            from, to, self.first[from].next_edge, 0, weight.clone(), E::default());
        let mut edge2 = Edge::create_edge(
            to, from, self.first[to].next_edge, 0, T::default(), E::default());
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

    /// 求从s到t的最大流
    pub fn get_max_flow(&mut self, s : usize, t : usize) -> T {
        self.dinic(s, t)
    }

    fn bfs(&self, levels : &mut Vec<u32>, s : usize) {
        levels[s] = 1;
        let mut q1 = vec![];
        let mut q2 = vec![];
        q2.push(s);
        while ! q1.is_empty() || ! q2.is_empty() {
            if q1.is_empty() {
                while !q2.is_empty() {
                    q1.push(q2.pop().unwrap());
                }
            }
            let now = q1.pop().unwrap();
            let mut temp = self.first_edge(now);
            while let Some(edge) = temp {
                let x = edge.to;
                if edge.weight != T::default() && levels[x] == 0 {
                    levels[x] = levels[now] + 1;
                    q2.push(x);
                }
                temp = self.next_edge(edge);
            }
        }
    }

    fn dfs(&mut self, now : usize, t : usize, levels : &Vec<u32>, flow : T) -> T {
        if now == t {
            flow
        }
        else {
            let edges = self.get_all_edges(now);
            let mut v = vec![];
            for (edge, index) in edges {
                v.push((edge.weight.clone(), edge.to, index));
            }
            let mut res = T::default();
            for (w, x, index) in v {
                if w != T::default() && levels[x] == levels[now] + 1 {
                    if flow == T::default() {
                        res = self.dfs(x, t, levels, w);
                    }
                    else if flow < w {
                        res = self.dfs(x, t, levels, flow.clone());
                    }
                    else {
                        res = self.dfs(x, t, levels, w);
                    }
                    if res != T::default() {
                        self.edges[index].weight = self.edges[index].weight.clone() - res.clone();
                        let temp = self.edges[index].opp_edge;
                        self.edges[temp].weight = self.edges[temp].weight.clone() + res.clone();
                        return res;
                    }
                }
            }
            res
        }
    }

    fn dinic(&mut self, s : usize, t : usize) -> T {
        let mut res = T::default();
        loop {
            let mut levels = vec![0; self.labels.len()];
            self.bfs(&mut levels, s);
            if levels[t] == 0 {
                break res
            }
            loop {
                let temp = self.dfs(s, t, &levels, T::default());
                if temp == T::default() {
                    break
                }
                res = res + temp;
            }
        }
    }

    fn spfa(&self, s : usize, t : usize, dist : &mut [E]) -> bool {
        let mut q = VecDeque::new();
        q.push_back(t); dist[t] = E::default();
        let mut vis = vec![false; self.labels.len()];
        let mut inque = vec![false; self.labels.len()];
        inque[t] = true;
        vis[t] = true;
        while !q.is_empty() {
            let now = q.pop_front().unwrap();
            let edges = self.get_all_edges(now);
            let mut v = vec![];
            for (edge, _) in edges {
                v.push((self.edges[edge.opp_edge].weight.clone(), edge.cost.clone(), edge.to, edge.reversed));
            }
            for (w, c, to, r) in v {
                if w == T::default() {
                    continue;
                }
                let newcost = if r { dist[now].clone() + c } else { dist[now].clone() - c };
                if !vis[to] || dist[to] > newcost {
                    vis[to] = true;
                    dist[to] = newcost;
                    if !inque[to] {
                        inque[to] = true;
                        if q.is_empty() || dist[*q.front().unwrap()] < dist[to] {
                            q.push_back(to);
                        }
                        else {
                            q.push_front(to);
                        }
                    }
                }
            }
            inque[now] = false;
        }
        vis[s]
    }

    fn mcmf_dfs(&mut self, now : usize, flow : T, cost : &mut E, dist : &[E], vis : &mut [bool], t : usize) -> T {
        vis[now] = true;
        if now == t {
            flow
        }
        else {
            let edges = self.get_all_edges(now);
            let mut v = vec![];
            for (edge, index) in edges {
                v.push((edge.opp_edge, edge.weight.clone(), edge.cost.clone(), edge.to, edge.reversed, index));
            }
            for (opp, w, c, to, r, i) in v {
                let temp = if r {dist[to].clone() - c.clone()} else {dist[to].clone() + c.clone()};
                if dist[now] != temp || vis[to] || w == T::default() {
                    continue;
                }
                let f : T;
                if flow == T::default() {
                    f = self.mcmf_dfs(to, w, cost, dist, vis, t);
                }
                else if flow < w {
                    f = self.mcmf_dfs(to, flow.clone(), cost, dist, vis, t);
                }
                else {
                    f = self.mcmf_dfs(to, w, cost, dist, vis, t);
                }
                if f != T::default() {
                    *cost = cost.clone() + M::mul(&f, &c);
                    self.edges[i].weight = self.edges[i].weight.clone() - f.clone();
                    self.edges[opp].weight = self.edges[opp].weight.clone() + f.clone();
                    return f;
                }
            }
            T::default()
        }
    }

    /// 求从s到t的最小费用最大流
    pub fn mcmf(&mut self, s : usize, t : usize) -> (T, E) {
        let mut cost = E::default();
        let mut flow = T::default();
        let mut dist = vec![E::default(); self.labels.len()];
        while self.spfa(s, t,&mut dist) {
            let mut vis = vec![false; self.labels.len()];
            vis[t] = true;
            while vis[t] {
                for i in &mut vis {
                    *i = false;
                }
                flow = flow + self.mcmf_dfs(s, T::default(), &mut cost, &dist, &mut vis, t);
            }
        }
        (flow, cost)
    }
}

use crate::io::BitIO;

impl<L, T, E, M : super::costtype::MulTE<T, E>> Graph<L, T, E, M> 
    where
        L : BitIO + Clone + Hash + Eq,
        E : BitIO + Clone + Default,
        T : BitIO + Clone + Default {
    /// 将当前的图的状态输出到文件中
    /// 
    /// L, T, E均需实现BitIO trait
    pub fn output_file(&self, file : &str) -> Result<(), Error> {
        let mut fs = File::create(file)?;
        fs.write(&self.labels.len().to_be_bytes())?;
        for i in &self.labels {
            let temp = i.to_bit();
            fs.write(&temp.len().to_be_bytes())?;
            fs.write(&temp)?;
        }
        fs.write(&self.edges.len().to_be_bytes())?;
        for edge in &self.edges {
            fs.write(&edge.from.to_be_bytes())?;
            fs.write(&edge.to.to_be_bytes())?;
            fs.write(&edge.next_edge.to_be_bytes())?;
            fs.write(&edge.opp_edge.to_be_bytes())?;
            fs.write(&(edge.reversed as u8).to_be_bytes())?;
            let temp = edge.weight.to_bit();
            fs.write(&temp.len().to_be_bytes())?;
            fs.write(&temp)?;
            let temp = edge.cost.to_bit();
            fs.write(&temp.len().to_be_bytes())?;
            fs.write(&temp)?;
        }
        fs.write(&self.first.len().to_be_bytes())?;
        for edge in &self.first {
            fs.write(&edge.from.to_be_bytes())?;
            fs.write(&edge.to.to_be_bytes())?;
            fs.write(&edge.next_edge.to_be_bytes())?;
            fs.write(&edge.opp_edge.to_be_bytes())?;
            fs.write(&(edge.reversed as u8).to_be_bytes())?;
            let temp = edge.weight.to_bit();
            fs.write(&temp.len().to_be_bytes())?;
            fs.write(&temp)?;
            let temp = edge.cost.to_bit();
            fs.write(&temp.len().to_be_bytes())?;
            fs.write(&temp)?;
        }
        Ok(())
    }
    /// 从文件中生成一个图
    /// 
    /// L, T, E均需实现BitIO trait
    pub fn input_file(file : &str) -> Result<Self, Error> {
        let mut res = Self::new();
        let mut fs = File::open(file)?;
        let mut buf = [0; size_of::<usize>()];
        assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
        let len = usize::from_be_bytes(buf);
        for _ in 0..len {
            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let len = usize::from_be_bytes(buf);
            let mut buf2 = vec![0; len];
            assert_eq!(fs.read(&mut buf2)?, len);
            let l = L::from_bit(&buf2);
            res.labels.push(l);
        }
        assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
        let len = usize::from_be_bytes(buf);
        for _ in 0..len {
            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let from = usize::from_be_bytes(buf);

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let to = usize::from_be_bytes(buf);

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let next_edge = usize::from_be_bytes(buf);

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let opp_edge = usize::from_be_bytes(buf);

            let mut buf3 = [0];
            assert_eq!(fs.read(&mut buf3)?, 1);
            let reversed = u8::from_be_bytes(buf3) != 0;

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let len = usize::from_be_bytes(buf);
            let mut buf2 = vec![0; len];
            assert_eq!(fs.read(&mut buf2)?, len);
            let weight = T::from_bit(&buf2);

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let len = usize::from_be_bytes(buf);
            let mut buf2 = vec![0; len];
            assert_eq!(fs.read(&mut buf2)?, len);
            let cost = E::from_bit(&buf2);
            
            res.edges.push(Edge{from, to, next_edge, opp_edge, reversed, weight, cost});
        }
        assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
        let len = usize::from_be_bytes(buf);
        for _ in 0..len {
            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let from = usize::from_be_bytes(buf);

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let to = usize::from_be_bytes(buf);

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let next_edge = usize::from_be_bytes(buf);

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let opp_edge = usize::from_be_bytes(buf);

            let mut buf3 = [0];
            assert_eq!(fs.read(&mut buf3)?, 1);
            let reversed = u8::from_be_bytes(buf3) != 0;

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let len = usize::from_be_bytes(buf);
            let mut buf2 = vec![0; len];
            assert_eq!(fs.read(&mut buf2)?, len);
            let weight = T::from_bit(&buf2);

            assert_eq!(fs.read(&mut buf)?, size_of::<usize>());
            let len = usize::from_be_bytes(buf);
            let mut buf2 = vec![0; len];
            assert_eq!(fs.read(&mut buf2)?, len);
            let cost = E::from_bit(&buf2);
            
            res.first.push(Edge{from, to, next_edge, opp_edge, reversed, weight, cost});
        }
        Ok(res)
    }
}

use crate::io::StrIO;
use graphviz_rust_bla::{exec, parse};
use graphviz_rust_bla::cmd::{CommandArg, Format};
use graphviz_rust_bla::printer::{PrinterContext, DotPrinter};
use graphviz_rust_bla::attributes::*;

impl <L, T, E, M : super::costtype::MulTE<T, E>> Graph<L, T, E, M> 
where
    L : StrIO + Clone + Hash + Eq,
    E : StrIO + Clone + Default + Add<Output = E> + Sub<Output = E> + PartialEq + PartialOrd,
    T : StrIO + Clone + Default + Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd, {
    pub fn output_to_dot(&self, file : &str) -> Result<(), Error> {
        use dot_structures::*;
        use dot_generator::*;
        let mut fs = File::create(file)?;
        let mut g = graph!(di id!(file));
        let mut temp = 0;
        for l in &self.labels {
            g.add_stmt(stmt!(node!(temp.to_str();attr!("label",l.to_str()))));
            temp = temp + 1;
        }
        for e in &self.edges {
            if e.reversed { continue; }
            let temp = self.edges[e.opp_edge].weight.clone();
            let tot = temp.clone() + e.weight.clone();
            let mut s = String::from("\"");
            s.push_str(&temp.to_str());
            s.push('/');
            s.push_str(&tot.to_str());
            s.push(',');
            s.push_str(&e.cost.to_str());
            s.push('"');
            g.add_stmt(stmt!(edge!(node_id!(e.from) => node_id!(e.to);attr!("label",s))));
        }
        let mut ctx = PrinterContext::default();
        fs.write_all(g.print(&mut ctx).as_bytes())?;
        Ok(())
    }

    fn get_from_id(x : &dot_structures::Id) -> &String {
        match x {
            dot_structures::Id::Html(s) => s,
            dot_structures::Id::Escaped(s) => s,
            dot_structures::Id::Plain(s) => s,
            dot_structures::Id::Anonymous(s) => s,
        }
    }

    fn get_from_vertex(x : &dot_structures::Vertex) -> usize {
        match x {
            dot_structures::Vertex::N(x) => usize::from_str(Self::get_from_id(&x.0)),
            _ => 0,
        }
    }

    pub fn from_dot(file : &str) -> Result<Self, Error> {
        let mut res = Self::new();
        use dot_structures::Graph::DiGraph;
        use dot_structures::Stmt::*;
        let mut fs = File::open(file)?;
        let mut buf = vec![];
        fs.read_to_end(&mut buf)?;
        let s = parse(&String::from_utf8(buf).unwrap()).unwrap();
        match s {
            DiGraph { id : _, strict : _, stmts } => {
                for stmt in stmts {
                    match stmt {
                        Node(node) => {
                            res.add_node(&L::from_str(Self::get_from_id(&node.attributes[0].1)));
                        },
                        Edge(edge) => {
                            let mut from = 0;
                            let mut to = 0;
                            match edge.ty {
                                dot_structures::EdgeTy::Pair(u, v) => {
                                    from = Self::get_from_vertex(&u);
                                    to = Self::get_from_vertex(&v);
                                },
                                _ => {}
                            }
                            if from == to {
                                panic!("from_dot : invalid edge form");
                            }
                            let mut s = Self::get_from_id(&edge.attributes[0].1).clone();
                            let p = s.find('/').unwrap();
                            let mut ss = s.split_off(p);
                            let w = T::from_str(&s);
                            s = ss.split_off(1);
                            let p = s.find(',').unwrap();
                            ss = s.split_off(p);
                            let ww = T::from_str(&s);
                            s = ss.split_off(1);
                            let c = E::from_str(&s);
                            res.add_edge2(from, to, &ww, &c);
                            let l = res.edges.len();
                            res.edges[l - 1].weight = res.edges[l - 1].weight.clone() + w.clone();
                            res.edges[l - 2].weight = res.edges[l - 2].weight.clone() - w.clone();
                        },
                        _ => ()
                    }
                }
            },
            _ => {panic!("from_dot : invalid graph form");}
        }
        Ok(res)
    }
}