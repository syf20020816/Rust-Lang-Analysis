# int_macros

请参看`tests_rust_lang/lang_tests/src/core_tests/num_shells_int_macros.rs`

- package : `src/num/int_macros.rs`

## Preface

❗请注意，这里的int_macros虽然和`src/num/shells/int_macros.rs`文件名称相同但是并不是一个东西，这里是设计了`int_impl!`宏

所有`impl u/i num types`都由该宏实现如（i8）：

```rust
impl i8 {
    int_impl! {
        Self = i8,
        ActualT = i8,
        UnsignedT = u8,
        BITS = 8,
        BITS_MINUS_ONE = 7,
        Min = -128,
        Max = 127,
        rot = 2,
        rot_op = "-0x7e",
        rot_result = "0xa",
        swap_op = "0x12",
        swapped = "0x12",
        reversed = "0x48",
        le_bytes = "[0x12]",
        be_bytes = "[0x12]",
        to_xe_bytes_doc = "",
        from_xe_bytes_doc = "",
        bound_condition = "",
    }
}
```

<hr />

## Ⓜ️Source

源码部分我将拆为2个部分进行说明

```rust
macro_rules! int_impl {
    (
       `please see Macro input`
    ) => {
       `please see Macro operation`
    }
}

```

### Macro input

作为整个宏的输入

```rust
macro_rules! int_impl {
    (
        Self = $SelfT:ty,
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
    ) =>{}
}
```

- `Self = $SelfT:ty` : Num的具体类型，如i8,u8
- `ActualT = $ActualT:ident`: Num真正的类型
- `UnsignedT = $UnsignedT:ty`：对应无符号类型
- `BITS = $BITS:literal`：该类型所占字节长度，如i8占用8个字节(literal：字面量)
- `BITS_MINUS_ONE = $BITS_MINUS_ONE:literal`：表示 `BITS - 1`
- `Min = $Min:literal`：最小值 match `$T:MIN`
- `Max = $Max:literal`：最大值 match `$T:MAX`
- `rot = $rot:literal`：表示旋转操作（若你不知道什么是旋转操作请看：technical_term.md）
- `rot_op = $rot_op:literal`：表示旋转操作的符号
- `rot_result = $rot_result:literal`：表示旋转操作的结果
- `swap_op = $swap_op:literal`: 表示交换操作的符号
- `swapped = $swapped:literal`: 表示交换结果
- `reversed = $reversed:literal`: 表示反转结果
- `le_bytes = $le_bytes:literal`: 表示小端字节序
- `be_bytes = $be_bytes:literal`: 表示大端字节序
- `to_xe_bytes_doc = $to_xe_bytes_doc:expr`: 用于转化为字节数组的文档注释
- ` from_xe_bytes_doc = $from_xe_bytes_doc:expr`: 用于从字节数组转化的文档注释
- `bound_condition = $bound_condition:literal`: 用于条件判断

### Macro operation

