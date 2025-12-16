// utils/ctx.rs
use std::panic::Location;

#[track_caller]
pub fn build_ctx<T: AsRef<str>>(msg: Option<T>) -> String {
    let loc = Location::caller();
    match msg {
        Some(m) => format!(
            "[{}] {} @ {}:{}",
            module_path!(),
            m.as_ref(),
            loc.file(),
            loc.line()
        ),
        None => format!("[{}] @ {}:{}", module_path!(), loc.file(), loc.line()),
    }
}

#[macro_export]
macro_rules! ctx {
    () => {{ $crate::utils::ctx::build_ctx::<&str>(None) }};
    ($msg:expr) => {{ $crate::utils::ctx::build_ctx(Some($msg)) }};
}

#[track_caller]
pub fn build_with_ctx<T: AsRef<str>>(s1: T, s2: Option<T>) -> String {
    let loc = Location::caller();
    match s2 {
        Some(s) => format!(
            "[{}] {} @ {}:{} - {}",
            module_path!(),
            s1.as_ref(),
            loc.file(),
            loc.line(),
            s.as_ref()
        ),
        None => format!(
            "[{}] @ {}:{} - {}",
            module_path!(),
            loc.file(),
            loc.line(),
            s1.as_ref()
        ),
    }
}

#[macro_export]
macro_rules! with_ctx {
    ($exp:expr) => {{ $crate::utils::ctx::build_with_ctx($exp, None) }};
    ($msg:expr, $exp:expr) => {{ $crate::utils::ctx::build_with_ctx($msg, Some($exp)) }};
}
