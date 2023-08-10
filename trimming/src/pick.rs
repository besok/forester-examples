use crate::{forester_builder, success, turn_on_logs, vis};
use forester_rs::runtime::action::keeper::ActionImpl;
use forester_rs::runtime::action::Action;
use forester_rs::runtime::args::RtArgs;
use forester_rs::runtime::rtree::builder::RtTreeBuilder;
use forester_rs::runtime::rtree::builder::*;
use forester_rs::runtime::rtree::rnode::FlowType;
use forester_rs::runtime::rtree::rnode::{RNode, RNodeName};
use forester_rs::runtime::trimmer::task::{RtTreeTrimTask, TrimTask};
use forester_rs::runtime::trimmer::{RequestBody, TreeSnapshot, TrimRequest};
use forester_rs::runtime::RtResult;
use forester_rs::simulator::actions::SimAction;
use forester_rs::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn pick_test() {
    turn_on_logs();

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("pick");

    let mut fb = forester_builder(&mut root);

    fb.register_action("pick", Action::sync(SimAction::Random(2000)));

    let mut forester = fb.build_with(|| success()).unwrap();

    vis(root.clone().join("before.svg"), &mut forester);

    forester.add_trim_task(TrimTask::rt_tree(Breaker));

    println!("{:?}", forester.run_until(Some(10)));

    vis(root.clone().join("after.svg"), &mut forester);
}

struct Breaker;

impl RtTreeTrimTask for Breaker {
    fn process(&self, snapshot: TreeSnapshot<'_>) -> RtResult<TrimRequest> {
        if snapshot.tick < 5 {
            Ok(TrimRequest::Skip)
        } else {
            let tree = snapshot.tree;
            if let Some(pick_id) = tree.analyze().find_by(|n| n.is_name("pick")) {
                let mut rtb = RtTreeBuilder::new_from(tree.max_id());

                let sub_tree = flow!(
                    r_sequence node_name!("complex_pick"), args!();
                        action!(node_name!("check_cond_pick")),
                        action!(node_name!("pick_impl"))
                );

                rtb.set_as_root(sub_tree, pick_id);

                let actions = HashMap::from_iter(vec![
                    (
                        "check_cond_pick".to_string(),
                        Action::sync(SimAction::Random(100)),
                    ),
                    (
                        "pick_impl".to_string(),
                        Action::sync(SimAction::Success(2000)),
                    ),
                ]);
                Ok(TrimRequest::attempt(RequestBody::new(rtb, actions)))
            } else {
                Ok(TrimRequest::Reject)
            }
        }
    }
}
