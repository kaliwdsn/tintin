# Rust入门-第一课

## 问题
> 创建一个Rust工程，

>（1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符

>（2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符

>（3）使用Cargo编译运行此工程

>（需要自行发现其中的细节，一个考点是：ascii码字符的顺序）

## 答案
需要遵循以下步骤：

创建Rust工程：
在您选择的目录中，打开终端并执行以下命令，创建一个新的Rust工程：


``` bash      
 cargo new class1
```
这将创建一个名为class1的新Rust工程。


```
za_rev.rs 和 az，分别包含了print_characters函数。在这些函数中，我们使用字节字面值（如b'a'）来表示字符，然后将其转换为字符形式打印出来。

编译和运行：
打开终端，导航到工程目录，并执行以下命令来使用Cargo编译和运行工程：


        
``` bash

cd class1
cargo run

```
>输出
> za_rev::print_characters() 输出:
```
a
`
_
^
]
\
[
Z

```

> az::print_characters()输出:
```
A
B
C
D
E
F
G
H
I
J
K
L
M
N
O
P
Q
R
S
T
U
V
W
X
Y
Z
[
\
]
^
_
`
a
b
c
d
e
f
g
h
i
j
k
l
m
n
o
p
q
r
s
t
u
v
w
x
y
z
```