//DS 210 final Project - Peiyuan Wu 
//Collaborators: None 
//Dec 15 2022 

use std::collections::VecDeque;
#[allow(dead_code)]

pub mod read_csv;
//call the module read_csv 
use read_csv::{read_csv_features, read_csv_edges};

//initiate some types for later use
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;


//struct for the undirected graph
struct Graph{
    n: usize,
    outedges: AdjacencyLists
}

impl Graph{
    //method to add edges to the graph struct
    fn add_edges(&mut self, edges: &ListOfEdges){
        for (u,v) in edges{
            self.outedges[*u].push(*v);
            self.outedges[*v].push(*u);
        }
    }

    //method to search for and delete an edge in a graph struct
    fn remove_edge(&mut self, edge:(Vertex,Vertex)){
        for i in 0..self.outedges[edge.0].len(){
            if self.outedges[edge.0][i] == edge.1{
                self.outedges[edge.0].remove(i);
                break;
            }
        }

        for i in 0..self.outedges[edge.1].len(){
            if self.outedges[edge.1][i] == edge.0{
                self.outedges[edge.1].remove(i);
                break;
            }
        }
    }

    //method that sort the outedges vector 
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    //method to create a new graph
    fn create_graph(n:usize, edges: &ListOfEdges) ->Graph{
        let mut g = Graph{n, outedges:vec![vec![];n]};
        g.add_edges(edges);
        g.sort_graph_lists();
        return g
    }

    
}

//functions used for counting the times shortest paths pass through each edges
//first element in the tuple is the count, second number in the tuple is the edge
fn create_edges_count(edges: &ListOfEdges) -> Vec<(usize,(usize,usize))>{
    let mut v:Vec<(usize,(usize,usize))> = Vec::new();

    for i in 0..edges.len(){
        v.push((0,(edges[i].0,edges[i].1)))
    }
    return v
}

//update the counter of edge inbetweenness 
fn update_edges_count( mut counter: Vec<(usize,(usize,usize))>, edges_passed: Vec<(usize,usize)>)-> Vec<(usize,(usize,usize))>{
    for i in 0..edges_passed.len(){
        println!("{}" , i ); 
        for j in 0..counter.len(){
            if counter[j].1 == edges_passed[i]{
                counter[j].0 += 1;
            }

            if (counter[j].1.0 == edges_passed[i].1 && counter[j].1.1 == edges_passed[i].0){
                counter[j].0 +=1
            } 
        }
    }
    return counter
}

//count the edge in betweeness of edges
fn bfs_count(i:usize,graph:&Graph, counter: Vec<(usize,(usize,usize))>) -> Vec<(usize,(usize,usize))>{
    let start: Vertex = i;
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    let mut edges_passed: Vec<(usize,usize)> = Vec::new();
    distance[start] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front(){
        for u in graph.outedges[v].iter() {
            if let None = distance[*u]{
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
                edges_passed.push((v,*u))
            }
        }
    }
    
    return update_edges_count(counter, edges_passed)
}

//find the nodes that are 3 distances within the starting point i
//returns a vector of nodes that are in the community
fn bfs_community(i:usize, graph: &Graph) -> Vec<usize>{
    let start: Vertex = i;
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    let mut community:Vec<usize> = vec![start];
    while let Some(v) = queue.pop_front(){
        for u in graph.outedges[v].iter(){
            if let None = distance[*u]{
                distance[*u] = Some(distance[v].unwrap()+1);
                if distance[*u].unwrap() == 3{
                    break; 
                }
                community.push(*u);
                queue.push_back(*u)
            }
        }
    }
    return community;
}

//given a list of nodes and a vector of their features, count the number of each type
fn count_type(community:Vec<usize>, features: &Vec<(u32, String)>) -> Vec<(&str, i32)>{
    let mut counter = 
        vec![("tvshow", 0), ("government",0),("company",0),("politician",0)];
    for node in community{
        for (kind, count) in counter.iter_mut(){
            if features[node].1 == *kind{
                 *count +=1;
            }
        }
    }
    return counter
}

//unit tests 
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn graph_test(){
        let test_edges = vec![(0,3),(1,2), (1,3),(2,5),(3,4),(4,1), (5,4)];
        let mut g = Graph::create_graph(6,&test_edges);
        assert_eq!(g.outedges[1], vec![2,3,4]);
    }
    
    #[test]
    fn edge_deletion_test(){
        let test_edges = 
            vec![(0,3),(1,2), (1,3),(2,5),(3,4),(3,5),(4,1), (5,4)];
        let mut g = Graph::create_graph(6,&test_edges);
        g.remove_edge((3,4));
        assert_eq!(g.outedges[3], vec![0,1,5])
    }
}


fn main() {
    //read in the features file to record the type of each vertex 
    let features = read_csv_features("musae_facebook_target.csv"); 

    //read the edges file to record down as a vector of edges
    let edges = read_csv_edges("musae_facebook_edges.csv");
    
    // n is the number of nodes
    let n = features.len();


    //create a struct of graph
    let mut graph = Graph::create_graph(n, &edges);


    //iterate 300 times to remove 300 edges that are likely to be bridges between communities
    for iterations in 0..300{
        //for each iteration, remove the edge with the highest edge-in-betweenness
        let mut counter = create_edges_count(&edges);
        for i in 0..n{
            counter = bfs_count(i,&graph,counter)
        }

        // sort the counter in a descending order, making the largest counter to be at the top
        counter.sort_unstable_by(|a, b| b.cmp(a));
        println!("in iteration {}, the highest in-betweennesss edge is{:?} ", iterations, counter[0].1);
        graph.remove_edge(counter[0].1)
    }

    //choosing random points for community detection 
    use rand::Rng;
    for iterations in 0..20 {
        let node = rand::thread_rng().gen_range(0..n);
        let community = bfs_community(node, &graph);
        println!("In this iteration {} ,The community has {} unit", iterations,community.len());

        let counter = count_type(community, &features);
        println!("Their types are: {:?}" ,counter);
    }

}
