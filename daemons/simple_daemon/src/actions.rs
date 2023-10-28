use forester_rs::runtime::args::{RtArgs, RtValue};
use forester_rs::runtime::context::TreeContextRef;
use forester_rs::runtime::{RuntimeError, TickResult};
use forester_rs::runtime::action::{Impl, Tick};
use crate::utils::{err, get_q, get_result, R};

pub struct Add;

impl Impl for Add {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let bb_g = ctx.bb();
        let mut bb = bb_g.lock()?;
        let result = get_result(&bb)?;
        println!(">> {:?} + {}", result,get_q(args.clone())? );
        bb.put(R.to_string(), RtValue::int(result + get_q(args)?))?;

        Ok(TickResult::success())
    }
}


pub struct Sub;

impl Impl for Sub {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let bb_r = ctx.bb();
        let mut bb = bb_r.lock()?;
        let result = get_result(&bb)?;
        println!(">> {:?} - {}", result,get_q(args.clone())? );
        bb.put(R.to_string(), RtValue::int(result - get_q(args)?))?;

        Ok(TickResult::success())
    }
}

pub struct Mul;

impl Impl for Mul {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let bb_r = ctx.bb();
        let mut bb = bb_r.lock()?;
        let result = get_result(&bb)?;
        println!(">> {:?} * {}", result,get_q(args.clone())? );
        bb.put(R.to_string(), RtValue::int(result * get_q(args)?))?;

        Ok(TickResult::success())
    }
}