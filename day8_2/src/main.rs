use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day8_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let tt: Vec<&str> = contents.lines().next().unwrap().split(' ').collect();
    let tree: Vec<i32> = tt.iter().map(|x| x.parse::<i32>().unwrap()).collect();

/*
    for n in tree {
        print!("{} ", n);
    }
*/

    let (_, value) = parse_node(&tree, 0);

    println!("value = {}", value);
}

// return (node size, value of this node)
fn parse_node(tree: &Vec<i32>, index: usize) -> (usize, i32) {
    let num_of_subnode: usize = tree[index] as usize;
    let num_of_metadata = tree[index + 1];
    let mut subnode_values = Vec::new();
    let mut subnode_index = index + 2;
    let mut value = 0;

    println!("parsing node starting from index {}", index);
    println!("    num of subnodes: {}  num of metadata: {}", num_of_subnode, num_of_metadata);

    for i in 0..num_of_subnode {
        let (size, value) = parse_node(tree, subnode_index);
        println!("    value of subnode[{}]: {}", i, value);
        subnode_values.push(value);
        subnode_index += size;
    }

    if num_of_subnode != 0 {
        println!("    has sub nodes");
        for i in 0..num_of_metadata {
            let metadata: usize = tree[subnode_index] as usize;
            println!("    metadata[{}]: {}", i, metadata);
            if metadata > 0 && metadata <= num_of_subnode {
                println!("        in range");
                value += subnode_values[metadata - 1];
            } else {
                println!("        out of range");
            }
            subnode_index += 1;
        }
    } else {
        println!("    no sub nodes");
        for i in 0..num_of_metadata {
            value += tree[subnode_index];
            subnode_index += 1;
        }
    }

    (subnode_index - index, value)
}
