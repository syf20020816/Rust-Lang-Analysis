# Technical Term

- author : syf20020816@outlook.com
- date : 2023-06-29

## 旋转操作

旋转操作（rotation operation）指的是对二进制数进行循环位移的操作。旋转操作在计算机编程中常常用于位操作、密码学、数据压缩等领域

旋转操作通常涉及两个参数：要旋转的二进制数和旋转的位数。例如，如果我们将 10101100 向左旋转 2 位，则结果应为 10110001。向左旋转意味着最左边的位将移动到最右边，同时保持其它位的相对顺序不变。

同样，如果我们将 10101100 向右旋转 3 位，则结果应为 10010101。向右旋转意味着最右边的位将移动到最左边，同时保持其它位的相对顺序不变。

> note ： 向左旋转变大，向右旋转变小

<img src="https://github.com/syf20020816/Rust-Lang-Analysis/blob/main/imgs/ro_example.png">

## 交换操作

交换操作（swap operation）指的是将二进制数高位与低位进行交换

例如：1011 0101 交换后 0101 1011