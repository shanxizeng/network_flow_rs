use network_flow::graph::Graph;

// https://www.luogu.com.cn/problem/P1251
#[test]
fn p1251() {
    let n = 3;
    let arr = [1, 7, 5];
    let a = 11;
    let b = 2;
    let c = 2;
    let d = 3;
    let e = 1;
    let mut g = Graph::<usize, i32, i32>::new();
    for i in 0..2*n+3 {
        g.add_node(&i);
    }
    let s = 0;
    let t = 2*n+2;
    for i in 1..n+1 {
        g.add_edge2(s, i, &i32::MAX, &a);
        g.add_edge2(i, t, &arr[i-1], &0);
        g.add_edge2(s, i+n, &arr[i-1], &0);
        g.add_edge2(i+n, i+n+1, &i32::MAX, &0);
        if i+b<=n {
            g.add_edge2(i+n, i+b, &i32::MAX, &c);
        }
        if i+d<=n {
            g.add_edge2(i+n, i+d, &i32::MAX, &e);
        }
    }
    let (_, c) = g.mcmf(s, t);
    assert_eq!(c, 134);
}

// https://www.luogu.com.cn/problem/P2754
#[test]
fn p2754() {
    let (n, m, k) = (2, 2, 1);
    let r = [1, 1];
    let arr: [[i32; 4]; 2] = [[3, 0, 1, 2], [3, 1, 2, -1]];
    let (s, t, mut temp) = (1, 2, 3);
    let mut las = vec![0; n+2];
    let mut c = vec![0; n+2];
    let mut g = Graph::<usize, i32>::create_graph(&vec![0, 1, 2]);
    las[0] = s;
    c[0] = s;
    las[n+1] = t;
    c[n+1] = t;
    for i in 1..n+1 {
        las[i] = temp;
        g.add_node(&temp);
        temp += 1;
    }
    let mut ans = 1;
    let mut flag = 0;
    loop {
        if ans > 200 && flag == 0 {
            break;
        }
        for i in 1..n+1 {
            c[i] = temp;
            g.add_node(&temp);
            temp = temp + 1;
            g.add_edge(las[i], c[i], &i32::MAX);
        }
        for i in 0..m {
            let y = arr[i][0];
            let x = ans % (y as usize) + 1;
            let u = if x == 1 { arr[i][y as usize] } else { arr[i][x-1] };
            let v = arr[i][x];
            let u = (u+(n as i32)+2)as usize % (n+2);
            let v = (v+(n as i32)+2)as usize % (n+2);
            g.add_edge(las[u], c[v], &r[i]);
        }
        flag += g.get_max_flow(s, t);
        if flag >= k {
            break;
        }
        for i in 1..n+1 {
            las[i] = c[i];
        }
        ans += 1;
    }
    assert_eq!(ans, 5);
}

// https://www.luogu.com.cn/problem/P2756
#[test]
fn p2756() {
    let n = 5;
    let m = 10 - n;
    let s = 0;
    let t = n + m + 1;
    let mut g = Graph::<usize, i32>::create_graph(&vec![0;t+2]);
    for i in 1..n+1 {
        g.add_edge(s, i, &1);
    }
    for i in n+1..t {
        g.add_edge(i, t, &1);
    }
    let p = [(1, 7), (1, 8), (2, 6), (2, 9), (2, 10), 
                               (3, 7), (3, 8), (4, 7), (4, 8), (5, 10)];
    for (u, v) in p {
        g.add_edge(u, v, &1);
    }
    assert_eq!(4, g.get_max_flow(s, t));
    let mut res = vec![];
    for i in 1..n+1 {
        let e = g.get_all_edges(i);
        for (edge, _) in e {
            if edge.is_full() && !edge.is_reversed() {
                res.push((i, edge.get_to()));
            }
        }
    }
    fn check_res(res : Vec<(usize, usize)>, p : &[(usize, usize)]) -> bool { 
        let mut ans = true;
        let mut hs1 = std::collections::HashSet::<usize>::new();
        let mut hs2 = std::collections::HashSet::<(usize, usize)>::new();
        for i in p {
            hs2.insert(*i);
        }
        for (x, y) in &res {
            if hs1.contains(x) {
                ans = false;
            }
            hs1.insert(*x);
            if hs1.contains(y) {
                ans = false;
            }
            hs1.insert(*y);
            if !hs2.contains(&(*x,*y)) {
                ans = false;
            }
        }
        ans
    }
    assert!(check_res(res, &p));
}

// https://www.luogu.com.cn/problem/P2762
#[test]
fn p2762() {
    let (n, m) = (2, 3);
    let s = 0;
    let t = n + m + 1;
    let mut g = Graph::<usize, usize>::create_graph(&vec![0; t + 1]);
    let arr = [[10, 1, 2], [25, 2, 3]];
    let mut sum = 0;
    for i in 0..n {
        sum += arr[i][0];
        g.add_edge(s, i + 1, &arr[i][0]);
        for j in &arr[i][1..] {
            g.add_edge(i + 1, *j+n, &usize::MAX);
        }

    }
    let arr = [5, 6, 7];
    for i in 0..m {
        g.add_edge(i+n+1, t, &arr[i]);
    }
    assert_eq!(sum - g.get_max_flow(s, t), 17);
    let v1 = g.get_cut(s);
    let mut r1 = vec![];
    let mut r2 = vec![];
    for i in v1 {
        if i > n {
            r2.push(i - n);
        }
        else if i > 0 {
            r1.push(i);
        }
    }
    r1.sort();
    r2.sort();
    assert_eq!(r1, vec![1, 2]);
    assert_eq!(r2, vec![1, 2 ,3]);
}