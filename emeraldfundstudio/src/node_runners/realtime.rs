use dashmap::DashMap;
use egui_snarl::{InPinId, NodeId, Snarl};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::node_editor::node_trait::{CheapCloneNodeDataTypeWithValue, EFNodeFNSerialized};

pub static NODE_COMPUTE_CACHE: Lazy<DashMap<usize, Vec<CheapCloneNodeDataTypeWithValue>>> =
    Lazy::new(|| Default::default());

pub fn filter_already_executed(nodes: &mut Vec<NodeId>) {
    nodes.retain(|n| !NODE_COMPUTE_CACHE.contains_key(&n.0));
}

pub fn clear_cache_from_node_onward(snarl: &Snarl<EFNodeFNSerialized<'_>>, id: &NodeId) {
    // TODO: Need to remove only it and dependencies
    NODE_COMPUTE_CACHE.clear();
    run_nodes(snarl);
}

pub fn is_node_realtime_executable(
    snarl: &Snarl<EFNodeFNSerialized<'_>>,
    id: NodeId,
    node: &EFNodeFNSerialized<'_>,
) -> bool {
    let n_inputs = node.get_node().get_inputs().len();

    let has_all_inputs_connected = (0..n_inputs).into_par_iter().all(|pin_id| {
        let in_pin = snarl.in_pin(InPinId {
            node: id,
            input: pin_id,
        });

        let has_at_least_one_input_connection = in_pin.remotes.len() > 0;
        if !has_at_least_one_input_connection {
            return false;
        }
        let has_computed_result = NODE_COMPUTE_CACHE
            .get(&in_pin.remotes.first().unwrap().node.0)
            .is_some();
        if has_computed_result {
            return true;
        }
        false
    });

    has_all_inputs_connected
}

fn get_executable_nodes(snarl: &Snarl<EFNodeFNSerialized<'_>>) -> Vec<NodeId> {
    snarl
        .node_ids()
        .filter_map(|(id, node)| {
            if is_node_realtime_executable(snarl, id, node) {
                Some(id)
            } else {
                None
            }
        })
        .collect()
}

pub fn run_nodes(snarl: &Snarl<EFNodeFNSerialized<'_>>) {
    let mut executable_nodes = get_executable_nodes(snarl);
    filter_already_executed(&mut executable_nodes);
    while executable_nodes.len() > 0 {
        executable_nodes.par_iter().for_each(|id| {
            let node = snarl.get_node(*id).unwrap();
            let inner_node = node.get_node();
            let n_inputs = node.get_node().get_inputs().len();
            let input_args = (0..n_inputs)
                .into_par_iter()
                .map(|pin_id| {
                    let in_pin = snarl.in_pin(InPinId {
                        node: *id,
                        input: pin_id,
                    });

                    let remote_output_pin = in_pin.remotes.first().unwrap();
                    let output_values = NODE_COMPUTE_CACHE.get(&remote_output_pin.node.0).unwrap();
                    output_values.get(remote_output_pin.output).unwrap().clone()
                })
                .collect::<Vec<CheapCloneNodeDataTypeWithValue>>();
            let results = inner_node.process_data(&input_args).unwrap();
            NODE_COMPUTE_CACHE.insert(id.0, results);
        });
        executable_nodes = get_executable_nodes(snarl);
        filter_already_executed(&mut executable_nodes);
    }
}
