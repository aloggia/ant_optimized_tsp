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

    let mut raw_graph = Graph::<i32, i32, Undirected>::new_undirected();

    for _ in 0..num_nodes + 1 {
        nodes.push(raw_graph.add_node(1));
    }
    raw_graph.add_edge(nodes[0], nodes[1], 7);
    raw_graph.add_edge(nodes[0], nodes[5], 3);
    raw_graph.add_edge(nodes[0], nodes[3], 27);
    raw_graph.add_edge(nodes[1], nodes[2], 27);
    raw_graph.add_edge(nodes[3], nodes[5], 29);
    raw_graph.add_edge(nodes[3], nodes[4], 6);
    raw_graph.add_edge(nodes[3], nodes[2], 18);
    raw_graph.add_edge(nodes[5], nodes[4], 20);
    raw_graph.add_edge(nodes[5], nodes[7], 5);
    raw_graph.add_edge(nodes[2], nodes[4], 26);
    raw_graph.add_edge(nodes[2], nodes[6], 9);
    raw_graph.add_edge(nodes[2], nodes[7], 1);
    raw_graph.add_edge(nodes[4], nodes[6], 28);
    raw_graph.add_edge(nodes[4], nodes[7], 19);
    raw_graph.add_edge(nodes[6], nodes[7], 12);
    raw_graph.add_edge(nodes[4], nodes[8], 15);
    raw_graph.add_edge(nodes[5], nodes[8], 23);
    raw_graph.add_edge(nodes[6], nodes[8], 3);

    let graph = Arc::new(Mutex::new(Frozen::new(&mut raw_graph)));

    for _ in 0..num_ants {
        let graph = Arc::clone(&graph);
        let ant = thread::spawn(move || {

        });
        ants.push(ant);
    }
    for ant in ants {
        ant.join().unwrap();
    }
    // TODO: Need a hash map mapping edges -> deposited pheremone levels

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

fn crawl_path() {
    /*
    TODO:
     Pick starting node
     create array of size n where n = num verticies -> keep track of visited nodes
     create array of size n where n = num verticies -> order of visited nodes
     mark starting node as visited
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