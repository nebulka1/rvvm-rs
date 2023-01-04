pub trait DeviceExt {
    type DataTy;

    fn new(
        address: u64,
        size: usize,
        op_size_range: ::core::ops::RangeInclusive<u8>,
        data: Self::DataTy,
    ) -> Self;
}
