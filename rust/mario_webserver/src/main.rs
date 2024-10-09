
#[derive(Debug)]
enum Transition {
    Mushroom,
    Fire,
    Feather,
    Revive,
    Damage,
}

#[derive(Debug, Clone)]
enum Powerup {
    Super,
    Fire,
    Cape,
}

#[derive(Debug, Clone)]
enum State {
    Alive(Option<Powerup>),
    Dead,
}

fn transition(state: &mut State, transition: Transition) {
    // Determine if a new state is reached by this transition.
    let new_state = match &state {
        State::Alive(powerup) => match (powerup, &transition) {
            (powerup, Transition::Damage) => match powerup {
                None => State::Dead,
                Some(Powerup::Super) => State::Alive(None),
                Some(_) => State::Alive(Some(Powerup::Super))
            }
            (None, Transition::Mushroom) => State::Alive(Some(Powerup::Super)),
            (_, Transition::Fire) => State::Alive(Some(Powerup::Fire)),
            (_, Transition::Feather) => State::Alive(Some(Powerup::Cape)),
            _ => state.clone(),
        }
        State::Dead => match transition {
            Transition::Revive => State::Alive(None),
            _ => State::Dead,
        }
    };

    // Move to the new state.
    println!("{:?} + {:?} = {:?}", state, transition, new_state);
    *state = new_state;
}

fn main() {
    let mut state = State::Alive(None);
    transition(&mut state, Transition::Fire);
    transition(&mut state, Transition::Fire);
    transition(&mut state, Transition::Feather);
    transition(&mut state, Transition::Damage);
    transition(&mut state, Transition::Fire);
    transition(&mut state, Transition::Mushroom);
    transition(&mut state, Transition::Damage);
    transition(&mut state, Transition::Damage);
    transition(&mut state, Transition::Damage);
    transition(&mut state, Transition::Mushroom);
    transition(&mut state, Transition::Damage);
    transition(&mut state, Transition::Revive);
}
