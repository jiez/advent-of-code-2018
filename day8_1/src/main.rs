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

    let (_, sum) = parse_node(&tree, 0);

    println!("sum = {}", sum);
}

// return (node size, sum of metadata)
fn parse_node(tree: &Vec<i32>, index: usize) -> (usize, i32) {
    let num_of_subnode = tree[index];
    let num_of_metadata = tree[index + 1];
    let mut sum = 0;
    let mut subnode_index = index + 2;

    for _i in 0..num_of_subnode {
        let (size, subsum) = parse_node(tree, subnode_index);
        sum += subsum;
        subnode_index += size;
    }

    for _i in 0..num_of_metadata {
        sum += tree[subnode_index];
        subnode_index += 1;
    }

    (subnode_index - index, sum)
}
