use alloc::{format, vec::Vec};

pub fn repeat_str(base:&str, number:usize) -> &str {
    let mut returns = "";
    for _ in 0..number {
        returns = push_str(returns, base);
    }
    return returns;
}

pub fn push_str(base:&str, string_add:&str) -> &str {
    return join(base, string_add, "").as_str();
}

pub fn join(base:&str, add:&str, between:&str) -> &str {
    return format!("{}{}{}", base, between, add);
}