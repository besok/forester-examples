use std::path::PathBuf;
use forester_rs::runtime::action::{Impl, Tick};
use forester_rs::runtime::args::{RtArgs, RtValue};
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime::context::TreeContextRef;
use forester_rs::runtime::{RuntimeError, TickResult, to_fail};
use forester_rs::tracer::{Tracer, TracerConfig};


fn main() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().to_path_buf();
    root.push("example1dtext");
    let mut fb = ForesterBuilder::from_fs();
    fb.main_file("robot_example.tree".to_string());
    fb.root(root.clone());

    fb.register_sync_action("pick", Pick);
    fb.register_sync_action("place", Place);
    fb.register_sync_action("move", Move);
    fb.register_sync_action("is_arrived", ArrivedCheck);
    fb.register_sync_action("define_direction", DefineDir);

    fb.tracer(tracer());
    let mut forester = fb.build().unwrap();
    let res = forester.run();

    println!("{:?}", res);
}

fn tracer() -> Tracer {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("");
    root.push("tracer.log");
    Tracer::create(TracerConfig::in_file(root, None)).unwrap()
}

struct Pick;

impl Impl for Pick {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        ctx.trace("Pick load!".to_string())?;
        println!("Pick load!");
        Ok(TickResult::success())
    }
}

struct Place;

impl Impl for Place {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        ctx.trace("Place load!".to_string())?;
        println!("Place load!");
        Ok(TickResult::success())
    }
}

struct Move;

impl Impl for Move {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let b = ctx.bb();
        let mut bb = b.lock()?;
        let current = bb
            .get("curr_coord".to_owned())?
            .and_then(|v| v.clone().as_int())
            .ok_or(RuntimeError::fail("current is absent".to_owned()))?;

        let direction = bb
            .get("direction".to_owned())?
            .and_then(|v| v.clone().as_int())
            .ok_or(RuntimeError::fail("current is absent".to_owned()))?;

        let step = if direction > 0 { 1 } else if direction < 0 { -1 } else { 0 };
        let next = current + step;
        let _ = bb.put("curr_coord".to_owned(), RtValue::int(next))?;
        ctx.trace(format!("move on one step from {current} to {next}"))?;
        println!("move on one step from {current} to {next}");
        Ok(TickResult::success())
    }
}

struct ArrivedCheck;

impl Impl for ArrivedCheck {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let current = current(&ctx)?;
        let target = target(args, &ctx)?;
        ctx.trace(format!("test if current {current} and target {target} are equal"))?;
        println!("test if current {current} and target {target} are equal");
        if current == target {
            Ok(TickResult::success())
        } else {
            Ok(TickResult::failure("target and current are not eq".to_owned()))
        }
    }
}

struct DefineDir;

impl Impl for DefineDir {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let current = current(&ctx)?;
        let target = target(args, &ctx)?;

        if current == target {
            ctx.bb().lock()?.put("direction".to_string(), RtValue::int(0))?;
        } else if current > target {
            ctx.bb().lock()?.put("direction".to_string(), RtValue::int(-1))?;
        } else {
            ctx.bb().lock()?.put("direction".to_string(), RtValue::int(1))?;
        }


        Ok(TickResult::success())
    }
}

fn target(args: RtArgs, ctx: &TreeContextRef) -> Result<i64, RuntimeError> {
    args
        .first()
        .and_then(|v| v.cast(ctx.clone()).int().ok())
        .flatten()
        .ok_or(RuntimeError::fail("target is absent".to_owned()))
}

fn current(ctx: &TreeContextRef) -> Result<i64, RuntimeError> {
    ctx
        .bb()
        .lock()?
        .get("curr_coord".to_owned())?
        .and_then(|v| v.clone().as_int())
        .ok_or(RuntimeError::fail("current is absent".to_owned()))
}
