use std::error::Error;
use csv;
use csv::StringRecord;


pub fn read_csv_features(filepath: &str) -> Vec<(u32, String)>{
    // Open file
    let file = std::fs::File::open(filepath).unwrap();
    let mut rdr = csv::ReaderBuilder::new().from_reader(file);

    let mut pages:Vec<(u32, String)> = Vec::new();
    for result in rdr.records().into_iter() {
        let record = result.unwrap();
        pages.push((record[0].parse().unwrap(),record[1].to_string()));
    }
    return pages;
}


type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

pub fn read_csv_edges(filepath: &str) -> ListOfEdges {
    let file = std::fs::File::open(filepath).unwrap();
    let mut rdr = csv::ReaderBuilder::new().from_reader(file);
    
    let mut edges:ListOfEdges = Vec::new();
    
    for result in rdr.records().into_iter() {
        let record = result.unwrap();
        edges.push((record[0].parse().unwrap(),record[1].parse().unwrap()));
    }
    return edges;
}
