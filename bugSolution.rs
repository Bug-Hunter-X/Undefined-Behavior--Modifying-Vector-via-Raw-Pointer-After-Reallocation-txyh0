fn main() {
    let mut v = vec![1, 2, 3];
    // Safe alternative: use indexing to modify vector elements
    v[0] = 10; 
    println!("v: {:?}", v);
} 