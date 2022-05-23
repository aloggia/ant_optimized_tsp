#[allow(unused_imports)]
use std::{
    collections::HashMap,
    env,
    sync::{
        Arc,
        Mutex,
    },
    thread,
};
use std::convert::TryInto;

use petgraph::{
    Graph,
    Undirected,
};
use rand::{Rng, thread_rng};
use rand::distributions::{
    Distribution,
    WeightedIndex,
};

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
    let num_ants = 2;
    let num_nodes: i32 = 6;
    let mut nodes = Vec::new();
    let mut ants = vec![];
    // dst_pow < one
    let dst_pow: f64 = 0.5;
    // pheromone_pow > 0
    let pheromone_pow: f64 = 2.0;
    let evaporation_rate: f64 = 0.3;
    let pheromone_str: f64 = 1.2;

    let mut graph = Graph::<i32, i32, Undirected>::new_undirected();

    for _ in 0..num_nodes {
        nodes.push(graph.add_node(1));
    }

    let mut edges: HashMap<usize, f64> = HashMap::with_capacity(graph.edge_count());

    edges.insert(graph.add_edge(nodes[0], nodes[1], 17).index(), 1.0);
    edges.insert(graph.add_edge(nodes[0], nodes[2], 7).index(), 1.0);
    edges.insert(graph.add_edge(nodes[0], nodes[3], 24).index(), 1.0);
    edges.insert(graph.add_edge(nodes[0], nodes[4], 3).index(), 1.0);
    edges.insert(graph.add_edge(nodes[0], nodes[5], 13).index(), 1.0);
    edges.insert(graph.add_edge(nodes[1], nodes[2], 30).index(), 1.0);
    edges.insert(graph.add_edge(nodes[1], nodes[3], 4).index(), 1.0);
    edges.insert(graph.add_edge(nodes[1], nodes[4], 14).index(), 1.0);
    edges.insert(graph.add_edge(nodes[1], nodes[5], 15).index(), 1.0);
    edges.insert(graph.add_edge(nodes[2], nodes[3], 2).index(), 1.0);
    edges.insert(graph.add_edge(nodes[2], nodes[4], 21).index(), 1.0);
    edges.insert(graph.add_edge(nodes[2], nodes[5], 11).index(), 1.0);
    edges.insert(graph.add_edge(nodes[3], nodes[4], 18).index(), 1.0);
    edges.insert(graph.add_edge(nodes[3], nodes[5], 5).index(), 1.0);
    edges.insert(graph.add_edge(nodes[4], nodes[5], 9).index(), 1.0);


    let graph = Arc::new(Mutex::new(graph));
    let edges = Arc::new(Mutex::new(edges));



    let ant_paths: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::with_capacity((num_nodes * num_ants) as usize)));

    for _ in 0..num_ants {
        let graph = Arc::clone(&graph);
        let edges = Arc::clone(&edges);
        let ant_paths = Arc::clone(&ant_paths);
        let ant = thread::spawn(move || {
            let mut ant_path = crawl_path(&graph, &edges, dst_pow, pheromone_pow);
            ant_paths.lock().unwrap().append(&mut ant_path);
        });
        ants.push(ant);
    }
    for ant in ants {
        ant.join().unwrap();
    }
    println!("{:?}", ant_paths.lock().unwrap());
    update_pheromones(&graph,
                      &edges,
                      &ant_paths,
                      evaporation_rate,
                      pheromone_str,
                      (num_nodes - 1) as usize,
                      num_nodes, num_ants);
}

fn crawl_path(graph: &Arc<Mutex<Graph<i32, i32, Undirected>>>,
              edges: &Arc<Mutex<HashMap<usize, f64>>>,
              dst_pow: f64,
              pheromone_pow: f64) -> Vec<usize> {
    /*
     Pick starting node
     create array of size n where n = num vertices -> keep track of visited nodes
     create array of size n where n = num vertices -> order of visited nodes
     mark starting node as visited
     for node in neighbors:
        calculate probability for each node
        if node is visited:
            remove it from consideration
     chose node
     mark the chosen node as visited
     add the chosen node to visited_node array
     repeat until all nodes visited
     find combined weight of traversed edges
     pass vector of all traversed edges & combined weight into update pheromone function
     */
    /*
 When choosing the next node do:
 let neighbors: Vec<_> = graph.neighbors(curr_node).collect();
 for i in neighbors {
    let weight = graph.edge_weight(graph.find_edge(nodes[1], nodes[2]).unwrap()).unwrap();
    let desirability = ((1/weight).pow(dst_pow)) * (pheromone_str.pow(pheromone_pow));
    Then we can probabilistically choose a dest depending on edge desirability
 */
    /*
    Estimated complexity: O((num_nodes - 1) * ((num_nodes - 1) * 3))
     */
    let num_nodes = graph.lock().unwrap().node_count();
    let nodes: Vec<_> = graph.lock().unwrap().node_indices().collect();

    let start_node: usize = rand::thread_rng().gen_range(0..graph.lock().unwrap().node_count()).try_into().unwrap();
    //let start_node = 0;
    //println!("{:?}", start_node);
    let mut curr_node = nodes[start_node];
    let mut visited_nodes: Vec<bool> = vec![false; num_nodes];
    let mut order_of_travel: Vec<_> = Vec::with_capacity(num_nodes);

    visited_nodes[start_node] = true;
    order_of_travel.push(start_node);


    while order_of_travel.len() != num_nodes {
        let mut neighbors: Vec<_> = graph.lock().unwrap().neighbors(curr_node).collect();
        //println!("{:?}", &neighbors);
        let mut neighbor_desirability: Vec<f64> = Vec::with_capacity(neighbors.len());

        for neighbor in neighbors.iter() {
            let edge = graph.lock().unwrap().find_edge(curr_node, *neighbor).unwrap();
            let weight = *graph.lock().unwrap().edge_weight(edge).unwrap() as f64;
            let pheromone_str = *edges.lock().unwrap().get(&edge.index()).unwrap();
            neighbor_desirability.push((1.0 / weight).powf(dst_pow) *
                (pheromone_str).powf(pheromone_pow))
        }
        let total_probability: f64 = neighbor_desirability.iter().sum();
        for mut neighbor in &neighbor_desirability {
            neighbor = &(neighbor / total_probability);
        }
        //println!("{:?}", neighbor_desirability);

        let mut node_dist = WeightedIndex::new(&neighbor_desirability).unwrap();
        let mut chosen_node = neighbors[node_dist.sample(&mut thread_rng())];
        let chosen_node_idx_neighbors = neighbors
            .iter()
            .position(|&x| x == chosen_node)
            .unwrap();

        if visited_nodes[chosen_node.index()] == true {
            neighbors.remove(chosen_node_idx_neighbors);
            neighbor_desirability.remove(chosen_node_idx_neighbors);
        }

        let total_probability: f64 = neighbor_desirability.iter().sum();
        for mut neighbor in &neighbor_desirability {
            neighbor = &(neighbor / total_probability);
        }
        //println!("{:?}", neighbor_desirability);

        node_dist = WeightedIndex::new(&neighbor_desirability).unwrap();
        chosen_node = neighbors[node_dist.sample(&mut thread_rng())];
        while visited_nodes[chosen_node.index()] == true {
            chosen_node = neighbors[node_dist.sample(&mut thread_rng())];
        }
        let chosen_node_as_usize = chosen_node.index();

        order_of_travel.push(chosen_node_as_usize);
        visited_nodes[chosen_node_as_usize] = true;
        curr_node = chosen_node;
    }
    order_of_travel
}

