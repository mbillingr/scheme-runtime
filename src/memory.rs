pub fn allocate<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}
