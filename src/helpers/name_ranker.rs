pub fn string_sumularity_ranker(animixplay: Vec<&str>, gogo: &str) -> (usize, String) {
    let mut score = 0;
    let mut index = 0;
    for a in &animixplay {
        let mut temp_score = 0;
        // compare the each first letter of the strings and then the second and so on without unwrapping
        for (_i, (a, b)) in a.chars().zip(gogo.chars()).enumerate() {
            if a == b {
                temp_score += 1;
            } else {
                break;
            }
        }
        if temp_score > score {
            score = temp_score;
            index = animixplay.iter().position(|&x| &x == a).unwrap();
        }
    }
    (index, gogo.to_string()) //
}
