extern crate petgraph;
use petgraph::GraphMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;



fn get_ids(line: &str) -> Vec<i64> {
/*
from a string line get a vector that contain a list of ids.
*/
    let split = line.split(",").take(2);
    let mut id_vec: Vec<i64> = Vec::new();
    for item in split {
        id_vec.push(item.parse::<i64>().unwrap());
    }
    id_vec

}


fn connect(node_a: i64, node_b: i64, graph: &mut GraphMap<i64, i64>) {
// This helper function connect node_a to node_b by adding an edge in the graph.
    if graph.contains_edge(node_a, node_b) {
        *graph.edge_weight_mut(node_a, node_b).unwrap() += 1;
    } else {
        graph.add_edge(node_a, node_b, 1);
    }
}

fn build_connection(node_pool: &mut Vec<i64>, graph: &mut GraphMap<i64, i64>) {
// This helper function will build connections among all the nodes in the node_pool
    let node_number = node_pool.len();
    if node_number == 1 {
        graph.add_node(node_pool.pop().unwrap());
    }
    if node_number == 2 {
        connect(node_pool.pop().unwrap(), node_pool.pop().unwrap(), graph);
    }
    if node_number > 2 {
        let head_node = node_pool.pop().unwrap();
        for node in node_pool.clone() {
            connect(head_node, node, graph);
        }
        build_connection(node_pool, graph);
    }

}

fn main() {
    let file_handle = File::open("/Users/divinites/Dropbox/reproj/economics-github/data/github_data_project_members.csv")
        .unwrap(); //This is to read a csv file export from github database, in the working version, it would be better to directly read mysql database.
    let f = BufReader::new(file_handle);
    let mut network: GraphMap<i64, i64> = petgraph::GraphMap::new();
    let mut initial_repo_id: i64 = 0;
    let mut user_pool: Vec<i64> = Vec::new();
    for line in f.lines() {
        let ids: Vec<i64> = get_ids(line.unwrap().as_str());
        let repo_id = ids[0];
        let user_id = ids[1];
        if initial_repo_id == 0 {
            initial_repo_id = repo_id;
            user_pool.push(user_id);
        } else {
            if initial_repo_id == repo_id {
                user_pool.push(user_id);
            } else {
                build_connection(&mut user_pool, &mut network);
                initial_repo_id = repo_id;
                user_pool.clear();
                user_pool.push(user_id);
            }

        }

    }
    build_connection(&mut user_pool, &mut network);
}