use rsbalancer::Node;

fn main() {
    // number of virtual nodes = node.weight * replicas
    let balancer = rsbalancer::consistent_hashing(
        vec![
            Node::new("ip1".to_string(), 1), // weight
            Node::new("ip2".to_string(), 1),
            Node::new("ip3".to_string(), 1),
        ],
        160, //replicas
    );

    for random_ip in 0..10 {
        println!(
            "{} == {}",
            balancer
                .get_matching_node_id(&random_ip.to_string())
                .unwrap(),
            balancer
                .get_matching_node(&random_ip.to_string())
                .unwrap()
                .get_id()
        );
    }
}
