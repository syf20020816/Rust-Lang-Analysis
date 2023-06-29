#[macro_export]
macro_rules! int_impl {
    (
        SelfT = $SelfT:ty,
        ActualT = $ActualT:ident,
        UnsignedT = $UnsignedT:ty,
        BITS = $BITS:literal,
        BITS_MINUS_ONE = $BITS_MINUS_ONE:literal,
        Min = $Min:literal,
        Max = $Max:literal,
        rot = $rot:literal,
        rot_op = $rot_op:literal,
        rot_result = $rot_result:literal,
        swap_op = $swap_op:literal,
        swapped = $swapped:literal,
        reversed = $reversed:literal,
        le_bytes = $le_bytes:literal,
        be_bytes = $be_bytes:literal,
        to_xe_bytes_doc = $to_xe_bytes_doc:expr,
        from_xe_bytes_doc = $from_xe_bytes_doc:expr,
        bound_condition = $bound_condition:literal,
    ) => {
        /// build constant MIN and MAX
        /// such as i8:
        /// - pub const MIN:i8 = !i8::MAX , !(128) == -128
        /// - pub const MAX:u8 = u8::MAX >> 1 , u8::MAX = 256 ,256 >> 1 == 128
        ///
        /// **see source code : pub const MIN: Self = !Self::MAX;**
        pub const MIN:$ActualT = !$ActualT::MAX;
        pub const MAX:$UnsignedT = <$UnsignedT>::MAX >> 1;
        /// Space occupied by type (bits) such as i8 : 8bits
        pub const BITS: u32 = <$UnsignedT>::BITS;
        /// impl for Self
        /// if myType is MyImplI8
        /// ```txt
        /// impl MyImplI8{
        ///     fn ...
        /// }
        /// ```
        impl $SelfT{
            /// here need from_str_radix function from `super::from_str_radix()`
            ///
            /// so we need to see this func
            // pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
            //     from_str_radix(src, radix)
            // }
            pub fn set_max(&mut self){
                self.max = MAX;
            }
            pub fn set_min(&mut self){
                self.min = MIN;
            }
        }
    };
}