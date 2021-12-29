#[derive(Clone, Debug)]
struct Player {
    id: usize,
    position: usize,
    score: usize,
}

impl Player {
    fn advance(&mut self, dice_total: usize) {
        self.position = (self.position - 1 + dice_total) % 10 + 1;
        self.score += self.position;
    }
}

trait Dice {
    fn roll(&mut self) -> usize;
    fn times_rolled(&self) -> usize;
}

#[derive(Clone, Debug)]
struct Deterministic100SidedDice {
    rolls: usize,
}

impl Deterministic100SidedDice {
    fn new() -> Deterministic100SidedDice {
        Deterministic100SidedDice{rolls: 0}
    }
}

impl Dice for Deterministic100SidedDice {
    fn roll(&mut self) -> usize {
        let value = (self.rolls % 100) + 1;
        self.rolls += 1;
        value
    }

    fn times_rolled(&self) -> usize {
        self.rolls
    }
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let mut players = vec![];
    for line in input.lines() {
        let space_separated: Vec<_> = line.split(" ").collect();
        assert_eq!(space_separated[0], "Player");
        assert_eq!(space_separated[2], "starting");
        assert_eq!(space_separated[3], "position:");
        let player_id: usize = space_separated[1].parse().unwrap();
        let starting_position: usize = space_separated[4].parse().unwrap();
        players.push(Player{
            id: player_id,
            position: starting_position,
            score: 0,
        });
    }
    assert_eq!(players.len(), 2);

    let mut dice = Deterministic100SidedDice::new();
    // for i in 0..500 {
    //     print!("{} ", dice.roll());
    // }
    // println!();
    // println!();
    // dice = Deterministic100SidedDice::new();

    let mut winner = None;
    while winner.is_none() {
        winner = play_one_turn(&mut players, &mut dice);
        println!("{:?}", dice);
        for player in &players {
            println!("{:?}", player);
        }
        println!();
    }

    let loser = players.into_iter().filter(|player| player.id != winner.unwrap()).next().unwrap();
    return dice.times_rolled() * loser.score;
}

fn play_one_turn(players: &mut Vec<Player>, dice: &mut impl Dice) -> Option<usize> {
    for player in players {
        let forward = dice.roll() + dice.roll() + dice.roll();
        player.position = (player.position - 1 + forward) % 10 + 1;
        player.score += player.position;
        if player.score >= 1000 {
            return Some(player.id);
        }
    }
    return None;
}

#[aoc(day21, part2)]
fn part2(input: &str) -> u64 {
    let mut players = vec![];
    for line in input.lines() {
        let space_separated: Vec<_> = line.split(" ").collect();
        assert_eq!(space_separated[0], "Player");
        assert_eq!(space_separated[2], "starting");
        assert_eq!(space_separated[3], "position:");
        let player_id: usize = space_separated[1].parse().unwrap();
        let starting_position: usize = space_separated[4].parse().unwrap();
        players.push(Player{
            id: player_id,
            position: starting_position,
            score: 0,
        });
    }
    assert_eq!(players.len(), 2);

    // what are all possible sums?
    // 3 (111)
    // 4 (112, 121, 211)
    // 5 (122, 212, 221, 113, 131, 311)
    // 6 (222, 123, 213, 321, 231, 132, 312)
    // 7 (133, 313, 331, 223, 232, 322)
    // 8 (233, 323, 332)
    // 9 (333)
    //
    // *** there are only 7 different values that can be added to 
    // a player's position each time
    // even though there's 27 universes underneath
    // this greatly simplifies the problem ***
    //
    // how many ways from position 7 to a score of 21?
    // round 1
    // 3 gives a score of 10
    // 4 a score of 1
    // 5 a score of 2
    // 6 a score of 3
    // 7 a score of 4
    // 8 a score of 5
    // 9 a score of 6
    // so the player is at a score of 6 minimum
    // round 2
    //
    // need to branch out for each possible score
    // function will return how many universes that thing happened
    //players[0].score = 20;
    // println!("player 0: {:?}", play_until_score_of_21(players[0].clone()));
    // println!("player 1: {:?}", play_until_score_of_21(players[1].clone()));

    // let p1 = play_until_score_of_21(players[0].clone()).universes_won_in_given_number_of_turns;
    // let p2 = play_until_score_of_21(players[1].clone()).universes_won_in_given_number_of_turns;

    // let total_number_of_universes: u64 = p1.iter().sum::<u64>() * p2.iter().sum::<u64>();
    // println!("total_number_of_universes: {}", total_number_of_universes);

    let (p0_winning_universes, p1_winning_universes) = find_dirac_winners(players[0].clone(), players[1].clone());
    println!("p0_winning_universes = {:?}", p0_winning_universes);
    println!("p1_winning_universes = {:?}", p1_winning_universes);

    if p0_winning_universes > p1_winning_universes {
        return p0_winning_universes;
    } else if p1_winning_universes > p0_winning_universes {
        return p1_winning_universes;
    } else {
        panic!("no overall winner");
    }
}

