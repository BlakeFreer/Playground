use core::fmt;
use strum::{Display, EnumString};
use strum_macros::EnumIter;

#[derive(Debug, Clone)]
pub enum Health {
    Regular,
    Super,
    Powerup(Powerup),
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Health::Regular => {}
            Health::Super => write!(f, "Super")?,
            Health::Powerup(p) => write!(f, "{}", p.to_string())?,
        }
        Ok(())
    }
}

#[derive(Debug, EnumString, EnumIter, Display)]
pub enum Item {
    Mushroom,
    Snowflake,
    FireFlower,
    Feather,
}

#[derive(Debug, Clone, Display)]
pub enum Powerup {
    Ice,
    Fire,
    Cape,
}

pub fn item_to_health(item: &Item) -> Health {
    match item {
        Item::Mushroom => Health::Super,
        Item::Snowflake => Health::Powerup(Powerup::Ice),
        Item::FireFlower => Health::Powerup(Powerup::Fire),
        Item::Feather => Health::Powerup(Powerup::Cape),
    }
}

#[derive(Debug, EnumString, EnumIter, Display)]
pub enum Transition {
    Revive,
    Damage,
    #[strum(disabled)]
    GetItem(Item),
}

#[derive(Clone)]
pub enum State {
    Alive(Health),
    Dead,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Dead => write!(f, "Dead")?,
            State::Alive(health) => write!(f, "{}Mario", health)?,
        }
        Ok(())
    }
}

pub fn transition(state: &State, transition: Transition) -> State {
    // Determine if a new state is reached by this transition.
    match &state {
        State::Alive(health) => match &transition {
            Transition::Damage => match health {
                Health::Regular => State::Dead,
                Health::Super => State::Alive(Health::Regular),
                Health::Powerup(_) => State::Alive(Health::Super),
            },
            Transition::GetItem(item) => match (health, item) {
                (Health::Powerup(_), Item::Mushroom) => state.clone(),
                (_, item) => State::Alive(item_to_health(item)),
            },
            Transition::Revive => state.clone(),
        },
        State::Dead => match transition {
            Transition::Revive => State::Alive(Health::Regular),
            _ => State::Dead,
        },
    }
}