#[allow(dead_code)]
fn update_pheromones(graph: &Arc<Mutex<Graph<i32, i32, Undirected>>>,
                     edges: &Arc<Mutex<HashMap<usize, f64>>>,
                     all_paths: &Arc<Mutex<Vec<usize>>>,
                     evaporation_rate: f64,
                     pheromone_str: f64,
                     edges_traversed: usize,
                     num_nodes: i32,
                     num_ants: i32) {
    // Indexing equation: ant * num_ants + sub_index
    /*
    TODO:
     pheromone put down by each ant: Q / Lk where Q is a constant and Lk is the the total cost of the
     kth ants tour
     for edge in traversal_array
        pheromone_lvl = ((1 - phermonone_const) * curr_pheromone_lvl) + sum of total pheromone put down on edge
     */
    let nodes: Vec<_> = graph.lock().unwrap().node_indices().collect();
    let mut ant_tour_cost = Vec::with_capacity(num_ants as usize);
    let mut all_paths_idx = 0;
    let mut total_path_cost = 0;
    let mut each_ant_idx = 0;

    // Loop through every element in in edges, and update the pheromone level by (1-evaporation_rate)
    for (_, value) in edges.lock().unwrap().iter_mut() {
        *value = (1.0 - evaporation_rate) * *value;
    }

    /*
     TODO: idea -> create Vec<Vec<_> to hold each edge in each path. Then we can loop through
      all the edges, look at each one as an index, and update its pheromones in the hash table
      Need to update (6 * num_ants) - num_ants edges

      TODO: Loop through each set: num_nodes - 1 times
     */
    while each_ant_idx < num_ants {
        //println!("At top of for loop");
        for i in 0..(num_nodes - 1) {
            let source_node_usize = *all_paths.lock().unwrap().get((each_ant_idx * nodes.len() as i32 + i) as usize).unwrap();
            let source_node = graph.lock().unwrap().node_indices().find(|x| x.index() == source_node_usize).unwrap();
            let dest_node_usize = *all_paths.lock().unwrap().get((each_ant_idx * nodes.len() as i32 + (i + 1)) as usize).unwrap();
            let dest_node = graph.lock().unwrap().node_indices().find(|x| x.index() == dest_node_usize).unwrap();
            let curr_edge = graph.lock().unwrap().find_edge(source_node, dest_node).unwrap();
            total_path_cost = total_path_cost + graph.lock().unwrap().edge_weight(curr_edge).unwrap();
            //println!("{:?}", each_ant_idx * num_ants + i);
        }
        ant_tour_cost.push(total_path_cost);
        //println!("{:?}", &ant_tour_cost.get(each_ant_idx as usize).unwrap());
        total_path_cost = 0;
        each_ant_idx = each_ant_idx + 1;
    }
    println!("{:?}", ant_tour_cost);

    each_ant_idx = 0;
    while each_ant_idx < num_ants {
        for i in 0..(num_nodes - 1) {
            let source_node_usize = *all_paths.lock().unwrap().get((each_ant_idx * nodes.len() as i32 + i) as usize).unwrap();
            let source_node = graph.lock().unwrap().node_indices().find(|x| x.index() == source_node_usize).unwrap();
            let dest_node_usize = *all_paths.lock().unwrap().get((each_ant_idx * nodes.len() as i32 + (i + 1)) as usize).unwrap();
            let dest_node = graph.lock().unwrap().node_indices().find(|x| x.index() == dest_node_usize).unwrap();
            let curr_edge = graph.lock().unwrap().find_edge(source_node, dest_node).unwrap().index();
            let curr_edge_weight = *edges.lock().unwrap().get(&curr_edge).unwrap();
            let new_edge_pheromone_lvl = curr_edge_weight + (pheromone_str / ant_tour_cost[each_ant_idx as usize] as f64);
            edges.lock().unwrap().insert(curr_edge, new_edge_pheromone_lvl);
        }
    }
}