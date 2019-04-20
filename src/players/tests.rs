
use crate::game::*;
use crate::players::*;

// returns an error if a player makes an illegal move
fn run_game(players: [&Player; 4]) -> Result<Team, String> {
    let mut g = Hokm::new(PlayerNumber::One);
    loop {
        match g.play(players) {
            GameEvent::Won(team) => return Ok(team),
            GameEvent::InvalidPlay(p, _) => return Err(format!("player number {:?} ({}) played illegally", p, players[p.as_index()].name())),
            _ => {}
        }
    }
}

#[test]
fn player_illegal_moves_1() {
    for _i in 0..100 {
        let r = run_game([
            &SensiblePlayer::new(),
            &SensiblePlayer::new(),
            &SensiblePlayer::new(),
            &SensiblePlayer::new(),
        ]);
        assert!(r.is_ok());
    }
}

#[test]
fn player_illegal_moves_2() {
    for _i in 0..100 {
        let r = run_game([
            &RandomPlayer,
            &RandomPlayer,
            &RandomPlayer,
            &SensiblePlayer::new(),
        ]);
        assert!(r.is_ok());
    }
}

#[test]
fn player_illegal_moves_3() {
    for _i in 0..100 {
        let r = run_game([
            &RandomPlayer,
            &RandomPlayer,
            &RandomPlayer,
            &RandomPlayer,
        ]);
        assert!(r.is_ok());
    }
}

#[test]
fn player_strength() {
    let mut sensible_wins = 0;
    let mut random_wins = 0;
    let n = 10000;
    for _i in 0..n {
        let r = run_game([
            &RandomPlayer,
            &SensiblePlayer::new(),
            &RandomPlayer,
            &SensiblePlayer::new(),
        ]);
        assert!(r.is_ok());
        match r.unwrap() {
            Team::PlayersTwoAndFour => sensible_wins += 1,
            Team::PlayersOneAndThree => random_wins += 1,
        }
    }
    println!("full {} to {}", sensible_wins, random_wins);
    assert!((sensible_wins as f64 / n as f64) > 0.7);
}

#[test]
fn player_strength_half_p2() {
    let mut sensible_wins = 0;
    let mut random_wins = 0;
    let n = 10000;
    for _i in 0..n {
        let r = run_game([
            &RandomPlayer,
            &SensiblePlayer::new(),
            &RandomPlayer,
            &RandomPlayer,
        ]);
        assert!(r.is_ok());
        match r.unwrap() {
            Team::PlayersTwoAndFour => sensible_wins += 1,
            Team::PlayersOneAndThree => random_wins += 1,
        }
    }
    println!("half_p2 {} to {}", sensible_wins, random_wins);
    assert!((sensible_wins as f64 / n as f64) > 0.57);
}

#[test]
fn player_strength_half_p4() {
    let mut sensible_wins = 0;
    let mut random_wins = 0;
    let n = 10000;
    for _i in 0..n {
        let r = run_game([
            &RandomPlayer,
            &RandomPlayer,
            &RandomPlayer,
            &SensiblePlayer::new(),
        ]);
        assert!(r.is_ok());
        match r.unwrap() {
            Team::PlayersTwoAndFour => sensible_wins += 1,
            Team::PlayersOneAndThree => random_wins += 1,
        }
    }
    println!("half_p4 {} to {}", sensible_wins, random_wins);
    assert!((sensible_wins as f64 / n as f64) > 0.59);
}

#[test]
fn player_strength_random() {
    let mut t1_wins = 0;
    let mut t2_wins = 0;
    let n = 10000;
    for _i in 0..n {
        let r = run_game([
            &RandomPlayer,
            &RandomPlayer,
            &RandomPlayer,
            &RandomPlayer,
        ]);
        assert!(r.is_ok());
        match r.unwrap() {
            Team::PlayersTwoAndFour => t1_wins += 1,
            Team::PlayersOneAndThree => t2_wins += 1,
        }
    }
    println!("random {} to {}", t1_wins, t2_wins);
    assert!((t1_wins as f64 / n as f64) > 0.48);
    assert!((t2_wins as f64 / n as f64) > 0.48);
}
