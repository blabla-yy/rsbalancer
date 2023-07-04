use std::hash::Hash;

use crate::errors::{DuplicatedKeyError, NotFoundError};
use crate::random::Random;
use crate::round_robin::RoundRobin;
use crate::weighted_round_robin::WeightedRoundRobin;

mod round_robin;
mod random;
mod weighted_round_robin;
mod nodes;
mod errors;


pub trait Balancer<T: Hash + Eq + Copy> {
    fn add_node(&mut self, node: Node<T>) -> Result<(), DuplicatedKeyError>;
    fn remove_node(&mut self, id: &T) -> Result<(), NotFoundError>;
    fn contains_id(&mut self, id: &T) -> bool;
    fn get_node(&self, id: &T) -> Option<&Node<T>>;
    fn get_nodes(&self) -> Vec<&Node<T>>;

    fn set_down(&mut self, id: &T, down: bool) -> Result<(), NotFoundError>;
    fn next(&mut self) -> Option<&Node<T>>;
    fn next_id(&mut self) -> Option<&T> {
        self.next().map(|n| &n.id)
    }
}

pub struct Node<T: Hash + Eq + Copy> {
    id: T,
    weight: i32,
    down: bool,
    current_weight: i32,
    effective_weight: i32,
}

impl<T: Hash + Eq + Copy> Node<T> {
    pub fn new_with_default_weight(id: T) -> Node<T> {
        Node {
            id,
            weight: 1,
            down: false,
            current_weight: 0,
            effective_weight: 1,
        }
    }

    pub fn new(id: T, weight: i32) -> Result<Node<T>, errors::ParameterError> {
        if weight <= 0 {
            return Err(errors::ParameterError::new("weight <= 0"));
        }
        Ok(Node {
            id,
            weight,
            down: false,
            current_weight: 0,
            effective_weight: weight,
        })
    }

    pub fn get_id(&self) -> T {
        self.id
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }

    pub fn is_down(&self) -> bool {
        self.down
    }
}

pub enum BalancerEnum {
    RR,
    WRR,
    Random,
}

pub fn new<'a, T: Hash + Eq + Copy + 'a>(balancer_enum: BalancerEnum, nodes: Vec<Node<T>>) -> Box<dyn Balancer<T> + 'a> {
    match balancer_enum {
        BalancerEnum::RR => {
            Box::new(RoundRobin::new(nodes))
        }
        BalancerEnum::WRR => {
            Box::new(WeightedRoundRobin::new(nodes))
        }
        BalancerEnum::Random => {
            Box::new(Random::new(nodes))
        }
    }
}

