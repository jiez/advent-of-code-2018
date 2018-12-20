
fn main() {
    let player_num = 478;
    let last_marble = 71240;
    
    let mut circle = vec![0; 1];
    let mut current_index = 0;
    let mut scores: Vec<u64> = vec![0; player_num];
    let mut marble = 1;

    'outer: loop {
        for player in 0..player_num {
            // place marble
            if marble % 23 == 0 {
                scores[player] += marble;
                // the marble 7 marbles counter-clockwise from the current one
                if current_index >= 7 {
                    current_index -= 7;
                } else {
                    current_index = circle.len() - (7 - current_index);
                }
                // take it out of circle
                scores[player] += circle.remove(current_index);
                if current_index == circle.len() {
                    current_index = 0;
                }
            } else {
                // insert between 1 and 2 marbles cloclwisze of the current one
                if current_index == circle.len() - 1 {
                    circle.insert(1, marble);
                    current_index = 1;
                } else {
                    circle.insert(current_index + 2, marble);
                    current_index += 2;
                }
            }
/*
            print!("[{}]", player + 1);
            for i in 0..circle.len() {
                if i == current_index {
                    print!(" ({})", circle[i]);
                } else {
                    print!("  {} ", circle[i]);
                }
            }
            println!("");
*/

            if marble == last_marble {
                break 'outer;
            } else {
                marble += 1;
            }

        }


    }

    let mut high_score: u64 = 0;
    for i in 0..player_num {
        if scores[i] != 0 {
            println!("[{}] {}", i, scores[i]);
        }
        if high_score < scores[i] {
            high_score = scores[i];
        }
    }
    
    println!("{} players; last marble {}: high score is {}", player_num, last_marble, high_score);
}


