#[cfg(feature = "tracing")]
pub use tracing::{
    debug, error, info, trace, warn,
    debug_span, error_span, info_span, trace_span, warn_span
};

#[cfg(not(feature = "tracing"))]
mod noop_macros {
    #[macro_export]
    macro_rules! trace {
        ($($args:tt)*) => {};
    }

    #[macro_export]
    macro_rules! debug {
        ($($args:tt)*) => {};
    }

    #[macro_export]
    macro_rules! info {
        ($($args:tt)*) => {};
    }

    #[macro_export]
    macro_rules! warn {
        ($($args:tt)*) => {};
    }

    #[macro_export]
    macro_rules! error {
        ($($args:tt)*) => {};
    }

    #[macro_export]
    macro_rules! trace_span {
        ($($args:tt)*) => {
            pub struct NoopSpan;
            impl NoopSpan {
                #[allow(unused)]
                pub fn enter(&self) -> NoopSpanGuard { NoopSpanGuard }
            }
            pub struct NoopSpanGuard;
            impl Drop for NoopSpanGuard {
                fn drop(&mut self) {}
            }
            NoopSpan
        };
    }

    // 修正后的宏定义方式
    #[macro_export]
    macro_rules! debug_span {
        ($($args:tt)*) => {
            $crate::noop_macros::trace_span!($($args)*)
        };
    }

    #[macro_export]
    macro_rules! info_span {
        ($($args:tt)*) => {
            $crate::noop_macros::trace_span!($($args)*)
        };
    }

    #[macro_export]
    macro_rules! warn_span {
        ($($args:tt)*) => {
            $crate::noop_macros::trace_span!($($args)*)
        };
    }

    #[macro_export]
    macro_rules! error_span {
        ($($args:tt)*) => {
            $crate::noop_macros::trace_span!($($args)*)
        };
    }
}

#[cfg(not(feature = "tracing"))]
pub use noop_macros::*;