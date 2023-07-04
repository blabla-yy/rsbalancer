use rsbalancer::{BalancerEnum, Node};

fn main() {
    let mut balancer = rsbalancer::new(BalancerEnum::RR, vec![
        Node::new_with_default_weight(1),
        Node::new_with_default_weight(2),
        Node::new_with_default_weight(3),
    ]);

    for _ in 0..10 {
        println!("{}", balancer.next_id().unwrap());
    }
}
