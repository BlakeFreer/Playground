use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
enum Item {
    Mushroom,
    FireFlower,
    Feather,
}

#[derive(Debug, Clone)]
enum Powerup {
    Super,
    Fire,
    Cape,
}

fn item_to_powerup(item: &Item) -> Powerup {
    match item {
        Item::Mushroom => Powerup::Super,
        Item::FireFlower => Powerup::Fire,
        Item::Feather => Powerup::Cape
    }
}

#[derive(Debug)]
enum Transition {
    Revive,
    Damage,
    GetItem(Item)
}

#[derive(Clone)]
enum State {
    Alive(Option<Powerup>),
    Dead,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        match self {
            State::Dead => write!(f, "Dead")?,
            State::Alive(powerup) => {
                if let Some(powerup) = powerup {
                    write!(f, "{:?}", powerup)?
                }
                write!(f, "Mario")?
            }
        }
        Ok(())
    }
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
    println!("{} + {:?} = {}", state, transition, new_state);
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
