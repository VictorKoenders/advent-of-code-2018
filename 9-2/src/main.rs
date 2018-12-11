use std::collections::VecDeque;

fn main() {
    let player_count: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let marble_count: usize = std::env::args().nth(2).unwrap().parse().unwrap();

    println!("{} players, {} marbles", player_count, marble_count);
    println!("{}", calculate_high_score(player_count, marble_count));
}

fn calculate_high_score(_player_nb: usize, _marble_number: usize) -> usize {
    let mut players_score = vec![0; _player_nb];
    let mut ring = VecDeque::new();
    ring.push_front(0);

    for marble in 1.._marble_number {
        if marble % 23 == 0 {
            (0..7).for_each(|_| {
                let tmp = ring.pop_back().expect("Rotate problem");
                ring.push_front(tmp);
            });
            players_score[marble % _player_nb] +=
                marble + ring.pop_front().expect("No value in the ring");
        } else {
            (0..2).for_each(|_| {
                let tmp = ring.pop_front().expect("Rotate problem");
                ring.push_back(tmp);
            });
            ring.push_front(marble);
        }
    }
    *players_score
        .iter()
        .max()
        .expect("No value in the player scores")
}
