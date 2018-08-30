//graph class
struct node{
    name: char,
    connections: Vec<char>
}

impl node {
    fn add_connection(&mut self, conn: char) {
        self.connections.push(conn);
    }
}


fn main() {
    let mut nodeA = node {
        name: 'a',
        connections: Vec::new()
    };

    nodeA.add_connection('b');

    println!("node A {}", nodeA.name);
    for i in &nodeA.connections {
        println!("connections: {}", i);
    }
    println!("hello, world!");
}
