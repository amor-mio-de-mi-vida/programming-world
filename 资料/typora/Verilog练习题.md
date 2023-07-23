# Verilog注意的点：

结构化建模时：

值得注意的是，在实例化元件时(相当于调用已知功能的电路)，**wire 类型信号可以被连接至任意端口上，但 reg 类型的信号只能被连接至元件的输入端口上。**在声明元件时，**我们可以将任意端口声明为 wire 类型，但只能将输出端口声明为 reg 类型**，否则会出现问题。

- assign 语句等号的左边必须时 wire 类型变量。(assign 持续性赋值，即当右侧表达式中变量发生变化时，左侧信号的值也会随之变化)

- output 端口要声明为 reg 类型。

- 多个 `initial` 或 `always` 结构中的语句会同时执行————并行工作。

- always 语句可省去 `@(敏感条件列表)`，此时整个结构无条件执行，一次执行完毕后，立即重新开始执行。一般配合延迟语句在 `testbench` 的编写中使用，例如：`always #5 clk = ~clk;`

- begin - end 顺序块的特点：

- 顺序块有以下特点：
  1. 顺序块中的语句是一条接一条按顺序执行的，只有前面的语句执行完成之后才能执行后面的语句，除非是带有内嵌延迟控制的非阻塞赋值语句。
  2. 如果语句包括延迟，那么延迟总是相对于前面那条语句执行完成的仿真时间的。

- 不要混用阻塞和非阻塞赋值，尽量只在组合逻辑中使用阻塞赋值，只在时序逻辑中使用非阻塞赋值。

- **模块中的语句除了顺序块之外，都是“并行的”**

- 输入输出端口若不特别说明类型及位宽，**默认为 1 位 `wire` 型**。

- 需要注意的是，**`reg` 型变量不能使用 `assign` 赋值**。而且，`reg` 型并**不一定被综合成寄存器**，它也可和 `always` 关键字配合（下一节会讲到），建模组合逻辑。

- **`assign` 语句不能在 `always` 和 `initial` 块中使用**。

- **在描述时序逻辑时要使用非阻塞式赋值 `<=`** 

- 缩减运算符

  运算符 `&`（与）、`|`（或）、`^`（异或）等作为单目运算符是**对操作数的每一位汇总运算**，如对于 `reg[31:0] B;` 中的 `B` 来说，`&B` 代表将 `B` 的每一位与起来得到的结果。

- 为了避免意料之外的锁存器的生成而导致错误，**建议大家为所有的 `if` 语句都写出相应的 `else` 分支**。
- 为了良好的代码可读性与可综合性，**不要在多个 `always` 块中对同一个变量进行赋值**！

parameter 型

`parameter` 类型用于**在编译时确认值的常量**，通过形如 `parameter 标识符 = 表达式;` 的语句进行定义，如：`parameter width = 8;`。在实例化模块时，可通过参数传递改变在被引用模块实例中已定义的参数（模块的实例化将在后面的章节进行介绍）。`parameter` 虽然看起来可变，但它属于常量，在编译时会有一个确定的值。

`parameter` 可以用于在模块实例化时指定数据位宽等参数，便于在结构相似、位宽不同的模块之间实现代码复用。

 integer 型

`integer` 数据类型一般为 32 位，与 C 语言中的 `int` 类似，**默认为有符号数**，在我们的实验中**主要用于 `for` 循环**（将在本章后面提到）。



## for 语句

Verilog 中 `for` 语句的语法和 C 语言基本相同。只是我们**通常会定义一个 `integer` 类型的变量作为循环变量**。下面给出一个例子（七人投票表决器，仅为演示使用，不一定为最佳实现）：

```verilog
module vote7(
    input [6:0] vote,
    output reg pass
    );
    reg[2:0] sum; // sum为reg型变量，用于统计赞成的人数
    integer i; // 循环变量
    always @(vote) begin // 此处使用always建模组合逻辑
        sum = 3'b000; // sum初值为0
        for (i = 0;i < 7;i = i + 1) begin // for语句
            if (vote[i]) sum = sum + 1; // 只要有人投赞成票，则sum加1
        end
        if (sum >= 3'd4) pass = 1'b1; // 若大于等于4人赞成，则表决通过
        else pass = 1'b0;
    end
endmodule
```

## while 语句

Verilog 中 `while` 语句的语法和 C 语言基本相同。下面给出一个例子（对一个 8 位二进制数中值为 1 的位进行计数，仅为演示使用，不一定为最佳实现）：

```verilog
module count1s_while(
    input clk,
    input [7:0] rega,
    output reg [3:0] count
    );

     always @(posedge clk) begin: count1 // 命名顺序块，建模时序逻辑
        reg[7:0] tempreg; // 用作循环执行条件表达式
        count = 0; // count初值为0
        tempreg = rega; // tempreg初值为rega
        while (tempreg) begin // 若tempreg非0，则执行以下语句
            if (tempreg[0]) count = count + 1; // 只要trmpreg最低位为1，则count加1
            tempreg = tempreg >> 1; // 逻辑右移1位
        end
    end
endmodule
```

