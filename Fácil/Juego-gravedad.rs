fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut vector: Vec<u32> = cubes.to_vec();
    vector.sort();
    if dir == 'L' {
        vector.reverse();
    } 
    return vector;
}