use crate::{forester_builder, success, turn_on_logs, vis};
use forester_rs::runtime::action::{Action, Impl, Tick};
use forester_rs::runtime::args::RtArgs;
use forester_rs::runtime::context::TreeContextRef;
use forester_rs::runtime::rtree::builder::RtTreeBuilder;
use forester_rs::runtime::rtree::builder::*;
use forester_rs::runtime::rtree::rnode::FlowType;
use forester_rs::runtime::rtree::rnode::{RNode, RNodeName};
use forester_rs::runtime::trimmer::task::{RtTreeTrimTask, TrimTask};
use forester_rs::runtime::trimmer::{RequestBody, TreeSnapshot, TrimRequest};
use forester_rs::runtime::{RtResult, RuntimeError, TickResult};
use forester_rs::simulator::actions::SimAction;
use forester_rs::*;
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn queue_test() {
    turn_on_logs();

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("queue");

    let mut fb = forester_builder(&mut root);

    fb.register_action("env", Action::sync(Prep));
    fb.register_action("exec", Action::sync(SimAction::Random(1000)));
    let mut forester = fb.build_with(|| success()).unwrap();
    vis(root.clone().join("before.svg"), &mut forester);

    forester.add_trim_task(TrimTask::rt_tree(Reorder));

    println!("{:?}", forester.run());

    vis(root.clone().join("after.svg"), &mut forester);
}

struct Prep;
impl Impl for Prep {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let shift = args
            .first()
            .and_then(|v| v.as_int())
            .ok_or(RuntimeError::fail("unexpected".to_string()));

        let duration: u64 = (6 - shift?).try_into().unwrap();
        let _ = std::thread::sleep(Duration::from_secs(duration));

        if rand::random() {
            Ok(TickResult::success())
        } else {
            Ok(TickResult::failure("Tail".to_string()))
        }
    }
}

struct Reorder;

impl RtTreeTrimTask for Reorder {
    fn process(&self, snapshot: TreeSnapshot<'_>) -> RtResult<TrimRequest> {
        if snapshot.tick < 5 {
            Ok(TrimRequest::Skip)
        } else {
            let tree = snapshot.tree;

            let node = tree
                .analyze()
                .find_by(|node| node.is_flow(&FlowType::RFallback));

            if let Some(id) = node {
                let node = tree.node(&id).expect("the node should be there");
                let elems = match node {
                    RNode::Flow(_, _, _, children) => {
                        let mut children = children.clone();
                        children.reverse();
                        Ok(children)
                    }
                    _ => Err(RuntimeError::fail("recover from unexpected".to_string())),
                };
                let mut rtb = RtTreeBuilder::new();
                rtb.set_as_root(flow!(r_fallback node_name!(), args!(), elems?), id);

                Ok(TrimRequest::attempt(RequestBody::new(
                    rtb,
                    Default::default(),
                )))
            } else {
                Ok(TrimRequest::Reject)
            }
        }
    }
}
