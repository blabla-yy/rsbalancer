## rsbalancer

A rust library that implements load balancing algorithms.

- round robin
- weighted round robin(like nginx)
- random
- consistent hashing

### Installation
```shell
cargo add rsbalancer
```

### Usage

### Weighted round robin
```rust
use rsbalancer::{BalancerEnum, Node, Balancer};

fn main() {
    let mut balancer = rsbalancer::weighted_round_robin(vec![
        Node::new("ip1", 1), // ip、weight
        Node::new("ip2", 1),
        Node::new("ip3", 2),
    ]);

    for _ in 0..10 {
        println!("{}", balancer.next_id().unwrap());
    }
}
```

### Consistent hashing
```rust
use rsbalancer::Node;

fn main() {
    // number of virtual nodes = node.weight * replicas
    let balancer = rsbalancer::consistent_hashing(
        vec![
            Node::new("ip1".to_string(), 1), // ip、weight
            Node::new("ip2".to_string(), 1),
            Node::new("ip3".to_string(), 1),
        ],
        160, //replicas
    );

    for random_ip in 0..10 {
        println!(
            "{} == {}",
            balancer
                .get_matching_node(random_ip.to_string())
                .unwrap()
                .get_id(),
            balancer
                .get_matching_node(random_ip.to_string())
                .unwrap()
                .get_id()
        );
    }
}
```