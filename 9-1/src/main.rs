fn main() {
    let player_count: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let marble_count: usize = std::env::args().nth(2).unwrap().parse().unwrap();

    println!("{} players, {} marbles", player_count, marble_count);

    let mut remaining_marbles: Vec<usize> = (0..=marble_count).collect();
    let mut ring_marbles: Vec<usize> = Vec::with_capacity(marble_count);
    let mut players = vec![Vec::new(); player_count];
    let mut ring_marble_index = 0usize;

    for player_index in (0..player_count).cycle() {
        if remaining_marbles.is_empty() { break; }

        let marble = remaining_marbles.remove(0);

        if marble % 23 == 0 && marble != 0 {
            let mut removing_marble_index = ring_marble_index as isize - 9;
            while removing_marble_index < 0 {
                removing_marble_index += ring_marbles.len() as isize;
            }
            let removing_marble_index = removing_marble_index as usize;
            let removed_marble = ring_marbles.remove(removing_marble_index);

            players[player_index].push(marble);
            players[player_index].push(removed_marble);

            ring_marble_index = removing_marble_index;
        } else {
            ring_marbles.insert(ring_marble_index, marble);
        }
        ring_marble_index += 2;
        while ring_marble_index > ring_marbles.len() {
            ring_marble_index -= ring_marbles.len();
        }
    }

    let mut highest = (0, 0);
    for (index, score) in players.into_iter().enumerate().filter(|(_, s)| !s.is_empty()) {
        let sum: usize = score.iter().sum();
        if sum > highest.1 {
            highest = (index, sum);
        }
    }

    println!("Player {} won with {} points", highest.0, highest.1);
}
