/////////////////
// node struct //
/////////////////

struct Node{
    name: char,
    connections: Vec<char>
}

impl Node {
    fn add_connection(&mut self, conn: char) {
        self.connections.push(conn);
    }
}

fn node_builder(name: char) -> Node{
    Node {
        name,
        connections: Vec::new()
    }
}

/////////////////
////// main /////
/////////////////

fn main() {
    //graph setup
    let mut graph = Vec::new();

    let mut node_a = node_builder('a');
    let mut node_b = node_builder('b');
    let mut node_c = node_builder('c');
    node_a.add_connection('b');
    node_b.add_connection('a');
    node_b.add_connection('c');
    node_c.add_connection('b');
    graph.push(node_a);
    graph.push(node_b);
    graph.push(node_c);

    let mut index_i = 0;
    let mut index_j = 0;

    for i in &graph {
        
        println!("{}: \nnode: {}",index_i, i.name);
        println!("connections:");
        for j in &i.connections {
            println!("  {}: {}",index_j, j);
            index_j = index_j + 1
        }
        index_j = 0;
        index_i = index_i + 1; 
    }
    println!("\n\nExiting Main");
}
