use std::{fmt::Debug, ops::ControlFlow, time::Duration};

use crate::RetryPolicy;

pub struct Traced<P>(pub P);

impl<P, R> RetryPolicy<R> for Traced<P>
where
    P: RetryPolicy<R>,
    R: Debug,
{
    fn should_retry(&mut self, result: R) -> ControlFlow<R, Duration> {
        // get debug output of result eagerly so we can log it if we need to retry
        let res = format!("{result:?}");

        match self.0.should_retry(result) {
            ControlFlow::Continue(duration) => {
                tracing::warn!(?duration, res, "waiting to retrying request");
                ControlFlow::Continue(duration)
            }
            ControlFlow::Break(b) => ControlFlow::Break(b),
        }
    }
}
