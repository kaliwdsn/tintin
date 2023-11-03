# Rust入门-第二课
## 所有权
>所有整数是复值 字符串移动

>引用可变引用 不可变引用 可变引用【scope】可以修改原始值
>可变引用具有排他性，不能同时存在

可变引用只能move 不能复制
不可变引用可以复制
// 定义一个包含字符串的结构体
struct Owner {
    data: String,
}

impl Owner {
    // 一个获取所有权的方法
    fn take_ownership(self) -> String {
        println!("Taking ownership of: {}", self.data);
        self.data
    }

    // 一个借用数据的方法
    fn borrow_data(&self) {
        println!("Borrowing data: {}", self.data);
    }
}

fn main() {
    // 创建一个包含字符串的结构体实例
    let original_owner = Owner {
        data: String::from("Hello, Rust!"),
    };

    // 调用拥有所有权的方法，将所有权转移
    let taken_data = original_owner.take_ownership();

    // 尝试使用已转移的所有权会导致编译错误
    // println!("Trying to use original_owner: {}", original_owner.data); // 这行代码将无法编译

    // 使用借用数据的方法
    println!("Using borrowed data: {}", taken_data);
}
// 定义一个包含字符串的结构体
struct Owner {
    data: String,
}

impl Owner {
    // 一个获取所有权的方法
    fn take_ownership(self) -> String {
        println!("Taking ownership of: {}", self.data);
        self.data
    }

    // 一个借用数据的方法
    fn borrow_data(&self) {
        println!("Borrowing data: {}", self.data);
    }
}

fn main() {
    // 创建一个包含字符串的结构体实例
    let original_owner = Owner {
        data: String::from("Hello, Rust!"),
    };

    // 调用拥有所有权的方法，将所有权转移
    let taken_data = original_owner.take_ownership();

    // 尝试使用已转移的所有权会导致编译错误
    // println!("Trying to use original_owner: {}", original_owner.data); // 这行代码将无法编译

    // 使用借用数据的方法
    println!("Using borrowed data: {}", taken_data);
}

在这个例子中，Owner结构体包含一个String类型的数据字段。在take_ownership方法中，结构体的所有权被转移，即self.data被返回并不再属于original_owner。尝试在main函数中使用original_owner.data将导致编译错误，因为所有权已经转移。

通过使用borrow_data方法，我们演示了如何通过引用（&self）借用结构体中的数据而不获取所有权。这样，我们可以在不转移所有权的情况下使用数据。

这个例子展示了Rust的所有权系统如何在编译时保障内存安全，通过所有权的转移和借用来确保程序的正确

### 作用域
### RALL