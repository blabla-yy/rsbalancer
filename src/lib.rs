use std::error::Error;
use std::fmt;
use std::hash::Hash;

use crate::random::Random;
use crate::round_robin::RoundRobin;
use crate::weighted_round_robin::WeightedRoundRobin;

mod round_robin;
mod random;
mod weighted_round_robin;


pub trait Balancer<T: Hash> {
    fn add_node(&mut self, node: Node<T>);
    fn next(&mut self) -> Option<&Node<T>>;
    fn next_id(&mut self) -> Option<&T> {
        self.next().map(|n| &n.id)
    }
}

pub struct Node<T: Hash> {
    id: T,
    weight: i32,
}

impl<T: Hash> Node<T> {
    pub fn new_with_default_weight(id: T) -> Node<T> {
        Node {
            id,
            weight: 1,
        }
    }

    pub fn new(id: T, weight: i32) -> Result<Node<T>, ParameterError> {
        if weight <= 0 {
            return Err(ParameterError::new("weight <= 0"));
        }
        Ok(Node {
            id,
            weight,
        })
    }
}

pub enum BalanceType {
    RR,
    WRR,
    Random,
}

pub fn new<'a, T: Hash + 'a>(balance_type: BalanceType, nodes: Vec<Node<T>>) -> Box<dyn Balancer<T> + 'a> {
    match balance_type {
        BalanceType::RR => {
            Box::new(RoundRobin::new(nodes))
        }
        BalanceType::WRR => {
            Box::new(WeightedRoundRobin::new(nodes))
        }
        BalanceType::Random => {
            Box::new(Random::new(nodes))
        }
    }
}

#[derive(Debug)]
pub struct ParameterError {
    message: String,
}

impl ParameterError {
    fn new(message: &str) -> ParameterError {
        ParameterError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParameterError: {}", self.message)
    }
}

impl Error for ParameterError {
    fn description(&self) -> &str {
        &self.message
    }
}