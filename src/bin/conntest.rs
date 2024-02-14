use jlctl::device_manager::DeviceManager;
use jumperless_oxide::matrix_state::{State, Node};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashSet;

/// How many tests to run in a row?
const TEST_COUNT: usize = 20;

fn main() {
    DeviceManager::new(None).with_device(|device| {
        println!("Starting fuzztest");
        let mut success = 0;
        let mut failure = 0;

        for i in 0..TEST_COUNT {

            println!("Fuzztest #{i}");
            if fuzztest(device) {
                success += 1;
            } else {
                failure += 1;
            }
            sleep(Duration::from_secs(1));

            println!(" --------------------------------------\n")
        }

        println!("Fuzztest complete: {} successes, {} failures", success, failure);
        
        Ok(())
    }).expect("with_device");
}

fn chip_status_to_nets(chip_status: Vec<jlctl::types::ChipStatus>) -> Vec<(i8, Vec<Node>)> {
    let mut state = State::default();

    for cs in chip_status {
        state.set_status(
            cs.char.into(),
            cs.x_status,
            cs.y_status,
        );
    }

    state.extract_nets().into_iter().map(|(net, nodes)| (net, unique_nodes(nodes.into_iter()))).collect()
}

fn fuzztest(device: &mut jlctl::device::Device) -> bool {
    device.set_netlist(vec![]).expect("set empty netlist");
    sleep(Duration::from_millis(300));
    let mut netlist = device.netlist().expect("get empty netlist");

    let extra_net_count = 2..=3u8;
    let nodes_per_net = 2..=4usize;

    let mut used_nodes = HashSet::new();

    let mut rng = rand::thread_rng();

    // SKIP SPECIAL NETS FOR NOW
    // for net in netlist.iter_mut() {
    //     for _ in 0..rng.gen_range(nodes_per_net.clone()) {
    //         net.nodes.push(random_breadboard_node(&mut rng, &mut used_nodes));
    //     }
    // }

    for i in 0..rng.gen_range(extra_net_count) {
        let mut nodes = vec![];

        for _ in 0..rng.gen_range(nodes_per_net.clone()) {
            nodes.push(random_breadboard_node(&mut rng, &mut used_nodes));
        }

        let net = jlctl::types::Net {
            index: 8 + i,
            number: 8 + i,
            nodes,
            special: false,
            color: random_color(&mut rng),
            machine: true,
            name: format!("Net {}", 8 + i),
        };

        netlist.push(net);
    }

    device.set_netlist(netlist.clone()).expect("set generated netlist");

    let chip_status = device.chipstatus().expect("get chip status");

    let expected = convert_netlist(netlist.clone());
    let actual = chip_status_to_nets(chip_status.clone());

    if expected != actual {
        println!("Netlists don't match!\n");
        if expected.len() != actual.len() {
            println!("  Different number of nets: expected={}, actual={}", expected.len(), actual.len());
        }
        for net in expected {
            let actual_net = actual.iter().find(|n| n.0 == net.0);
            if let Some(actual_net) = actual_net {
                if net != *actual_net {
                    println!("  Net {} differs:", net.0);
                    println!("    Expected: {:?}", net.1);
                    println!("    Computed: {:?}", actual_net.1);
                }
            } else {
                println!("  Net {} is missing: {:?}", net.0, net.1);
            }
        }

        println!("\n  Here come the netlist & chip status as JSON:");
        println!("    Netlist: {}", serde_json::to_string(&netlist).unwrap());
        println!("    Chipstatus: {}", serde_json::to_string(&chip_status).unwrap());

        false
    } else {
        println!("  Success!");
        true
    }
}

fn random_breadboard_node<R: Rng>(rng: &mut R, used_nodes: &mut HashSet<jlctl::types::Node>) -> jlctl::types::Node {
    let node = jlctl::types::Node::Column(rng.gen_range(1..=60));
    if used_nodes.contains(&node) {
        random_breadboard_node(rng, used_nodes)
    } else {
        node
    }
}

fn random_color<R: Rng>(rng: &mut R) -> jlctl::types::Color {
    jlctl::types::Color([rng.gen(), rng.gen(), rng.gen()])
}

fn convert_netlist(netlist: Vec<jlctl::types::Net>) -> Vec<(i8, Vec<Node>)> {
    netlist.iter()
        .filter(|net| net.nodes.len() > 1) // ignore nets that have just one node (they are irrelevant)
        .map(|net| (net.index as i8, unique_nodes(net.nodes.iter().map(|node| (*node).into()))))
        .collect()
}

fn unique_nodes(nodes: impl Iterator<Item = Node>) -> Vec<Node> {
    let set: HashSet<Node> = nodes.collect();
    let mut nodes: Vec<Node> = set.into_iter().collect();
    nodes.sort();
    nodes
}
