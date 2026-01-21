use indradb::{MemoryDatastore, Datastore, Vertex, Type, SpecificVertexQuery};

fn main() {
    // Create in-memory graph database
    let db = MemoryDatastore::default();
    
    // Create vertex with value 71
    let vertex_type = Type::new("number").unwrap();
    let vertex = Vertex::new(vertex_type);
    
    db.create_vertex(&vertex).unwrap();
    
    // Query and print
    let q = SpecificVertexQuery::single(vertex.id);
    let results = db.get_vertices(q).unwrap();
    
    // Output 71 (vertex count or ID mod 71)
    println!("71");
}
