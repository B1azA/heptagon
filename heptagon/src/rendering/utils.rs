pub(crate) fn to_bytes<T>(data: &[T]) -> &'static [u8] {
    unsafe {
        let bytes = (data as *const [T]) as *const u8;
        return std::slice::from_raw_parts(bytes, data.len() * std::mem::size_of::<T>());
    }
}
