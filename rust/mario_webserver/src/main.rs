
#[derive(Debug)]
enum Transition {
    Mushroom,
    Fire,
    Feather,
    Revive,
    Damage,
}

enum Powerup {
    Super,
    Fire,
    Cape,
}

enum State {
    Alive(Option<Powerup>),
    Dead,
}

fn transition(state: &mut State, transition: Transition) {
    match state {
        State::Alive(powerup) => match (powerup, transition) {
            (None, Transition::Mushroom) => *state = State::Alive(Some(Powerup::Super)),
            (None, Transition::Damage) => *state = State::Dead,
            (_, Transition::Fire) => *state = State::Alive(Some(Powerup::Fire)),
            (_, Transition::Feather) => *state = State::Alive(Some(Powerup::Cape)),
            (Some(Powerup::Fire) | Some(Powerup::Cape), Transition::Damage) => *state = State::Alive(Some(Powerup::Super)),
            (Some(Powerup::Super), Transition::Damage) => *state = State::Alive(None),
            (_, Transition::Revive) => {},
            (Some(_), Transition::Mushroom) => {},
        }
        State::Dead => match transition {
            Transition::Revive => *state = State::Alive(None),
            _ => *state = State::Dead,
        }
    }
}

fn main() {
    let mut state = State::Alive(None);
    transition(&mut state, Transition::Fire);
    println!("Hello, world!");
}
