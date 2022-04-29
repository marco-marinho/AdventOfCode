fn main(){
    let p1 = get_turns_deterministic(5 , 1);
    let p2 = get_turns_deterministic(10, 4);
    let val = calc_t1(&p1, &p2);
    println!("Task 1: {}", val);
}

fn calc_t1(p1: &[i64], p2: &[i64]) -> i64{
    let len1 = p1.len();
    let len2 = p2.len();
    if len1 < len2 {
        let lossing_score = p2[len1 - 2];
        let rolls = ((len1 * 2) - 1) * 3;
        lossing_score * rolls as i64
    }
    else {
        let lossing_score = p1[len2 - 1];
        let rolls = len2 * 2 * 3;
        lossing_score * rolls as i64
    }

}

fn get_turns_deterministic(start_pos: i64, start_die: i64) -> Vec<i64>{
    let mut output = Vec::new();
    let mut score = 0;
    let mut die = start_die - 6;
    let mut cur_pos = start_pos;
    while score < 1000 {
        die += 6;
        let steps = (die * 3) + 3;
        cur_pos = (cur_pos + steps) % 10;
        if cur_pos == 0 {cur_pos = 10;}
        score += cur_pos;
        output.push(score);
    }
    output
}