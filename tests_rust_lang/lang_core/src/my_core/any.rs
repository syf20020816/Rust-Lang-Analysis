//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/14
//! @version:0.0.1
//! @description:
//! ```

use std::intrinsics;

//! 用于动态键入或类型反射的实用程序。Any和TypeId Any本身可以用于获取TypeId，并且在用作trait对象时具有更多功能。
//! 作为&dyn-Any（借用的trait对象），它有is和dowcast_ref方法，用于测试包含的值是否为给定类型，并将内部值作为类型进行引用。
//! 与&mut dyn Any一样，还有downstast_mut方法，用于获取对内部值的可变引用。
//! Box＜dyn Any＞添加了向下转换方法，该方法试图转换为Box＜T＞。有关详细信息，请参阅Box文档。
//! 请注意，&dyn-Any仅限于测试值是否为指定的具体类型，而不能用于测试类型是否实现了特性。
// 智能指针和dyn-Any当使用Any作为特征对象时，特别是对于Box＜dyn Any＞或Arc＜dyn Any＞这样的类型，
// 需要记住的一点是，只需对值调用.type_id（）就会产生容器的TypeId，而不是底层的特征对象。
// 这可以通过将智能指针转换为&dyn Any来避免，后者将返回对象的TypeId。例如：
pub struct Any;


/// TypeId表示一个类型的全局唯一标识符。
/// 每个TypeId都是一个不透明的对象，不允许检查内部内容，但允许进行克隆、比较、打印和显示等基本操作。
/// TypeId目前仅适用于归因于“static”的类型，但将来可能会取消此限制。
/// 虽然TypeId实现了Hash、PartialOrd和Ord，但值得注意的是，Rust版本之间的哈希和排序会有所不同。
/// 小心在代码中依赖它们！
#[derive(Debug, Copy, Clone, Eq, PartialOrd, Ord)]
pub struct TypeId {
    t: u128,
}

/// 实现比较
impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl TypeId {
    /// 返回用于实例化此泛型函数的类型的TypeId
    ///
    /// 关键字 ?Sized 用于表示一个类型可能是不定大小（unsized）的类型。不定大小类型是指在编译时无法确定具体大小的类型，例如动态分配的数组或 trait 对象。
    ///
    /// 在 pub const unsafe fn of<T: ?Sized + 'static>() 中，T: ?Sized 表示泛型类型 T 可能是不定大小的类型。这意味着函数 of 可以接受任意大小的 T 类型作为参数，包括不定大小的类型。
    ///
    /// 为什么要使用 ?Sized 前缀呢？这是因为 Rust 默认情况下要求泛型类型是大小已知的（sized），这样才能在栈上分配内存。但是有些类型的大小是在运行时才能确定的，例如具有动态数组长度的数组类型，或者 trait 对象。在这种情况下，我们需要在类型前面加上 ?Sized 来表明它是不定大小的，从而允许它们作为泛型参数。
    pub const unsafe fn of<T: ?Sized + 'static>() -> TypeId {
        let t: u128 = intrinsics::type_id::<T>();
        TypeId {
            t
        }
    }
}