// #[derive(Clone, Debug)]
// struct PlayResult {
//     universes_won_in_given_number_of_turns: Vec<u64>,
// }

// fn play_until_score_of_21(player: Player) -> PlayResult {
//     if player.score >= 21 {
//         return PlayResult{
//             universes_won_in_given_number_of_turns: vec![1],
//         }
//     }

//     // 3 (111)
//     // 4 (112, 121, 211)
//     // 5 (122, 212, 221, 113, 131, 311)
//     // 6 (222, 123, 213, 321, 231, 132, 312)
//     // 7 (133, 313, 331, 223, 232, 322)
//     // 8 (233, 323, 332)
//     // 9 (333)
//     let possible_dice_totals = vec![
//         (3, 1),
//         (4, 3),
//         (5, 6),
//         (6, 7),
//         (7, 6),
//         (8, 3),
//         (9, 1),
//     ];

//     let mut play_result = PlayResult{
//         universes_won_in_given_number_of_turns: vec![0; 25],
//     };

//     for (dice_total, number_of_universes) in possible_dice_totals {
//         //println!("{:?}", play_result);
//         let mut subplayer = player.clone();
//         subplayer.advance(dice_total);
//         if subplayer.score >= 21 {
//             play_result.universes_won_in_given_number_of_turns[1] += number_of_universes;
//         } else {
//             let subresult = play_until_score_of_21(subplayer);
//             for (i, v) in subresult.universes_won_in_given_number_of_turns.into_iter().enumerate() {
//                 if i == 24 {
//                     if v > 0 {
//                     panic!("cant handle this (shouldnt happen)");
//                     }
//                     break;
//                 }
//                 play_result.universes_won_in_given_number_of_turns[i+1] += v * number_of_universes;
//             }
//         }
//     }
//     return play_result;
// }

fn find_dirac_winners(current_player: Player, other_player: Player) -> (u64, u64) {
    if current_player.score >= 21 || other_player.score >= 21 {
        panic!("should not reach here");
    }

    // 3 (111)
    // 4 (112, 121, 211)
    // 5 (122, 212, 221, 113, 131, 311)
    // 6 (222, 123, 213, 321, 231, 132, 312)
    // 7 (133, 313, 331, 223, 232, 322)
    // 8 (233, 323, 332)
    // 9 (333)
    let possible_dice_totals = vec![
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];

    let mut current_player_winning_universes = 0;
    let mut other_player_winning_universes = 0;
    for (dice_total, number_of_universes) in possible_dice_totals {
        let mut subplayer = current_player.clone();
        subplayer.advance(dice_total);
        if subplayer.score >= 21 {
            current_player_winning_universes += number_of_universes;
        } else {
            let (other, current) = find_dirac_winners(other_player.clone(), subplayer);
            current_player_winning_universes += number_of_universes * current;
            other_player_winning_universes += number_of_universes * other;
        }
    }
    return (current_player_winning_universes, other_player_winning_universes);
}