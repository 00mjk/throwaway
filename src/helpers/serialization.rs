pub const fn is_none_or_false(value: &Option<bool>) -> bool {
    if let Some(value) = value {
        return !(*value);
    }

    true
}
