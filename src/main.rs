use network_flow::graph::Graph;
use std::io::Error;

fn main() -> Result<(),Error> {
    let mut g = Graph::<String, u32, u32>::new();
    g.add_node(&String::from("astesia1"));
    g.add_node(&String::from("astesia2"));
    g.add_node(&String::from("astesia3"));
    g.add_node(&String::from("astesia4"));
    g.add_node(&String::from("astesia5"));
    g.add_node(&String::from("astesia6"));
    g.add_edge(0, 1, &10);
    g.add_edge(0, 2, &3);
    g.add_edge(1, 2, &2);
    g.add_edge(1, 3, &5);
    g.add_edge(2, 4, &7);
    g.add_edge(3, 4, &1);
    g.add_edge(3, 5, &3);
    g.add_edge(4, 5, &9);
    assert_eq!(9, g.get_max_flow(0, 5));
    g.output_to_dot("test.dot")?;
    // let mut gg = Graph::<String, u32, u32>::input_file(String::from("test.txt")).unwrap();
    // println!("{:?}", g.get_max_flow(0, 5));
    Ok(())
}