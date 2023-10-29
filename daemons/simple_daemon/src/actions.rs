use forester_rs::runtime::args::{RtArgs, RtValue};
use forester_rs::runtime::context::TreeContextRef;
use forester_rs::runtime::{RuntimeError, TickResult};
use forester_rs::runtime::action::{Impl, Tick};
use crate::utils::{err, get_q, get_result, R, r};


pub struct Add;

impl Impl for Add {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let result = get_result(&ctx.bb().lock()?)?;
        let value = RtValue::int(result + get_q(args)?);
        ctx.trace(format!("the result is {}", value))?;
        ctx.bb().lock()?.put(r(), value)?;

        Ok(TickResult::success())
    }
}


pub struct Sub;

impl Impl for Sub {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let result = get_result(&ctx.bb().lock()?)?;
        let value = RtValue::int(result - get_q(args)?);
        ctx.trace(format!("the result is {}", value))?;
        ctx.bb().lock()?.put(r(), value)?;

        Ok(TickResult::success())
    }
}

pub struct Mul;

impl Impl for Mul {
    fn tick(&self, args: RtArgs, ctx: TreeContextRef) -> Tick {
        let result = get_result(&ctx.bb().lock()?)?;
        let value = RtValue::int(result * get_q(args)?);
        ctx.trace(format!("the result is {}", value))?;
        ctx.bb().lock()?.put(r(), value)?;

        Ok(TickResult::success())
    }
}