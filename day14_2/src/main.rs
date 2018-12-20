fn search(recipes: &Vec<usize>, target: &Vec<usize>) -> bool {
    let mut found = false;

    if recipes.len() > target.len() {
        found = true;
        for i in 0..target.len() {
            if recipes[recipes.len() - target.len() + i] != target[i] {
                found = false;
                break;
            }
        }
    }
    found
}

fn main() {
    let mut recipes: Vec<usize> = vec![3, 7];
    let mut first_idx = 0;
    let mut second_idx = 1;
    let mut num_of_recipes = 2;
    //let target: Vec<usize> = vec![5, 1, 5, 8, 9];
    //let target: Vec<usize> = vec![0, 1, 2, 4, 5];
    //let target: Vec<usize> = vec![5, 9, 4, 1, 4];
    //let target: Vec<usize> = vec![9, 2, 5, 1, 0];
    let target: Vec<usize> = vec![5, 5, 6, 0, 6, 1];

    'outer: loop {
        let sum = recipes[first_idx] + recipes[second_idx];
        if sum / 10 > 0 {
            recipes.push(sum / 10);
            num_of_recipes += 1;
            if search(&recipes, &target) {break;}
        }
        recipes.push(sum % 10);
        num_of_recipes += 1;
        if search(&recipes, &target) {break;}

        first_idx += (1 + recipes[first_idx]) % num_of_recipes;
        first_idx %= num_of_recipes;
        second_idx += (1 + recipes[second_idx]) % num_of_recipes;
        second_idx %= num_of_recipes;

/*
        for i in 0..recipes.len() {
            if i == first_idx {
                print!("({})", recipes[i]);
            } else if i == second_idx {
                print!("[{}]", recipes[i]);
            } else {
                print!(" {} ", recipes[i]);
            }
        }
        println!("");
*/
    }

    println!("{}", num_of_recipes - target.len());
}