**有符号数的处理方法：**

- 使用到 `$signed()` 尽量使得运算数间有/无符号相同（移位运算是个例外），比如进行加法时，两个操作数都有符号，或者都有符号，这样容易确定结果的符号。同时将 `$signed()` 用于较为简单的表达式
- 对于复杂的表达式避免使用 `$signed()` ，如果希望使用 `$signed()` 可以将这一部分抽离出来单独作为一个变量
- **最简单也是最重要的一点，当你不确定 `$signed()` 的行为时，不妨自行编写一个简单的testbench观察一下，通过观察结果可以直截了当地做出判断。**

用verilog写状态机

```verilog
`define S0 2'b00
`define S1 2'b01
`define S2 2'b10
`define S3 2'b11
module counting(
    input [1:0] num,
    input clk,
    output ans
    );

reg [1:0] status;

initial
begin
	status <= `S0;
end

always@(posedge clk)
begin
	case(status)
	`S0 : begin
				if (num == 2'b01)
				begin
					status <= `S1;
				end
				else if (num == 2'b10)
				begin
					status <= `S0;
				end
				else if (num == 2'b11)
				begin
					status <= `S0;
				end
				else 
				begin
					status <= `S0; //对于一切非正常输出，回到状态0
				end
			end
	
	`S1 : begin
				if (num == 2'b01)
				begin
					status <= `S1;
				end
				else if (num == 2'b10)
				begin
					status <= `S2;
				end
				else if (num == 2'b11)
				begin
					status <= `S0;
				end
				else 
				begin
					status <= `S0; //对于一切非正常输出，回到状态0
				end
			end
			
	`S2 : begin
				if (num == 2'b01)
				begin
					status <= `S1;
				end
				else if (num == 2'b10)
				begin
					status <= `S0;
				end
				else if (num == 2'b11)
				begin
					status <= `S3;
				end
				else 
				begin
					status <= `S0; //对于一切非正常输出，回到状态0
				end
			end
			
	`S3 : begin
				if (num == 2'b01)
				begin
					status <= `S1;
				end
				else if (num == 2'b10)
				begin
					status <= `S0;
				end
				else if (num == 2'b11)
				begin
					status <= `S0;
				end
				else 
				begin
					status <= `S0; //对于一切非正常输出，回到状态0
				end
			end
	endcase
end

assign ans = (status == `S3) ? 1'b1 : 1'b0;
endmodule
```

最后我们的输出模块，对 ans 根据当前的状态进行判断。这里和我们的表格存在一定的出入。表格中的 ans 为下一状态发生后的 ans 取值，也就是当当前状态为 S2，num 为 3 时，下一状态为 S3，同时一下个状态发生时我们的 ans 取 1。**而在 Verilog 中由于是对当前的状态与 ans 进行分析，所以就当状态为 S3 时，ans 取 1。**

### 状态机总结：

通过上述分析，我们的模块简单分为了 3 个部分，**初始化**，**状态更新**，**输出更新**，而分别利用 `initial` 块，`always` 块以及 `assign` 语句完成。而为了降低我们书写程序的复杂性，我们引用了**宏定义**的对不同状态进行了化简表达。

对于大部分的状态机，关键点在于寻找到所有的状态，并正确的完整状态迁移，同时要处理好 `always` 的敏感变量，剩下的书写部分并不存在很大的难度。

## tb的编写

### 模块实例化的方法

对电路元件模块实例化最常见的语法是：`<模块名> <实例名>(端口信号映射);`。

其中，端口信号映射方式也有两种：

- **位置映射：**`<模块名> <实例名>(信号 1, 信号 2, ...);`，其中信号 n 对应被实例化模块声明时排在第 n 位的端口。
- **名映射：**`<模块名> <实例名>(.端口名 a(信号 1), .端口名 b(信号 2), ...);`，其中信号 n 对应其前的端口名。

值得注意的是，在实例化元件时，`wire` 类型信号可以被连接至**任意端口**上，但 `reg` 类型的信号只能被连接至元件的**输入端口**上。在声明元件时，我们可以将任意端口声明为 `wire` 类型，但只能将输出端口声明为 `reg` 类型，否则会出现问题。

我们也可以悬空部分不需要连接的端口，下图的 uut_0、uut_1、uut_2 分别对应位置映射、名映射与悬空端口的实例。建议**每一行只连接一个端口信号**避免混乱。

- 层次化事件队列

  非阻塞赋值在执行的时候会将右侧运算在posedge clk执行，在posedge clk之后会进行统一赋值;

![image-20221017003822812](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221017003822812.png)

![image-20221017004745621](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221017004745621.png)

![image-20221017005858547](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221017005858547.png)
