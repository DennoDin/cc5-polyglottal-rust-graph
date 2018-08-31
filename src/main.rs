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

// fn shortest_path<'a>(graph: &Vec<Node>, start: char, finish: char, steps: &'a i8, result: &'a mut i8){
//     // let mut i8 result = 0;
//     let mut temp_graph = Vec::new();
//     let mut exists: bool = false;
//     let mut end: bool = false;
//     for i in graph {
//         match i.name {
//             start => exists = true,
//             finish => end = true,
//             _ => {
//                 let mut node = node_builder(i.name);
//                 for connects in &i.connections {
//                     node.add_connection(*connects);
//                 }
//                 temp_graph.push(node);
//             }

//         }
//     }
//     if exists && !end {
//         for i in graph {
//             if i.name != start {
//                 steps = &(steps + 1);
//                 shortest_path(&temp_graph, i.name, finish, steps, result);
//                 steps = &(steps - 1);
//             }
//         }
//     }
//     if end {
//         if steps < result {
//             result = &(steps);
//         }
//     }
// }

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

    //graph printing

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


    //distance calculation

    let mut result: i8 = 32;
    let mut steps: i8 = 0;

    println!("Result: {}", result);
    println!("\n\nExiting Main");
}
