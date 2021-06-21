
extern crate proc_macro;    // Rust 自带 proc_macro crate, 无需添加依赖（在 Rust 1.31.0 时，extern crate 仍是必须的）

// 这些 crate 让解析任何我们所要处理的 Rust 代码变得更简单：为 Rust 编写整个的解析器并不是一件简单的工作。
use crate::proc_macro::TokenStream;
use quote::quote;   // 将 syn 解析的数据结构转换回 Rust 代码。
use syn;            // 将字符串中的 Rust 代码解析成为一个可以操作的数据结构。

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 首先将来自 TokenStream 的 input 转换为一个我们可以解释和操作的数据结构（语法树）
    let ast = syn::parse(input).unwrap();
    /*
    syn::parse函数获取一个 TokenStream 并返回一个表示解析出 Rust 代码的 DeriveInput 结构体，大致如下
        DeriveInput {
            // --snip--

            ident: Ident {
                ident: "Pancakes",
                span: #0 bytes(95..103)
            },
            data: Struct(
                DataStruct {
                    struct_token: Struct,
                    fields: Unit,
                    semi_token: Some(
                        Semi
                    )
                }
            )
        }

    该结构体的字段展示了我们解析的 Rust 代码是一个类单元结构体，其 ident（ identifier，表示名字）
    当我们在main.rs里面定义struct Pancakes;前面加上#[derive(HelloMacro)]注解时，此时ident为 Pancakes。
    该结构体里面有更多字段描述了所有类型的 Rust 代码，查阅[syn 中 DeriveInput 的文档](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html)以获取更多信息。

    当调用 syn::parse 函数失败时，我们用 unwrap 来使 hello_macro_derive 函数 panic。
    在错误时 panic 对过程宏来说是必须的，因为 proc_macro_derive 函数必须返回 TokenStream 而不是 Result，以此来符合过程宏的 API。
    这里选择用 unwrap 来简化了这个例子；在生产代码中，则应该通过 panic! 或 expect 来提供关于发生何种错误的更加明确的错误信息。


    impl_hello_macro 函数用于构建所要包含在内的 Rust 新代码其输出也是 TokenStream。
    所返回的 TokenStream 会被加到我们的 crate 用户所写的代码中，因此，当用户编译他们的 crate 时，他们会获取到我们所提供的额外功能。
    */
    impl_hello_macro(&ast)
}

/*
注意 hello_macro_derive 函数中代码分割的方式，它负责解析 TokenStream，
而 impl_hello_macro 函数则负责转换语法树：这让编写一个过程式宏更加方便。

当用户在一个类型上指定 #[derive(HelloMacro)] 时，hello_macro_derive 函数将会被调用。
原因在于我们已经使用 proc_macro_derive 及其指定名称对 hello_macro_derive 函数进行了注解：HelloMacro ，其匹配到 trait 名，这是大多数过程宏遵循的习惯。
*/

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // 得到一个包含以 ast.ident 作为注解类型名字（标识符）的 Ident 结构体实例
    let name = &ast.ident;

    // quote! 宏让我们可以编写希望返回的 Rust 代码
    let gen = quote! {
        // quote!宏也提供了一些非常酷的模板机制；我们可以写 #name ，然后 quote! 会以名为 name 的变量值来替换它。
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
                /*
                此处所使用的 stringify! 为 Rust 内置宏。其接收一个 Rust 表达式，如 1 + 2 ， 然后在编译时将表达式转换为一个字符串常量，如 "1 + 2" 。
                这与 format! 或 println! 是不同的，它计算表达式并将结果转换为 String 。
                有一种可能的情况是，所输入的 #name 可能是一个需要打印的表达式，因此我们用 stringify! 。
                stringify! 编译时也保留了一份将 #name 转换为字符串之后的内存分配。
                */
            }
        }
    };
    // quote! 宏执行的直接结果并不是编译器所期望的并需要转换为 TokenStream。
    // 为此需要调用 into 方法，它会消费这个中间表示（intermediate representation，IR）并返回所需的 TokenStream 类型值。
    gen.into()
}
