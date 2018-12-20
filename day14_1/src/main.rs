fn main() {
    let mut recipes: Vec<usize> = vec![3, 7];
    let mut first_idx = 0;
    let mut second_idx = 1;
    let mut num_of_recipes = 2;
    //let target: usize = 9;
    //let target: usize = 5;
    //let target: usize = 18;
    //let target: usize = 2018;
    let target: usize = 556061;

    while num_of_recipes < target + 10 {
        let sum = recipes[first_idx] + recipes[second_idx];
        if sum / 10 > 0 {
            recipes.push(sum / 10);
            num_of_recipes += 1;
        }
        recipes.push(sum % 10);
        num_of_recipes += 1;

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

    for i in target..target + 10 {
        print!("{}", recipes[i]);
    }
    println!("");

}