```rust
        pub const MIN: Self = !Self::MAX;

        #[stable(feature = "assoc_int_consts", since = "1.43.0")]
        pub const MAX: Self = (<$UnsignedT>::MAX >> 1) as Self;

        pub const BITS: u32 = <$UnsignedT>::BITS;

        pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
            from_str_radix(src, radix)
        }

        pub const fn count_ones(self) -> u32 { (self as $UnsignedT).count_ones() }

        pub const fn count_zeros(self) -> u32 {
            (!self).count_ones()
        }


        pub const fn leading_zeros(self) -> u32 {
            (self as $UnsignedT).leading_zeros()
        }

 
        pub const fn trailing_zeros(self) -> u32 {
            (self as $UnsignedT).trailing_zeros()
        }


        pub const fn leading_ones(self) -> u32 {
            (self as $UnsignedT).leading_ones()
        }


        pub const fn trailing_ones(self) -> u32 {
            (self as $UnsignedT).trailing_ones()
        }

        pub const fn rotate_left(self, n: u32) -> Self {
            (self as $UnsignedT).rotate_left(n) as Self
        }


        pub const fn rotate_right(self, n: u32) -> Self {
            (self as $UnsignedT).rotate_right(n) as Self
        }

        pub const fn swap_bytes(self) -> Self {
            (self as $UnsignedT).swap_bytes() as Self
        }


        pub const fn reverse_bits(self) -> Self {
            (self as $UnsignedT).reverse_bits() as Self
        }


        pub const fn from_be(x: Self) -> Self {
            #[cfg(target_endian = "big")]
            {
                x
            }
            #[cfg(not(target_endian = "big"))]
            {
                x.swap_bytes()
            }
        }

        pub const fn from_le(x: Self) -> Self {
            #[cfg(target_endian = "little")]
            {
                x
            }
            #[cfg(not(target_endian = "little"))]
            {
                x.swap_bytes()
            }
        }

        
        pub const fn to_be(self) -> Self { // or not to be?
            #[cfg(target_endian = "big")]
            {
                self
            }
            #[cfg(not(target_endian = "big"))]
            {
                self.swap_bytes()
            }
        }

        pub const fn to_le(self) -> Self {
            #[cfg(target_endian = "little")]
            {
                self
            }
            #[cfg(not(target_endian = "little"))]
            {
                self.swap_bytes()
            }
        }

       
        pub const fn checked_add(self, rhs: Self) -> Option<Self> {
            let (a, b) = self.overflowing_add(rhs);
            if unlikely!(b) {None} else {Some(a)}
        }

       
        pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
            // SAFETY: the caller must uphold the safety contract for
            // `unchecked_add`.
            unsafe { intrinsics::unchecked_add(self, rhs) }
        }

        pub const fn checked_add_unsigned(self, rhs: $UnsignedT) -> Option<Self> {
            let (a, b) = self.overflowing_add_unsigned(rhs);
            if unlikely!(b) {None} else {Some(a)}
        }

        
        pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
            let (a, b) = self.overflowing_sub(rhs);
            if unlikely!(b) {None} else {Some(a)}
        }

      
        pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
            unsafe { intrinsics::unchecked_sub(self, rhs) }
        }

        pub const fn checked_sub_unsigned(self, rhs: $UnsignedT) -> Option<Self> {
            let (a, b) = self.overflowing_sub_unsigned(rhs);
            if unlikely!(b) {None} else {Some(a)}
        }


        pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
            let (a, b) = self.overflowing_mul(rhs);
            if unlikely!(b) {None} else {Some(a)}
        }


        pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
            // SAFETY: the caller must uphold the safety contract for
            // `unchecked_mul`.
            unsafe { intrinsics::unchecked_mul(self, rhs) }
        }


        pub const fn checked_div(self, rhs: Self) -> Option<Self> {
            if unlikely!(rhs == 0 || ((self == Self::MIN) && (rhs == -1))) {
                None
            } else {
                // SAFETY: div by zero and by INT_MIN have been checked above
                Some(unsafe { intrinsics::unchecked_div(self, rhs) })
            }
        }


        pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
            // Using `&` helps LLVM see that it is the same check made in division.
            if unlikely!(rhs == 0 || ((self == Self::MIN) & (rhs == -1))) {
                None
            } else {
                Some(self.div_euclid(rhs))
            }
        }

        pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
            if unlikely!(rhs == 0 || ((self == Self::MIN) && (rhs == -1))) {
                None
            } else {
                // SAFETY: div by zero and by INT_MIN have been checked above
                Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
            }
        }

        pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
            // Using `&` helps LLVM see that it is the same check made in division.
            if unlikely!(rhs == 0 || ((self == Self::MIN) & (rhs == -1))) {
                None
            } else {
                Some(self.rem_euclid(rhs))
            }
        }


        pub const fn checked_neg(self) -> Option<Self> {
            let (a, b) = self.overflowing_neg();
            if unlikely!(b) {None} else {Some(a)}
        }

      
        pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
            let (a, b) = self.overflowing_shl(rhs);
            if unlikely!(b) {None} else {Some(a)}
        }

      
        pub const unsafe fn unchecked_shl(self, rhs: u32) -> Self {
            // SAFETY: the caller must uphold the safety contract for
            // `unchecked_shl`.
            // Any legal shift amount is losslessly representable in the self type.
            unsafe { intrinsics::unchecked_shl(self, rhs.try_into().ok().unwrap_unchecked()) }
        }


        pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
            let (a, b) = self.overflowing_shr(rhs);
            if unlikely!(b) {None} else {Some(a)}
        }

        pub const unsafe fn unchecked_shr(self, rhs: u32) -> Self {
            // SAFETY: the caller must uphold the safety contract for
            // `unchecked_shr`.
            // Any legal shift amount is losslessly representable in the self type.
            unsafe { intrinsics::unchecked_shr(self, rhs.try_into().ok().unwrap_unchecked()) }
        }

        pub const fn checked_abs(self) -> Option<Self> {
            if self.is_negative() {
                self.checked_neg()
            } else {
                Some(self)
            }
        }

        pub const fn checked_pow(self, mut exp: u32) -> Option<Self> {
            if exp == 0 {
                return Some(1);
            }
            let mut base = self;
            let mut acc: Self = 1;

            while exp > 1 {
                if (exp & 1) == 1 {
                    acc = try_opt!(acc.checked_mul(base));
                }
                exp /= 2;
                base = try_opt!(base.checked_mul(base));
            }
            // since exp!=0, finally the exp must be 1.
            // Deal with the final bit of the exponent separately, since
            // squaring the base afterwards is not necessary and may cause a
            // needless overflow.
            acc.checked_mul(base)
        }


        pub const fn saturating_add(self, rhs: Self) -> Self {
            intrinsics::saturating_add(self, rhs)
        }

        pub const fn saturating_add_unsigned(self, rhs: $UnsignedT) -> Self {
            // Overflow can only happen at the upper bound
            // We cannot use `unwrap_or` here because it is not `const`
            match self.checked_add_unsigned(rhs) {
                Some(x) => x,
                None => Self::MAX,
            }
        }

        pub const fn saturating_sub(self, rhs: Self) -> Self {
            intrinsics::saturating_sub(self, rhs)
        }

        pub const fn saturating_sub_unsigned(self, rhs: $UnsignedT) -> Self {
            // Overflow can only happen at the lower bound
            // We cannot use `unwrap_or` here because it is not `const`
            match self.checked_sub_unsigned(rhs) {
                Some(x) => x,
                None => Self::MIN,
            }
        }

        pub const fn saturating_neg(self) -> Self {
            intrinsics::saturating_sub(0, self)
        }

        pub const fn saturating_abs(self) -> Self {
            if self.is_negative() {
                self.saturating_neg()
            } else {
                self
            }
        }

        pub const fn saturating_mul(self, rhs: Self) -> Self {
            match self.checked_mul(rhs) {
                Some(x) => x,
                None => if (self < 0) == (rhs < 0) {
                    Self::MAX
                } else {
                    Self::MIN
                }
            }
        }

        pub const fn saturating_div(self, rhs: Self) -> Self {
            match self.overflowing_div(rhs) {
                (result, false) => result,
                (_result, true) => Self::MAX, // MIN / -1 is the only possible saturating overflow
            }
        }

        pub const fn saturating_pow(self, exp: u32) -> Self {
            match self.checked_pow(exp) {
                Some(x) => x,
                None if self < 0 && exp % 2 == 1 => Self::MIN,
                None => Self::MAX,
            }
        }

        pub const fn wrapping_add(self, rhs: Self) -> Self {
            intrinsics::wrapping_add(self, rhs)
        }

        pub const fn wrapping_add_unsigned(self, rhs: $UnsignedT) -> Self {
            self.wrapping_add(rhs as Self)
        }

        pub const fn wrapping_sub(self, rhs: Self) -> Self {
            intrinsics::wrapping_sub(self, rhs)
        }

        pub const fn wrapping_sub_unsigned(self, rhs: $UnsignedT) -> Self {
            self.wrapping_sub(rhs as Self)
        }

        pub const fn wrapping_mul(self, rhs: Self) -> Self {
            intrinsics::wrapping_mul(self, rhs)
        }

        pub const fn wrapping_div(self, rhs: Self) -> Self {
            self.overflowing_div(rhs).0
        }

        pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
            self.overflowing_div_euclid(rhs).0
        }

        pub const fn wrapping_rem(self, rhs: Self) -> Self {
            self.overflowing_rem(rhs).0
        }

        pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
            self.overflowing_rem_euclid(rhs).0
        }

        pub const fn wrapping_neg(self) -> Self {
            (0 as $SelfT).wrapping_sub(self)
        }

        #[rustc_allow_const_fn_unstable(const_inherent_unchecked_arith)]
        pub const fn wrapping_shl(self, rhs: u32) -> Self {
            // SAFETY: the masking by the bitsize of the type ensures that we do not shift
            // out of bounds
            unsafe {
                self.unchecked_shl(rhs & (Self::BITS - 1))
            }
        }

        #[rustc_allow_const_fn_unstable(const_inherent_unchecked_arith)]
        pub const fn wrapping_shr(self, rhs: u32) -> Self {
            // SAFETY: the masking by the bitsize of the type ensures that we do not shift
            // out of bounds
            unsafe {
                self.unchecked_shr(rhs & (Self::BITS - 1))
            }
        }


        pub const fn wrapping_abs(self) -> Self {
             if self.is_negative() {
                 self.wrapping_neg()
             } else {
                 self
             }
        }

        pub const fn unsigned_abs(self) -> $UnsignedT {
             self.wrapping_abs() as $UnsignedT
        }

        pub const fn wrapping_pow(self, mut exp: u32) -> Self {
            if exp == 0 {
                return 1;
            }
            let mut base = self;
            let mut acc: Self = 1;

            while exp > 1 {
                if (exp & 1) == 1 {
                    acc = acc.wrapping_mul(base);
                }
                exp /= 2;
                base = base.wrapping_mul(base);
            }

            // since exp!=0, finally the exp must be 1.
            // Deal with the final bit of the exponent separately, since
            // squaring the base afterwards is not necessary and may cause a
            // needless overflow.
            acc.wrapping_mul(base)
        }

        pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
            let (a, b) = intrinsics::add_with_overflow(self as $ActualT, rhs as $ActualT);
            (a as Self, b)
        }

        pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
            // note: longer-term this should be done via an intrinsic.
            // note: no intermediate overflow is required (https://github.com/rust-lang/rust/issues/85532#issuecomment-1032214946).
            let (a, b) = self.overflowing_add(rhs);
            let (c, d) = a.overflowing_add(carry as $SelfT);
            (c, b != d)
        }

        pub const fn overflowing_add_unsigned(self, rhs: $UnsignedT) -> (Self, bool) {
            let rhs = rhs as Self;
            let (res, overflowed) = self.overflowing_add(rhs);
            (res, overflowed ^ (rhs < 0))
        }

        pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
            let (a, b) = intrinsics::sub_with_overflow(self as $ActualT, rhs as $ActualT);
            (a as Self, b)
        }

        pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
            // note: longer-term this should be done via an intrinsic.
            // note: no intermediate overflow is required (https://github.com/rust-lang/rust/issues/85532#issuecomment-1032214946).
            let (a, b) = self.overflowing_sub(rhs);
            let (c, d) = a.overflowing_sub(borrow as $SelfT);
            (c, b != d)
        }

        pub const fn overflowing_sub_unsigned(self, rhs: $UnsignedT) -> (Self, bool) {
            let rhs = rhs as Self;
            let (res, overflowed) = self.overflowing_sub(rhs);
            (res, overflowed ^ (rhs < 0))
        }


        pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
            let (a, b) = intrinsics::mul_with_overflow(self as $ActualT, rhs as $ActualT);
            (a as Self, b)
        }

        pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
            // Using `&` helps LLVM see that it is the same check made in division.
            if unlikely!((self == Self::MIN) & (rhs == -1)) {
                (self, true)
            } else {
                (self / rhs, false)
            }
        }

        pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
            // Using `&` helps LLVM see that it is the same check made in division.
            if unlikely!((self == Self::MIN) & (rhs == -1)) {
                (self, true)
            } else {
                (self.div_euclid(rhs), false)
            }
        }

        pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
            if unlikely!(rhs == -1) {
                (0, self == Self::MIN)
            } else {
                (self % rhs, false)
            }
        }

        pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
            if unlikely!(rhs == -1) {
                (0, self == Self::MIN)
            } else {
                (self.rem_euclid(rhs), false)
            }
        }

        pub const fn overflowing_neg(self) -> (Self, bool) {
            if unlikely!(self == Self::MIN) {
                (Self::MIN, true)
            } else {
                (-self, false)
            }
        }


        pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
            (self.wrapping_shl(rhs), rhs >= Self::BITS)
        }

        pub const fn overflowing_abs(self) -> (Self, bool) {
            (self.wrapping_abs(), self == Self::MIN)
        }

        pub const fn overflowing_pow(self, mut exp: u32) -> (Self, bool) {
            if exp == 0 {
                return (1,false);
            }
            let mut base = self;
            let mut acc: Self = 1;
            let mut overflown = false;
            // Scratch space for storing results of overflowing_mul.
            let mut r;

            while exp > 1 {
                if (exp & 1) == 1 {
                    r = acc.overflowing_mul(base);
                    acc = r.0;
                    overflown |= r.1;
                }
                exp /= 2;
                r = base.overflowing_mul(base);
                base = r.0;
                overflown |= r.1;
            }

            // since exp!=0, finally the exp must be 1.
            // Deal with the final bit of the exponent separately, since
            // squaring the base afterwards is not necessary and may cause a
            // needless overflow.
            r = acc.overflowing_mul(base);
            r.1 |= overflown;
            r
        }


        pub const fn pow(self, mut exp: u32) -> Self {
            if exp == 0 {
                return 1;
            }
            let mut base = self;
            let mut acc = 1;

            while exp > 1 {
                if (exp & 1) == 1 {
                    acc = acc * base;
                }
                exp /= 2;
                base = base * base;
            }

            // since exp!=0, finally the exp must be 1.
            // Deal with the final bit of the exponent separately, since
            // squaring the base afterwards is not necessary and may cause a
            // needless overflow.
            acc * base
        }

        pub const fn div_euclid(self, rhs: Self) -> Self {
            let q = self / rhs;
            if self % rhs < 0 {
                return if rhs > 0 { q - 1 } else { q + 1 }
            }
            q
        }


        pub const fn rem_euclid(self, rhs: Self) -> Self {
            let r = self % rhs;
            if r < 0 {
                r.wrapping_add(rhs.wrapping_abs())
            } else {
                r
            }
        }


        pub const fn div_floor(self, rhs: Self) -> Self {
            let d = self / rhs;
            let r = self % rhs;
            if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
                d - 1
            } else {
                d
            }
        }

        pub const fn div_ceil(self, rhs: Self) -> Self {
            let d = self / rhs;
            let r = self % rhs;
            if (r > 0 && rhs > 0) || (r < 0 && rhs < 0) {
                d + 1
            } else {
                d
            }
        }


        pub const fn next_multiple_of(self, rhs: Self) -> Self {
            // This would otherwise fail when calculating `r` when self == T::MIN.
            if rhs == -1 {
                return self;
            }

            let r = self % rhs;
            let m = if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
                r + rhs
            } else {
                r
            };

            if m == 0 {
                self
            } else {
                self + (rhs - m)
            }
        }

        pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
            // This would otherwise fail when calculating `r` when self == T::MIN.
            if rhs == -1 {
                return Some(self);
            }

            let r = try_opt!(self.checked_rem(rhs));
            let m = if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
                // r + rhs cannot overflow because they have opposite signs
                r + rhs
            } else {
                r
            };

            if m == 0 {
                Some(self)
            } else {
                // rhs - m cannot overflow because m has the same sign as rhs
                self.checked_add(rhs - m)
            }
        }

        pub const fn ilog(self, base: Self) -> u32 {
            assert!(base >= 2, "base of integer logarithm must be at least 2");
            if let Some(log) = self.checked_ilog(base) {
                log
            } else {
                int_log10::panic_for_nonpositive_argument()
            }
        }

        pub const fn ilog2(self) -> u32 {
            if let Some(log) = self.checked_ilog2() {
                log
            } else {
                int_log10::panic_for_nonpositive_argument()
            }
        }


        pub const fn ilog10(self) -> u32 {
            if let Some(log) = self.checked_ilog10() {
                log
            } else {
                int_log10::panic_for_nonpositive_argument()
            }
        }

        pub const fn checked_ilog(self, base: Self) -> Option<u32> {
            if self <= 0 || base <= 1 {
                None
            } else {
                let mut n = 0;
                let mut r = self;

                // Optimization for 128 bit wide integers.
                if Self::BITS == 128 {
                    let b = Self::ilog2(self) / (Self::ilog2(base) + 1);
                    n += b;
                    r /= base.pow(b as u32);
                }

                while r >= base {
                    r /= base;
                    n += 1;
                }
                Some(n)
            }
        }


        pub const fn checked_ilog2(self) -> Option<u32> {
            if self <= 0 {
                None
            } else {
                // SAFETY: We just checked that this number is positive
                let log = (Self::BITS - 1) - unsafe { intrinsics::ctlz_nonzero(self) as u32 };
                Some(log)
            }
        }


        pub const fn checked_ilog10(self) -> Option<u32> {
            if self > 0 {
                Some(int_log10::$ActualT(self as $ActualT))
            } else {
                None
            }
        }


        pub const fn abs(self) -> Self {
            // Note that the #[rustc_inherit_overflow_checks] and #[inline]
            // above mean that the overflow semantics of the subtraction
            // depend on the crate we're being called from.
            if self.is_negative() {
                -self
            } else {
                self
            }
        }

        pub const fn abs_diff(self, other: Self) -> $UnsignedT {
            if self < other {
                // Converting a non-negative x from signed to unsigned by using
                // `x as U` is left unchanged, but a negative x is converted
                // to value x + 2^N. Thus if `s` and `o` are binary variables
                // respectively indicating whether `self` and `other` are
                // negative, we are computing the mathematical value:
                //
                //    (other + o*2^N) - (self + s*2^N)    mod  2^N
                //    other - self + (o-s)*2^N            mod  2^N
                //    other - self                        mod  2^N
                //
                // Finally, taking the mod 2^N of the mathematical value of
                // `other - self` does not change it as it already is
                // in the range [0, 2^N).
                (other as $UnsignedT).wrapping_sub(self as $UnsignedT)
            } else {
                (self as $UnsignedT).wrapping_sub(other as $UnsignedT)
            }
        }

        pub const fn signum(self) -> Self {
            // Picking the right way to phrase this is complicated
            // (<https://graphics.stanford.edu/~seander/bithacks.html#CopyIntegerSign>)
            // so delegate it to `Ord` which is already producing -1/0/+1
            // exactly like we need and can be the place to deal with the complexity.
            self.cmp(&0) as _
        }


        pub const fn is_positive(self) -> bool { self > 0 }

        pub const fn is_negative(self) -> bool { self < 0 }

        pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
            self.to_le().to_ne_bytes()
        }

        pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
            // SAFETY: integers are plain old datatypes so we can always transmute them to
            // arrays of bytes
            unsafe { mem::transmute(self) }
        }

        pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
            Self::from_be(Self::from_ne_bytes(bytes))
        }

        pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
            Self::from_le(Self::from_ne_bytes(bytes))
        }

        pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
            // SAFETY: integers are plain old datatypes so we can always transmute to them
            unsafe { mem::transmute(bytes) }
        }

        pub const fn max_value() -> Self {
            Self::MAX
        }
```



## Code

