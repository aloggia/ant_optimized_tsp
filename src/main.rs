use std::{
    sync::{
        Mutex,
        Arc,
    },
    thread,
    env,
    collections::HashMap,
};
use petgraph::{
    Graph,
    Undirected
};
use petgraph::graph::{
    Frozen,
};
use petgraph::csr::EdgeIndex;
use rand::Rng;
use petgraph::adj::NodeIndex;
use std::convert::TryInto;
/*
TODO: either create or import a graph library
 */

/*
TODO: Code algorithm
 Algorithm idea: Pick starting node
 Create array of all possible destinations from that node
 Weight each edge for picking -> all edges have a distance/cost, but we need to weight them so the
 ant randomly picks an edge to take: Higher the distance, higher the weight
 When a node is visited, mark as visited so the ant isn't doubling back on itself
 Save the path the ant took as an array listing all visited nodes in order of visit
    - Because the ant can only travel on edges, the path will always be valid
    - The total cost of the path is the sum of the weights of all traveled edges
 */

// TODO: destination = distances[curr_idx, pot_nxt_idx]
// TODO: desirability = pow(1/destination, dst_power) * pow(pheromone_str, pheremone_pow)
// TODO: Multi-thread the ants -> run multiple ants at once
// TODO: pheremone_str = pheremoneTrails[curr_idx, pot_nxt_idx]
/*
TODO: parameters:
    - dst_power
    - pheremone_str
    - pheremone_intensity
    - initial_pheremone_intensity
    - evaporation_rate
    - num_ants
 */

fn main() {
    //let args: Vec<String> = env::args().collect();
    //let num_ants = &args[1].parse::<i32>().unwrap();
    let num_ants = 5;
    let num_nodes: i32 = 8;
    let mut nodes = Vec::new();
    //let mut edges = Vec::new();
    let mut ants = vec![];

    let mut graph = Graph::<i32, i32, Undirected>::new_undirected();

    for _ in 0..num_nodes + 1 {
        nodes.push(graph.add_node(1));
    }
    graph.add_edge(nodes[0], nodes[1], 7);
    graph.add_edge(nodes[0], nodes[5], 3);
    graph.add_edge(nodes[0], nodes[3], 27);
    graph.add_edge(nodes[1], nodes[2], 27);
    graph.add_edge(nodes[3], nodes[5], 29);
    graph.add_edge(nodes[3], nodes[4], 6);
    graph.add_edge(nodes[3], nodes[2], 18);
    graph.add_edge(nodes[5], nodes[4], 20);
    graph.add_edge(nodes[5], nodes[7], 5);
    graph.add_edge(nodes[2], nodes[4], 26);
    graph.add_edge(nodes[2], nodes[6], 9);
    graph.add_edge(nodes[2], nodes[7], 1);
    graph.add_edge(nodes[4], nodes[6], 28);
    graph.add_edge(nodes[4], nodes[7], 19);
    graph.add_edge(nodes[6], nodes[7], 12);
    graph.add_edge(nodes[4], nodes[8], 15);
    graph.add_edge(nodes[5], nodes[8], 23);
    graph.add_edge(nodes[6], nodes[8], 3);

    let graph = Arc::new(Mutex::new(graph));
    let edges: Arc<Mutex<HashMap<EdgeIndex, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    //let nodes: Arc<Mutex<Vec<_>>> = Arc::new(Mutex::new(nodes));

    for _ in 0..num_ants {
        let graph = Arc::clone(&graph);
        let edges = Arc::clone(&edges);
        let ant = thread::spawn(move || {
            crawl_path(&graph, &edges);
        });
        ants.push(ant);
    }
    for ant in ants {
        ant.join().unwrap();
    }
    // TODO: Need a hash map mapping edges -> deposited pheromone levels

    //let neighbors: Vec<_> = graph.lock().unwrap().neighbors(nodes[1]).collect();


    /*
    TODO: When choosing the next node do:
     let neighbors: Vec<_> = graph.neighbors(curr_node).collect();
     for i in neighbors {
        let weight = graph.edge_weight(graph.find_edge(nodes[1], nodes[2]).unwrap()).unwrap();
        let desirability = ((1/weight).pow(dst_pow)) * (pheromone_str.pow(pheromone_pow));
        Then we can probabilistically choose a dest depending on edge desirability
     */

    // Thread spawning loop
    /*for _ in 0..num_ants {
        // TODO: Create threads here
    }
     */
}

fn crawl_path(graph: &Arc<Mutex<Graph<i32, i32, Undirected>>>,
              edges: &Arc<Mutex<HashMap<EdgeIndex, i32>>>) {

    /*
    TODO:
     Pick starting node
     create array of size n where n = num verticies -> keep track of visited nodes
     create array of size n where n = num verticies -> order of visited nodes
     mark starting node as visited
     for node in neighbors:
        calculate probability for each node
        if node is visited:
            remove it from conideration
     chose node
     mark the chosen node as visited
     add the chosen node to visited_node array
     repeat until all nodes visited
     find combined weight of traversed edges
     pass vector of all traversed edges & combined weight into update pheromone function
     */
    let num_nodes = graph.lock().unwrap().node_count();
    let mut nodes: Vec<_> = Vec::new();
    for i in graph.lock().unwrap().node_indices() {
        nodes.push(i);
    }
    let start_node: usize = rand::thread_rng().gen_range(0..graph.lock().unwrap().node_count()).try_into().unwrap();
    let mut curr_node = nodes[start_node];
    let mut visited_nodes: Vec<bool> = vec![false; num_nodes];
    let mut order_of_travel: Vec<NodeIndex> = Vec::with_capacity(num_nodes);

    visited_nodes[start_node] = true;
    order_of_travel.push(start_node as u32);
    while order_of_travel.len() != num_nodes {
        let neighbors: Vec<_> = graph.lock().unwrap().neighbors(curr_node).collect();
        let mut neighbor_weight: Vec<usize> = Vec::with_capacity(neighbors.len());

        for neighbor in neighbors {
            let edge = graph.lock().unwrap().find_edge(curr_node, neighbor).unwrap();
            neighbor_weight.push(graph.lock().unwrap().edge_weight(edge).unwrap())
        }

        for edge in neighbor_weight {
            edge = graph.lock().unwrap()
            let weight = graph.lock().unwrap().find_edge(curr_node, )
        }
    }
}

fn update_pheromones() {
    /*
    TODO:
     for edge in edges:
        pull edge from edge_&_pheromone hash map
        update pheromone levels depending on score of traveresed path
     */
}

fn evaporate_pheromones() {
    /*
    TODO:
     For each edge in hash map
        decrease pheromone level by the evaporation rate
     */
}