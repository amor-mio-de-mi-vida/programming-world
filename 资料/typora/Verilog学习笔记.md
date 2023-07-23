# Verilog学习笔记

------------------------------------

## 前言

### Verilog 建模概述

使用 Verilog 语言进行结构级建模时，可将原理图中器件及连线映射为相应的语言描述要素，从而完成建模。该层次建模描述使用低层次器件（如常见简单门电路）描述模块（module），同时使用连线描述系统内各模块输入输出间的关系。

### 结构化建模

结构化建模就是把基本门和/或功能单元（主要是我们自己定义的模块）连接起来，从而产生特定的功能元件。结构化建模的过程，类似于我们在 Logisim 中连接一个电路。

对电路元件进行实例化的最常见语法是：

> 模块名 实例名(端口信号映射);

其中，端口信号映射的格式也有两种：

> 1. **位置映射**：`模块名 实例名(信号1, 信号2, ...)`，其中信号 n 对应被实例化模块声明时排在第 n 位的端口。
> 2. **名映射**：`模块名 实例名(.端口名a(信号1), .端口名b(信号2), ...)`，其中信号 n 对应其前的端口名。

值得注意的是，在实例化元件时，wire 类型信号可以被连接至任意端口上，但 reg 类型的信号只能被连接至元件的输入端口上。在声明元件时，我们可以将任意端口声明为 wire 类型，但只能将输出端口声明为 reg 类型，否则会出现问题。

例子：

```verilog
module Adder(
    input a, 
    input b, 
    input cin, 
    output sum, 
    output overflow
    );
    wire s1, s2, s3;
    //xor 与 and 均为原语，是系统预定义的模块
    xor xor1(sum, a, b, cin);
    and and1(s1, a, b);
    and and2(s2, a, cin);
    and and3(s3, b, cin);
    or or1(overflow, s1, s2, s3);
endmodule   
```

对应的电路图如下所示：

![image-20220916231126159](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220916231126159.png)

### 行为级描述

使用行为级描述设计电路时，我们主要描述电路输入信号和输出信号间的逻辑关系，关注电路“**干什么**”而不是“**怎么做**”。低层次内部结构和实现细节等的考虑，以及将其转化为物理电路的过程，都由有关软件自动完成。其中这一转化过程就叫综合 (synthesis)。在编写代码时，我们也要注重自己编写的代码的可综合性，了解自己代码对应的真实电路。

行为级描述的方法一般有两种：1. 利用连续赋值语句 `assign` 描述电路。2. 利用 `initial` 结构、`always` 结构和过程控制语句描述电路。

#### 连续赋值语句 assign

```verilog
assign signal = expression;
```

其中 `signal` 必须是 **`wire`** 型数据，而 `expression` 则是由数据和运算符组成的表达式。

`assign` 语句的作用是将右侧表达式的值持续性的赋给左侧的信号，**一般用于描述一个信号是如何由其他信号生成的**。所谓持续性，指的是**当右侧表达式中变量的发生变化时，左侧信号的值也会随之变化。**

`assign` 语句**非常适合简单的组合逻辑的描述**，经常与三目运算符配合使用。一般来说，`assign` 语句综合出来的电路是右侧表达式化简后所对应的逻辑门组合。

#### 过程控制语句与有关结构

一个电路的输出，可以由**输入信号**决定（此时为纯粹的组合逻辑电路），也可以由**输入信号**和**电路当前的状态**决定（此时电路一般为时序电路）。我们可以把电路的当前状态（如果有）和电路的输出抽象为一些**变量**。通过描述不同条件下对这些**变量的变化规律**来描述电路，这就是利用过程控制语句与有关结构进行的行为级描述。

上文所述的变量，其实就是 `Verilog` 中的 `reg` 类型数据。要注意的是，`reg` 类型数据只是一个变量，用途是方便我们的描述，并不一定对应一个真实电路中的寄存器。综合工具在综合时，会通过对整体结构的分析，来判断电路具体如何实现。

利用过程控制语句与有关结构进行的行为级描述时，我们要解决两个主要问题：

1. 如何将**变量**和模块的**输出**建立起关系。
2. 如何**描述**不同情况下各变量的**变化规律**。

一般可以采用两种方法将变量和模块的输出建立起联系：

1. 采用 `assign` 语句，将变量的值赋给输出信号（此时输出信号为 `wire` 型）
2. 直接将 `output` 端口声明为 `reg` 型

#### 变量变化规律的描述

我们采用过程控制语句对变量的变化进行描述。过程控制语句与我们学习过的 C 语言有一定相似，主要包括阻塞/非阻塞赋值语句、`if`/`case` 等条件语句以及各种循环语句。

> 赋值语句：
> 在Verilog HDL语言中，信号有两种赋值方式：
> (1).非阻塞(Non_Blocking)赋值方式( 如 b <= a; )
>
> 1.块结束后才完成赋值操作。
> 		2.b的值并不是立刻就改变的。
> 		3.这是一种比较常用的赋值方法。（特别在编写可综合模块时）
> (2).阻塞(Blocking)赋值方式( 如 b = a; )
>
> 1.赋值语句执行完后,块才结束。
> 		2.b的值在赋值语句执行完后立刻就改变的。
> 		3.可能会产生意想不到的结果。
>
> b <= a;
> 这种方式的赋值并不是马上执行的，也就是说"always"块内的下一条语句执行后，b并不等于a，而是保持原来的值。"always"块结束后，才进行赋值。
>
> b = a;
> 这种赋值方式是马上执行的。也就是说执行下一条语句时，b已等于a。尽管这种方式看起来很直观，但是可能引起麻烦。下面举例说明:
>
> [例1]:
>
> ```verilog
> always @( posedge clk )
> begin
> b<=a;
> c<=b;
> end
> ```
>
> [例1] 中的"always"块中用了非阻塞赋值方式，定义了两个reg型信号b和c，clk信号的上升 沿到来时，b就等于a，c就等于b，这里应该用到了两个触发器。请注意：赋值是在"always"块结束后 执行的，c应为原来b的值。这个"always"块实际描述的电路功能如下图所示：
>
> ![image-20220917000257324](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917000257324.png)
>
> [例2]:
>
> ```verilog
> always @(posedge clk)
> begin
> b=a;
> c=b;
> end
> ```
>
> [例2]中的 "always"块用了阻塞赋值方式。clk信号的上升沿到来时，将发生如下的变化：b马上取a 的值，c马上取b的值(即等于a)，生成的电路图如下所示只用了一个触发器来寄存器a的值，又输出给 b和c。这大概不是设计者的初衷，如果采用[例1]所示的非阻塞赋值方式就可以避免这种错误。
>
> ![image-20220917000343647](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917000343647.png)

过程控制语句不能直接存在于模块内，一般**只能在** `initial` 或 `always` 结构中出现（*还可以出现在任务和函数中，但是我们的实验基本上不会涉及*）。一个模块中可以包含多个 `initial` 或 `always` 结构。多个 `initial` 或 `always` 结构中的语句会同时执行。

#### 两种基本结构和语句块

##### INITIAL 结构

`initial` 结构，形式为 `initial 语句（块）`，从仿真 0 时刻开始执行其中的语句。在整个仿真过程中只执行一次，一般用于初始化某些变量的值。

##### ALWAYS 结构

`always` 结构，形式为 `always@(敏感条件列表) 语句（块）`，从仿真 0 时刻开始执行。

敏感条件列表一般由多个敏感条件之间用 `or` 连接形成。`always` 条件语句在敏感条件列表中有任一条件满足时被触发。

敏感条件主要分为两种：边沿敏感和电平敏感，一般不应混用。边沿敏感条件的格式为 `posedge/negedge 信号名`，表明信号在处于上升沿/下降沿的时候会执行结构中的语句，一般用于时序逻辑的描述。电平敏感条件的格式为信号名，表明该信号发生改变（注意不是为真）时会执行结构中的语句，一般用于组合逻辑的描述。

使用 `always` 结构描述组合逻辑时，可以使用 `always@(*)` 的简写，表示语句中所涉及到变量的任何变化都会引起该 `always` 结构的执行。

可省去 `@(敏感条件列表)`，此时整个结构无条件执行，一次执行完毕后，立即重新开始执行。一般配合延迟语句在 `testbench` 的编写中使用，例如：`always #5 clk = ~clk;`

##### 语句块

块语句的作用是将多条语句合并成一组，使它们像一条语句那样。在使用上一节提到的各种控制语句或者要使用 `always`/`initial` 过程块时，如果要执行多条语句，就可以使用块语句，这就类似于 C 语言中大括号里的语句。

块语句有两种：顺序块和并行块。顺序块的关键字是 `begin` - `end`，并行块的关键字是 `fork` - `join`，关键字位于块语句的起始位置和结束位置，相当于 C 语言中的左大括号和右大括号。块语句也可以嵌套。

从名字就可以看出这两种块的特点，顺序块中的语句是顺序执行的，而并行块中的语句是并行执行的。在我们的实验中只会用到顺序块，顺序块有以下特点：

1. 顺序块中的语句是一条接一条按顺序执行的，只有前面的语句执行完成之后才能执行后面的语句，除非是带有内嵌延迟控制的非阻塞赋值语句。
2. 如果语句包括延迟，那么延迟总是相对于前面那条语句执行完成的仿真时间的。

配合语句块使用，两种基本结构的格式就变为：

```verilog
initial begin
// more procedural statements
end

always@(/*敏感条件列表*/) begin
// more procedural statements
end
```

#### 常见的过程控制语句

1.**`if` 语句**

```verilog
if (expression1)
    statement1;
else if (expression2)
    statement2;
else
    statement3;
```

2.**`while` 语句**

```verilog
while (expression)
    statement;
```

3.**`for` 语句**

```verilog
for (expression1; expression2; expression3)
    statement;
```

其中，每个 statement 可能是阻塞赋值语句或非阻塞赋值语句。他们在仿真时具有不同的行为。具体影响可以参考 **Verilog 语法**部分的讲解，实现原理可以参考**高级特性**的层次化事件队列。

#### 过程控制语句描述的一般用法

`always` 块实现组合逻辑的一般格式为：

```verilog
always@(*) begin
// 采用阻塞赋值，描述电路的变化规律
end
```

always块实现时序逻辑的一般格式为：

```verilog
always@(posedge clk) begin
// 采用非阻塞赋值，描述电路的变化规律
end
```

**注意：不要混用阻塞和非阻塞赋值，尽量只在组合逻辑中使用阻塞赋值，只在时序逻辑中使用非阻塞赋值。**

------------------------------------

## VCS入门：

### VCS 与 Verdi 简介

VCS 是一款编译型的 Verilog 仿真器。它能够将 Verilog 像编译 C 语言程序那样编译成一个可执行文件，只需运行这个可执行文件，就能进行仿真。VCS 的编译与运行都是以命令行形式完成的。

Verdi 是一个调试平台。它能够查看 VCS 仿真程序的运行轨迹、波形，同时对仿真过程中的值来源、错误原因进行追踪和分析。

### VCS 项目结构

![image-20220917085734463](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917085734463.png)

本示例工程中，各个文件的作用列举如下。

> . ├── clean.sh           # 脚本文件，用于清除编译产生的可执行文件和缓存 
>
> ├── compile.sh         # 脚本文件，执行此脚本，即可将 Verilog 源代码文件编译为可执行文件 		
>
> ├── README.md          # 工程说明文件，可直接双击打开 
>
> ├── run.sh             # 脚本文件，执行该脚本即可运行编译出的可执行文件，启动仿真 
>
> ├── sim                # 空文件夹，用于存放编译、仿真的结果 
>
> ├── src                # 存放源代码的文件夹 │  
>
> ├── adder.v        # 一个示例 Verilog 文件，简单的加法器 
>
> │  └── tb.v           # 一个示例 Testbench 文件 
>
> └── verdi.sh           # 脚本文件，执行该脚本可以打开 Verdi，查看仿真产生的波形

项目目录下，`.sh` 结尾的文件是脚本文件，用于执行一些常用任务；`src` 目录存放了设计的源代码和 Testbench；`sim` 目录将存放编译好的二进制文件和运行时产生的波形。

### 编译项目

若要开始仿真，首先需要将 Verilog 源代码编译为可执行文件。在 VCS-Example 文件夹中打开终端，并输入 `./compile.sh`，就可以编译项目了。执行命令后，会看到 VCS 的运行日志，如果编译成功，则输出类似这样：

![image-20220917090251850](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917090251850.png)

运行 `cat compile.sh` 或用 Sublime Text 打开 `compile.sh`，即可查看这个脚本的代码。第一行为 Shebang；`#` 开头的行为注释；具有实际功能的是最后一行，其调用 `vcs` 命令进行编译，将 `src` 目录下的所有 `.v` 文件编译为 `sim` 文件夹下的 `simv` 文件。具体参数的含义请参考注释。

### 运行仿真

直接运行 `./run.sh`，即可开始仿真。仿真过程会输出以下日志：

![image-20220917090625826](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917090625826.png)

开头几行是 Verdi 波形抓取器的日志；中间的加号和等号是 `tb.v` 中使用 `$display` 系统任务输出的日志（`$display` 的使用方法请参考后文 [系统任务](https://cscore.e1.buaa.edu.cn/tutorial/verilog/verilog-6/verilog-6-2/)）；最后是程序运行结束的提示。

同样，打开 `run.sh` 即可看到其命令及含义，在此不再赘述。

#### 波形文件的生成

运行仿真时，仿真程序可以将过程中产生的波形存放到 fsdb 文件中，供稍后调试时查看。我们提供的项目模板中，已经为大家写好了波形文件的生成选项，直接运行 `verdi.sh` 脚本即可查看波形。

如果同学们自己编写 Testbench 或仿真脚本，则需要注意以下 3 个条件，否则将无法在 Verdi 中正常查看仿真波形。

1. 在编译命令中，存在 `-fsdb` 选项（可参考 `compile.sh`）；
2. 在运行命令中，存在 `+fsdbfile+wave.fsdb` 选项（可参考 `run.sh`）；
3. 在 Testbench 中，添加了 `fsdbDumpvars()` 命令（可参考 `tb.v`）。

#### 查看波形

在仿真结束后，运行 `./verdi.sh`，即可打开 Verdi 交互式调试工具。在 Verdi 中，我们可以方便地查看波形和代码，对设计问题做出分析。

在源代码面板，我们可以查看设计的源代码。**注意 Verdi 无法显示中文，因此中文注释会显示为乱码。**

在波形面板，我们可以对信号的波形进行查看。**波形查看器默认没有波形，需手动添加要查看波形的信号。**在源代码面板中将信号名称拖动到波形面板左侧区域，或单击信号名称并按 Ctrl+W，就可查看信号的波形。

#### 重新加载代码、波形

在修改代码、重新进行仿真后，若想查看新的代码和波形，并不需要重新打开 Verdi。只需要按下 Shift+L 键（或点击界面左上方的 File->Reload Design...），就可以重新加载代码和波形，并且波形窗口中的信号列表不会丢失。

#### 调整进制显示方式

Verdi 中，信号波形默认以十六进制显示。如需以其他进制显示，则右击需要修改的信号，选择 Set Radix 选项，然后选择需要显示的方式。例如，若想以十进制方式显示值，则选择 Decimal 选项。

![image-20220917091713554](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917091713554.png)

#### 保存波形列表

关闭 Verdi 后，波形面板中的波形列表会丢失，下次打开 Verdi 会得到一个空的列表。如果在波形面板中添加了一些波形，而不想下次手动添加，可在波形面板中点击 File->Save Signal...，将波形列表保存下来，下次打开 Verdi 时，选择 Restore Signal 打开该文件，即可恢复所有添加的波形。

![image-20220917091835396](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917091835396.png)

#### 活动代码查看

在菜单中点击 Source->Active Annotation 选项（或点击源代码面板并按 X 键），即可开关 Active Annotation 功能。该功能可在源代码中实时显示信号的值（如图所示），在波形面板中点击不同时间，即可显示该时刻的值，十分方便。

#### 追踪驱动信号

在 Verdi 中，双击一个信号的名称，即可找到这个信号的驱动信号，十分方便。除此之外，Verdi 还可以显示信号的传播来源。

-----------------------------------------------

## Verilog语法

### 模块定义的方法

模块（module）是 Verilog HDL 的基本功能单元，它实际上代表了具有一定功能的电路实体。下面我们通过一个简单的与门实例来说明模块定义的基本语法：（**Verilog HDL 的注释方式与 C 语言相同**）

```verilog
module AndGate(
    input i1,
    input i2,
    output o
    ); // 模块名定义、端口定义及IO说明
    assign o = i1 & i2; // 模块功能定义
endmodule // 结束模块定义
```

或

```verilog
module AndGate(i1,i2,o); // 模块名定义及端口定义
    input i1;
    input i2; // 也可合并为一句: input i1,i2;
    output o;
    // 上为IO说明
    assign o = i1 & i2; // 模块功能定义
endmodule // 结束模块定义
```

两种方法没有实质上的区别，只是形式上有所不同：方法 1 对方法 2 中的端口定义及 IO 说明进行了合并。

从上面的例子可以看出，一个模块以 `module` 开始，以 `endmodule` 结束，包括模块名、端口定义、I/O 说明、内部信号声明和功能定义等部分。需要指出的是，**模块中的语句除了顺序块之外，都是“并行的”**；输入输出端口若不特别说明类型及位宽，**默认为 1 位 `wire` 型**。

### 常用数据类型

#### wire型

`wire` 型数据属于线网 `nets` 型数据，通常用于表示**组合逻辑信号**，可以将它类比为电路中的导线。它本身并不能存储数据，需要**有输入才有输出**（这里输入的专业术语叫驱动器），且输出随着输入的改变而即时改变。**一般使用 `assign` 语句对 `wire` 型数据进行驱动**。

`wire` 型的数据分为标量（1 位）和向量（多位）两种。**可以在声明过程中使用范围指示器指明位数**，如 `wire [31:0] a;`。冒号两侧分别代表最高有效位（MSB, Most Significant Bit）和最低有效位（LSB, Least Significant Bit）。在访问时，可以使用形如 `a[7:4]` 的方式取出 a 的第 7-4 位数据。

一般**在使用 `wire` 型数据前应先声明它**。但如果在模块实例的端口信号列表中使用了一个未声明的变量，则会将其**默认定义为 1 位的 `wire` 变量**。

**需要注意的是，信号变量与 C 语言中的变量有所不同，不能像 C 语言一样随意赋值，一般需要按照组合逻辑的规则进行操作。**比如，对于 `wire` 型变量 `a`，`assign a = a + 1`是不合法的。

#### reg型

`reg` 型是寄存器数据类型，具有**存储**功能。它**也分为标量和向量**，类似 `wire` 型，可以类比前面的教程。一般**在 `always` 块内**使用 `reg` 型变量（`always` 块将在本章后面提到），通过赋值语句来改变寄存器中的值。为了确定何时进行赋值，我们经常需要用到各种控制结构，包括 `while`、`for`、`switch` 等，这与 C 语言中的做法十分相似。

需要注意的是，**`reg` 型变量不能使用 `assign` 赋值**。而且，`reg` 型并**不一定被综合成寄存器**，它也可和 `always` 关键字配合（下一节会讲到），建模组合逻辑。

利用 reg 数据类型建模存储器

我们可以通过对 `reg` 型变量建立数组来对存储器建模，例如 `reg [31:0] mem [0:1023];`，其中**前面的中括号内为位宽，后面的中括号内为存储器数量**。这种写法在我们开始搭建CPU后会用到。

我们可以通过引用操作访问存储器型数据元素，类似于位选择操作，例如 `mem[2]` 就是访问 `mem` 中的第 3 个元素。

需要注意的是，Verilog HDL 中没有多维数组。

#### 数字字面量

Verilog 中的数字字面量可以按二进制（b 或 B）、八进制（o 或 O）、十六进制（h 或 H）、十进制（d 或 D）表示。

数字的完整表达为 `<位宽>'<进制><值>`，如 `10'd100`。省略位宽时采用默认位宽（与机器有关，一般为 **32 位**），省略进制时默认为**十进制**，值部分可以用下划线分开提高可读性，如 `16'b1010_1011_1111_1010`。

Verilog 中除了普通的数字以外，还有两个特殊的值：**`x`** 和 **`z`**。`x` 为不定值，当某一二进制位的值不能确定时出现，变量的**默认初始值为 `x`**。`z` 为高阻态，代表**没有连接到有效输入上**。对于位宽大于 1 的数据类型，**`x` 与 `z` 均可只在部分位上出现**。

> 问题？
>
> 3’101
>
> 8’b_0011_1010

#### integer型

`integer` 数据类型一般为 32 位，与 C 语言中的 `int` 类似，**默认为有符号数**，在我们的实验中**主要用于 `for` 循环**（将在本章后面提到）。

#### parameter型

`parameter` 类型用于**在编译时确认值的常量**，通过形如 `parameter 标识符 = 表达式;` 的语句进行定义，如：`parameter width = 8;`。在实例化模块时，可通过参数传递改变在被引用模块实例中已定义的参数（模块的实例化将在后面的章节进行介绍）。`parameter` 虽然看起来可变，但它属于常量，在编译时会有一个确定的值。

`parameter` 可以用于在模块实例化时指定数据位宽等参数，便于在结构相似、位宽不同的模块之间实现代码复用。

#### 几个例子：

```verilog
wire[7:0] w1;
// [7:0] 为该变量的位宽，代表该变量为 8 位，可以通过索引访问任意一位。
wire[32:1] w2, w3;
// 位宽可以不从 0 开始，此时访问某一位时需与声明相符，如 w2 的最高位为 w2[32]。
reg[31:0] r1, r2, mem[1023:0];
// 可以同时声明存储器和单个 reg，存储器的地址索引同样可以不从 0 开始。
2'b11;      // 2 位，对应十进制 3
32'd12;     // 32 位，对应十进制 12
32'h11;     // 32 位，对应十进制 17
32'o11;     // 32 位，对应十进制 9
4'b10x0;    // 从低位数第 2 位为不定值
```

**另外需要注意，`assign`、`always` 等关键字不可以用作变量名。**

### 组合逻辑建模常用语法

#### assign语句

`assign` 语句是连续赋值语句，是组合逻辑的建模利器，其作用是**用一个信号来驱动另一个信号**。如 `assign a = b;`，其中 `a` 为 `wire` 型（也可由**位拼接**得到，见运算符部分），`b` 是由数据和运算符组成的表达式。

`assign` 语句与 C 语言的赋值语句有所不同，这里“驱动”的含义类似于电路的连接，也就是说，`a` 的值**时刻**等于 `b`。这也解释了 `assign a = a + 1;` 这样的语句为什么是不合法的。由于这样的特性，**`assign` 语句不能在 `always` 和 `initial` 块中使用**。

`assign` 语句**经常与三目运算符配合使用建模组合逻辑**。一般来说，`assign` 语句综合出来的电路是右侧表达式化简后所对应的逻辑门组合。

> 问题？
>
> assign意味着左侧的信号值始终等于右侧，因此F项中令w1始终等于w1 | w1是错误的。

#### 常用运算符

Verilog HDL 中有相当多的运算符都与 C 语言基本相同，如：

- 基本运算符：`+`, `-`, `*`, `/`, `%` 等
- 位运算符：`&`, `|`, `~`, `^`, `>>`, `<<` 等
- 逻辑运算符：`&&`, `||`, `!` 等
- 关系运算符：`>`, `<`, `>=`, `<=` 等
- 条件运算符：`? :`

这些运算的运算规则与 C 语言相同，只是在操作数中出现了不定值 `x` 和高阻值 `z` 的话最终结果可能也是带 `x` 或 `z` 的。另外 Verilog 中没有自增、自减运算符。下面主要介绍其他与 C 不同的部分。

- 逻辑右移运算符 `>>` 与算术右移运算符 `>>>`

  它们的区别主要在于前者在最高位**补 0**，而后者在最高位**补符号位**。

- 相等比较运算符 `==` 与 `===` 和 `!=` 与 `!==`

  `==` 和 `!=` 可能由于不定值 `x` 和高阻值 `z` 的出现导致结果为**不定值 `x`**，而 `===` 和 `!==` 的结果一定是**确定的 0 或 1**（`x` 与 `z` 也参与比较）。

- 阻塞赋值 `=` 和非阻塞赋值 `<=`

  不同于 `assign` 语句，这两种赋值方式被称为过程赋值，通常出现在 `initial` 和 `always` 块中，**为 `reg` 型变量赋值**。这种赋值类似 C 语言中的赋值，不同于 `assign` 语句，赋值仅会在一个时刻执行。由于 Verilog 描述硬件的特性，Verilog程序内会有大量的并行，因而产生了这两种赋值方式。这两种赋值方式的详细区别我们会在之后的小节内介绍，这里暂时只需记住一点：为了写出正确、可综合的程序，**在描述时序逻辑时要使用非阻塞式赋值 `<=`** 。

- 位拼接运算符 `{}`

  这个运算符可以将几个信号的某些位**拼接**起来，例如 `{a, b[3:0], w, 3'b101};`；可以简化重复的表达式，如 `{4{w}}` 等价于 `{w,w,w,w}`；还可以嵌套，`{b, {3{a, b}}}` 等价于 `{b, {a, b, a, b, a, b}}`，也就等价于 `{b, a, b, a, b, a, b}`。

- 缩减运算符

  运算符 `&`（与）、`|`（或）、`^`（异或）等作为单目运算符是**对操作数的每一位汇总运算**，如对于 `reg[31:0] B;` 中的 `B` 来说，`&B` 代表将 `B` 的每一位与起来得到的结果。

### 时序逻辑建模常用语法

#### always块

`always` 块有如下两种用法：

- 若 `always` 之后紧跟 `@(...)`，其中括号内是敏感条件列表，表示当**括号中的条件满足**时，将会执行 `always` 之后紧跟的语句或**顺序语句块**（和 C 语言中的语句块类似，只是将大括号用 `begin` 和 `end` 替换了）。这种用法**主要用于建模时序逻辑**。

举例如下：

```verilog
always @(posedge clk)  // 表示在 clk 上升沿触发后面的语句块
begin
// 一些操作
end
```

- 若 `always` 之后紧跟 `@ *` 或 `@(*)`，则**表示对其后紧跟的语句或语句块内所有信号的变化敏感。**这种用法主要用于与 `reg` 型数据和阻塞赋值配合，**建模组合逻辑**。

- 若 `always` 紧跟语句，则表示在该语句执行完毕之后立刻再次执行。这种用法主要配合后面提到的时间控制语句使用，来产生一些周期性的信号。

  

`always` 的敏感条件列表中，条件使用变量名称表示，例如 `always @(a)` 表示当变量 `a` 发生变化时执行之后的语句；若条件前加上 `posedge` 关键字，如 `always @(posedge a)`，表示当 `a` 达到上升沿，即从 `0` 变为 `1` 时触发条件，下降沿不触发；加上 `negedge` 则是下降沿触发条件，上升沿不触发。每个条件使用逗号 `,` 或 `or` 隔开，**只要有其中一个条件被触发，`always` 之后的语句都会被执行**。

为了良好的代码可读性与可综合性，**不要在多个 `always` 块中对同一个变量进行赋值**！

#### initial块

`initial` 块后面紧跟的语句或顺序语句块在硬件仿真开始时就会运行，且仅会运行一次，一般用于对 `reg` 型变量的取值进行初始化。`initial` 块通常仅用于仿真，是**不可综合的**。下面的代码用于给寄存器 `a` 赋初始值 `0`：

```verilog
reg a;

initial begin
    a = 0;
end
```

注意：**wire型变量是无法在initial块中赋值的，并且always块中对wire型变量的赋值也无法实现记忆功能**

#### if语句

Verilog 中 `if` 语句的语法和 `C` 语言基本相同，也有 `else if`、`else` 这样的用法。但是，`if` 语句**只能出现在顺序块中**，其后的分支也只能是语句或顺序块。举例如下（下面的例子也使用了 `always` 建模组合逻辑）：

```verilog
always @ * begin
    if (a > b) begin
        out = a;
    end
    else begin
        out = b;
    end
end
```

为了避免意料之外的锁存器的生成而导致错误，**建议大家为所有的 `if` 语句都写出相应的 `else` 分支**。

注意：always @(*) 表示的是触发条件是**always引导的顺序块中所有驱动信号的变化**，而不是本模块所有信号的变化。

#### case语句

Verilog 中的 `case` 语句与 C 语言的写法略有区别，详见下方的示例。`case` 语句同样**只能出现在顺序块中**，其中的分支也只能是语句或顺序块。与 C 语言不同，`case` 语句在**分支执行结束后不会落入下一个分支**，而会自动退出。举例如下：

```verilog
always @(posedge clk) begin
  case(data)
      0: out <= 4;
      1: out <= 5;
      2: out <= 2;
      3: begin
          out <= 1;
      end
      default: ;
  endcase
end
```

需要指出的是，`case` 语句进行的是**全等比较**，也就是每一位都相等（包括 `x` 和 `z`）才认为相等。另外，还有 `casex` 和 `casez` 两种语句，我们的课程涉及不多，感兴趣的同学可以自行查阅相关资料。

#### for语句

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

注意：verilog里面没有自增运算符！

#### while语句

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

> 问题？第7行 begin: count1是什么意思？

#### 时间控制语句

时间控制语句通常出现在测试模块中，用来产生符合我们期望变化的测试信号，比如每隔 5 个时间单位变更一次信号等。这个语句通过关键字 `#` 实现延时，格式为 **`#时间`**，当延时语句出现在顺序块中时它后面的语句会在延时完毕后继续执行。举例如下：

```verilog
#3;         // 延迟 3 个时间单位
#5 b = a;   // b 为 reg 型，延迟 5 个时间单位后执行赋值语句
always #5 clk = ~clk;   // 每过 5 个时间单位触发一次，时钟信号反转，时钟周期为 10 个时间单位
assign #5 b = a;        // b 为 wire 型，将表达式右边的值延时 5 个时间单位后赋给 b
```

**注意：模块内部不同的顺序块之间是并行的**

连续赋值->电路导通     wire->导线

### 模块的典型内部结构

一个模块的典型结构可以大致划分为三个部分：组合逻辑、时序逻辑和对其他模块的引用。

在组合逻辑部分，通常使用到的语法为 **`assign` 语句**，用于对 `wire` 型变量进行连续赋值。根据情况，我们也可能会使用 **`always` 块**来建模组合逻辑。

在时序逻辑部分，**`always` 块**是必不可少的。通常我们会在 `always` 块中使用各种**流程控制语句**建模时序逻辑。有时，我们还需要 **`initial` 块**对变量进行一定的初始化。

引用其他模块时，我们会用到**模块实例化**的语法。这个语法将在后面的小节内为大家详细介绍。大家可以通过视频教程和练习题做初步了解。

### 阻塞赋值与非阻塞赋值

下面我们通过一个简单的实例，介绍阻塞赋值与非阻塞赋值的区别。

```verilog
module blocked_and_non_blocked(
    input clk,
    input a,
    output reg b_blocked,
    output reg c_blocked,
    output reg b_non_blocked,
    output reg c_non_blocked
    );

     // 阻塞赋值
     always @(posedge clk) begin
        b_blocked = a;
        c_blocked = b_blocked;
     end
     // 非阻塞赋值
     always @(posedge clk) begin
        b_non_blocked <= a;
        c_non_blocked <= b_non_blocked;
     end
endmodule
```

#### 非阻塞赋值语句

处在一个 `always` 块中的非阻塞赋值是**在块结束时**同时**并发**执行的。对于 ISim，**在每一条非阻塞赋值执行前**，仿真器“按下快门”保存下了在 `<=` 右边参与运算的变量值。在块结束进行赋值时，对于 `<=` 左边被赋值的变量，都是用“快照”中的值参与运算的。

![image-20220917104514941](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917104514941.png)

#### 阻塞赋值语句

阻塞赋值语句的执行是具有明确**顺序**关系的，在 `begin` - `end` 的顺序块中，当前一句阻塞赋值完成后（即 `=` 左边的变化为右边的值后），下一条阻塞赋值语句才会被继续执行。![image-20220917104549320](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917104549320.png)

### 有符号数的处理方法

在 Verilog HDL 中，`wire`、`reg` 等数据类型**默认都是无符号的**。当你希望做符号数的操作时，你需要使用 **`$signed()`**。

提醒： 为了方便同学们理解，这一节中会用一些简洁的代码段进行说明，同学们能够理解代码含义即可，不必关注上下文。但同时为了简洁可能存在不良的代码风格，请同学们不要轻易模仿。

#### 一个简单的例子

下面我们先通过一个比较器的例子进行说明：

```verilog
module comparator(
    input clk,
    input reset,
    input [3:0] a,
    input [3:0] b,
    output res
    );

     assign res = a > b;

endmodule
```

我们希望程序实现比较 a, b 大小的功能，若 a > b，res 输出 1，否则输出 0，下面进行测试。

```verilog
module comparator_tb;

    // Inputs
    reg clk;
    reg reset;
    reg [3:0] a;
    reg [3:0] b;

    // Outputs
    wire res;

    // Instantiate the Unit Under Test (UUT)
    comparator uut (
        .clk(clk), 
        .reset(reset), 
        .a(a), 
        .b(b), 
        .res(res)
    );

    initial begin
        clk = 0;
        reset = 0;
        a = 4;
        b = 1;

        #100 b = -1;
    end

endmodule
```

我们初始化 a = 4，b = 1，100ns 后 b 变为 -1。期望的结果是 res 始终为 1。下面是波形：

![image-20220917105303110](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917105303110.png)

可以看到 100ns 后，res 输出变为了 0，与预期不符。其原因在于比较时 Verilog 将 a 和 b 默认视为无符号数，-1 会被认为是 15(`4'b1111`)。

将比较代码修改为 `assign res = $signed(a) > $signed(b);`，程序即可达到预期结果。

值得一提的是，在对无符号数和符号数同时操作时，Verilog 会自动地做数据类型匹配，**将符号数向无符号数转化**。同样使用上面的例子，将代码修改为 `assign res = a > $signed(b);`，这样得到的结果仍然是错误的，因为在执行 `a > $signed(b)` 时，a 是无符号数，b 是符号数，Verilog 默认向无符号类型转化，得到的结果仍是无符号数的比较结果。

此外，对于**移位运算符，其右侧的操作数总是被视为无符号数，并且不会对运算结果的符号性产生任何影响**。结果的符号由运算符左侧的操作数和表达式的其余部分共同决定。

#### 推荐使用方案总结

一、使用到 `$signed()` 尽量使得运算数间有/无符号相同（移位运算是个例外），比如进行加法时，两个操作数都有符号，或者都有符号，这样容易确定结果的符号。同时将 `$signed()` 用于较为简单的表达式，例如我们推荐下面的第一种写法而尽量避免第二种写法。第二种写法不仅可读性较差，还容易导致意想不到的问题。

```verilog
    assign a = $signed(b) > $signed(c);

    assign a = (($signed(b) + $signed(c)) >> d) > $signed(e); 
```



二、对于复杂的表达式避免使用 `$signed()` ，如果希望使用 `$signed()` 可以将这一部分抽离出来单独作为一个变量，比如

```verilog
wire [1:0] in_a,in_b,out;
wire [1:0] op;
// wrong !!!
assign out = op == 0? in_a + in_b:
             op == 1? in_a - in_b:
             op == 2? in_a >>> in_b:
             $signed(in_a) >>> in_b;
// correct
wire shift;
assign shift = $signed(in_a) >>> in_b;
assign out = op == 0? in_a + in_b:
             op == 1? in_a - in_b:
             op == 2? in_a >>> in_b:
             shift;
```

这里需要注意，算术右移在左操作数无符号时高位仍然补0，与逻辑右移效果相同。



三、如果你实在担心使用 `$signed()` 会出现意想不到的 bug，那么最简单的方式就是避开它。比如符号拓展可以写成如下

```verilog
wire [1:0] unsignedValue = 2'b11;
wire [31:0] extendedValue;
// use $signed()
assign extendedValue = $signed(unsignedValue);
// do not use $signed()
assign extendedValue = {30{unsignedValue[1]},unsignedValue};
```

相信凭借大家对补码的了解，处理这些应该不难。



四、**最简单也是最重要的一点，当你不确定 `$signed()` 的行为时，不妨自行编写一个简单的testbench观察一下，通过观察结果可以直截了当地做出判断。**

> ## Verilog中的signed和unsigned
>
> ### 一、右值按[signed](https://so.csdn.net/so/search?q=signed&spm=1001.2101.3001.7020)还是unsigned
>
> - 1、一条运算究竟是按unsigned还是signed运算，取决于其右值的操作数是否含有unsigned变量，只要右值存在unsigned变量，整个操作就会按unsigned处理，否则必须要右值全是signed变量，整个操作才按signed处理。
>
> - 例如：
>
> - ```verilog
>   reg signed [7:0] din;
>   integer dou1,dou2;
>   initial 
>   	din = -5;
>   assign dout1 = din + 1;
>   assign dout2 = din + 1'b1; 
>   ```
>
>   此处dout1= -5 + 1 = -4,因为din为signed变量，不做特定描述的常数也为signed，所以dout1的结果为正确的有符号结果。
>   而dout2 = -5 + 1’b1 = 251 + 1’b1 = 252,因为1’b1默认为unsigned，所以-5会被转换到unsigned类型再进行运算。
>
>   2、如果对signed进行截位运算，那么即使是signed变量，截位后也会变成unsigned变量
>   例如：
>
>   ```verilog
>   reg signed [7:0] din;
>   integer dou1,dou2;
>   initial 
>   	din = -5;
>   assign dout1 = din[6:0];
>   assign dout2 = din[7:0]; 
>   ```
>
>   此处dout1=7b，因为只截取了111_1011，不会管有没有符号位，而dout2即使把符号位都截取了，也是fb。
>
>   ### 二、signed的自动扩位
>
>   如果是signed运算会先扩展到右值中最大的位宽再参与运算，例如：
>
>   ```verilog
>   reg signed [7:0] a,b;
>   reg signed       c;
>   integer dout;
>   initial begin 
>   	a = 1;
>   	b = 2;
>   end
>   assign dout = a + b + c;
>   ```
>
>   这里c会先扩展为8位，再参与运算，光这么看好像没什么问题，但是如果c=1时，扩位后c=1111_1111。计算后肯定不符合理论值，其原因就是**1bit的数无法同时表示sign和value**，所以通常会手动补上符号位：
>
>   ```verilog
>   assign dout = a + b + {1'b0,c};
>   ```
>
>   ### 三、系统函数$ signed 和 $unsigned
>
>   在verilog中可以通过$signed函数对一个unsigned变量在运算过程中作为signed变量处理，例如上文说的
>
>   ```verilog
>   reg signed [7:0] din;
>   integer dou1,dou2;
>   initial 
>   	din = -5;
>   assign dout1 = din + 1;
>   assign dout2 = din + $signed（1'b1）; 
>   ```
>
>   如果对1’b1使用$signed，那么dout1和dout2的值会一样，都是-4。
>
>   **而$unsigned函数，虽然存在，但是经测试其没有什么实质作用，对一个signed变量，其不能自动对一负值转换成绝对值。**
>
>   总结
>   1.只有当右值全为signed，计算才会按signed完成；否则，右值又要存在unsigned数，就按unsigned处理；
>   2.不论是对signed还是unsigned数进行截位，截位后的数据都是unsigned；
>   3.当右值位数不等时进行运算，小位宽数会先扩位到大位宽再参与运算；扩位按最高位进行扩，所以要尤其注意正负
>   4.如果在signed和unsigned计算中不想过分考虑扩位截位等逻辑，可以用$signed()函数来简化。



### 宏定义的简单使用

类似 C 语言，Verilog HDL 也提供了编译预处理指令。下面我们对其中的宏定义部分作一简要介绍。

在 Verilog HDL 语言中，为了和一般的语句相区别，编译预处理命令**以符号 ```（反引号，backtick）开头**（位于主键盘左上角，其对应的上键盘字符为 `~`。注意这个符号不同于单引号）。这些预处理命令的有效作用范围为定义命令之后到本文结束或到其他命令定义替代该命令之处。

宏定义用一个指定的标识符(即名字)来代表一个字符串，它的一般形式为：``define 标识符(宏名) 字符串(宏内容)`。它的作用是指定用标识符来代替字符串，在编译预处理时，把程序中该命令以后所有的标识符都替换成字符串。举例如下：

```verilog
`define WORDSIZE 8
// 省略模块定义
 reg[1:`WORDSIZE] data;
// 相当于定义 reg[1:8] data; 
```

注意，**引用宏名时也必须在宏名前加上符号 `**，以表明该名字是经过宏定义的名字。

----------------------------------------

## Verilog题目实例与分析

### 组合电路题目描述

简化ALU

![image-20220917205647230](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917205647230.png)

题目说明：

通过给定的输入 inputA，inputB 以及 op，根据功能输出 out。

### 组合电路例题实现过程

题目分析：

题目中的规格已经确定了我们的输入与输出，并给定了相应的变量名称。我们只需要通过判断相应的 op 的取值，根据 inputA 与 B，计算出 out 对应的取值。对于组合逻辑，一般不需要自己额外去定义新的变量，只需要将输出与输入的逻辑表达式给出就可以完成我们的任务了。

对于组合逻辑一般有两种实现方式：

- `assign` 赋值语句。

- `always` + **阻塞赋值**语句。

我们首先选择第一种方式。我们之前学过，`assign` 语句是对 `wire` 类型进行赋值的，在题目中我们需要对 out 进行赋值，所以 out 的类型应该为 `wire` 类型，而对于 `assign` 右侧的变量没有要求。其次，由于不同的 op 会对应不同的 out 取值，我们可以利用三目运算符来完成操作。建立如下模型。

```verilog
module assignALU(
    input [3:0] inA,
    input [3:0] inB,
    input [1:0] op,
    output [3:0] ans
    );

    // 利用三目运算符完成运算

     assign ans = (op == 2'b00) ? inA + inB :
                      (op == 2'b01) ? inA - inB :
                      (op == 2'b10) ? inA | inB :
                      (op == 2'b11) ? inA & inB :
                      4'b000; // error

endmodule
```

对于第二种方法，我们知道 `always@` 后面跟着的是**敏感变量**，如果某些值发生变化则执行 `always` 语句块。而对于 `always@(*)` 中的 `*`，敏感变量**由综合器根据 `always` 里面的输入变量自动添加**，不用自己考虑。

在 `always` 块中，我们可以选择利用 `case` 分支语句或者 `if` 分支语句对 op 进行判断，并利用 `always` 块保证对每次更新后的变量重新计算输出变量的值。**同时由于 `always` 块中赋值时左侧需要为 `reg` 类型，尽管我们知道实际电路 out 不会是寄存器，但是我们仍需要将其定义为 `reg` 类型。**而对于右侧对数据类型没有严格的要求，所以用默认的 `wire` 即可实现。

> ## verilog中reg型与wire型的区别：
>
> ## 赋值语句
>
> #### 连续赋值语句
>
> `wire`型数据只能被assign赋值，用以指定的组合逻辑信号。
> 如： `assign b = a；`
> [表达式](https://so.csdn.net/so/search?q=表达式&spm=1001.2101.3001.7020)右侧的计算结果可以立即更新到左侧，所以`wire`型数据需要持续的驱动，给wire型信号a逻辑值相当于通过导线。例：在组合逻辑电路中定义内部信号为wire型。
>
> #### 过程赋值语句
>
> `reg`型一般在`always` `initial`过程语句中
> 即使`always*(a or b or c)`敏感列表是此种类型,变量仍是reg型，综合出来为组合逻辑
>
> #### 输入输出
>
> 由于模块间的例化，对于输入信号，连接的上一级输出是组合逻辑输出还是寄存器输出(可由`wire`型/`reg`型驱动)，所以对于当前模块来说是`wire`型；而输出端口只能驱动wire型，自己决定是组合逻辑输出还是寄存器输出。
>
> ## 综合
>
> #### wire型变量
>
> 综合出来是一根导线，用来连接电路，这时易理解它没有驱动能力，不能存储值
>
> #### reg型变量
>
> reg型可综合成`register`(边沿触发)对应[触发器](https://so.csdn.net/so/search?q=触发器&spm=1001.2101.3001.7020),`latch`(电平触发)对应[锁存器](https://so.csdn.net/so/search?q=锁存器&spm=1001.2101.3001.7020),`wire`(作为中间变量)
>
> ## 仿真文件
>
> 待仿真文件输入类型信号需在仿真文件中设置为reg型；
> 待仿真文件输出类型信号需在仿真文件中设置为wire型；
> 可以这样理解：待仿真文件相当于一个黑盒，对黑盒进行测试，所以输入信号在仿真文件为reg型【数据源不断产生数据流】，输出信号在仿真文件中为wire型。
> 同时也说明待仿真文件的输入信号为wire型【双向端口】，输出信号为reg型
>
> 为什么待仿真文件的输入信号为wire型【双向端口】，输出信号为reg型？

虽然两种方法皆可以实现组合逻辑，但考虑接下来的任务，我们在此建议尽量**使用 `assign` 赋值**实现组合逻辑，而对 `always` + 阻塞赋值的实现方法做了解，只有在必要场景下使用后者：

1. **`assign` 赋值语句的结构清晰**：结合三目运算符的多层嵌套调用（如上文代码所示）可以非常简洁地实现对应的组合逻辑。
2. **`always` + 阻塞赋值的实现方法容易产生混淆**：方法容易与实现时序逻辑的非阻塞赋值混淆从而埋下问题，且实现的语句更为复杂。

因而我们建议：在不考虑综合后时序开销的情况下**尽量用 `assign` + 三目的形式实现组合逻辑**，从而保持 `always` 代码块中仅有非阻塞赋值语句。在考虑到时序开销的情况下（计组课设不对时序开销有任何严格约束），可以使用形如 `assign result = (is_A & A) | (is_B & B);` 的形式将连续三目表达式转换为并行的 MUX。

### 时序电路题目描述

题目说明：

小明正在学习数数，只有当完整的从 1 数到 3 才会得到正确的反馈（输出 1），其中不能插入任何数字，对数字 1 之前的数字没有任何要求。若不满足条件，则都会输出 0。端口定义如下：

![image-20220917211758449](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917211758449.png)

我们约定，在**每个时钟上升沿**的时候，状态机都会读入小明输入的数字，并对小明输入的数字进行判断，并确定是否输出正确的结果。

### 时序电路状态机分析：

这是一道 **Moore 型状态机**的题目。我们首先对状态机进行简单的分析，具体的细节在 Logisim 章节中有详细的讲解。首先我们可以判断一共存在 4 个状态 S0-S3，分别代表了没有读取到任何有意义的数字，读取了 1，读取了 12，以及读取了 123。当我们判断出读取了 123 后，将会输出 1，否则一直会输出 0。我们同时需要考虑处于任意一个状态时，输入了 1, 2, 3 时，状态将会如何转移。

![image-20220917211923530](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917211923530.png)

并将我们整理到的图转化为更容易表达的表格。

![image-20220917211955366](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917211955366.png)

当我们用 Logisim 处理时，需要分别去考虑状态的每一位前后的变化，并写出布尔表达式。但对于 Verilog，我们只用**判断出状态的转移**就可以了，不需要手动的写任何逻辑表达式。

### 时序电路例题实现过程

我们知道，对于时序逻辑，需要一个**寄存器去保存当前状态**，用 `reg` 类型进行存储。由于我们一共有四种状态，所以状态需要有至少两位进行保存。请注意，这里的 `reg` 类型与组合逻辑中利用 `always` 方法将 out 定义为 `reg` 类型有所不同，这里的状态在电路中就是一个寄存器。为了满足在“时钟上升沿”进行操作，我们需要在 `always` 中将敏感变量设置为 `posedge clk`，来满足在此时完成我们对状态进行更新的操作。更新操作需要结合 num 和当前状态进行更新，所以可以选择 `switch` 分支语句或者 `if` 分支语句进行操作。

同时我们需要考虑如何处理我们的 ans。在 Logisim 中，我们讨论的“输出”模块就是我们去计算 ans 的部分，对应于此，我们最好用**组合逻辑**对其进行赋值。这里我们利用 `assign` 语句+三目运算符进行判断，细节和之前讲述的组合逻辑很相似，不过这里的输入就变成了我们的“当前状态”这个变量。至此，我们的模型大体分为了两个部分，第一部分在 `always` 语句块中对状态进行更新，第二部分是在输出模块中利用 `assign` 对 ans 进行赋值。

代码具体如下：

首先我们可以使用宏定义对不同状态进行定义，用 `2'b00` 代表 S0，……，`2'b11` 代表 S3，方便我们以后对状态进行判断。

```verilog
`define S0 2'b00
`define S1 2'b01
`define S2 2'b10
`define S3 2'b11
module counting(
    input [1:0] num,
    input clk,
    input ans
    );

reg [1:0] status;
```

其次我们需要对状态变量进行**初始化**，否则其初始值为 `x`，无法进行以后的操作。

```verilog
initial
begin
    status <= `S0;
end
```

之后是我们的 `always` 块，这里只选取当状态为 S0 的判断。这里需要注意我们需要同时对状态以及输入的 num 进行判断，一定要使用非阻塞赋值防止不必要的 bug。这里注意我们 `always` 的敏感变量的选取。

```verilog
always@(posedge clk)
begin
    case(status)
        `S0: begin
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
                    status <= `S0; // 对于一切非正常输出，回到状态0
                end
            end
```

最后我们的输出模块，对 ans 根据当前的状态进行判断。这里和我们的表格存在一定的出入。表格中的 ans 为下一状态发生后的 ans 取值，也就是当当前状态为 S2，num 为 3 时，下一状态为 S3，同时一下个状态发生时我们的 ans 取 1。而在 Verilog 中由于是对当前的状态与 ans 进行分析，所以就当状态为 S3 时，ans 取 1。

```verilog
assign ans = (status == `S3) ? 1'b1 : 1'b0;
```

最后我们利用仿真查看结果。（具体仿真教程将会在下一节进行讲解）

![image-20220917212330187](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917212330187.png)

通过上述分析，我们的模块简单分为了 3 个部分，**初始化**，**状态更新**，**输出更新**，而分别利用 `initial` 块，`always` 块以及 `assign` 语句完成。而为了降低我们书写程序的复杂性，我们引用了**宏定义**的对不同状态进行了化简表达。

对于大部分的状态机，关键点在于寻找到所有的状态，并正确的完整状态迁移，同时要处理好 `always` 的敏感变量，剩下的书写部分并不存在很大的难度。

如果同学自己动手尝试，会发现这个模块无法进行综合，也就是不能得到我们目标电路。这是由于在 Verilog 中一些模块是**无法被综合**的，而我们使用的 `initial` 就是其中之一。如果我们想对其进行综合，可以引入 reset 输入，当其置 1 时，需要将状态机进行复位，也就是状态被赋值为 S0，而避免了手动初始化的尴尬局面，感兴趣的同学可以自己动手尝试一下。对于综合操作，在前 7 个 Project 中不会应用，而会在 P8 中进行应用。所以我们不需要强求一定要写出可以综合的模块出来。

其实我们可以发现，Verilog 作为硬件描述语言，我们目前是在行为级进行的描述，也就是描述模块完成了什么任务。通常是将模块分解为对应时序的 `always` 模块以及对应组合逻辑的 `assign` 赋值，并根据需求加入初始化的 `initial` 模块。我们并没有像 Logisim 中过分纠结使用具体什么电器元件，而是通过对模块进行抽象的行为层次的描述让 EDA 工具帮助我们综合出我们想要的电路。

最后我们书写完模块后，去验证模块的正确性需要利用 ISim 的仿真与调试功能，这将会在下一个章节进行讲述。

例程：

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

-----------------------------------------

## Verilog工程的设计开发调试

### 编写可综合的Verilog代码

下面，我们列出几条建议，帮助同学们编写出可综合的 Verilog 代码。需要注意的是，我们只需在我们编写的“模块”（即实际会被转换成硬件的那部分）中保证可综合性，而 Testbench 是顺序执行的（不会转换为硬件），编写 Testbench 时无需保证可综合性，**下面的建议不适用于 Testbench**。

#### 不要用 initial 块、不要为寄存器赋初值

`initial` 块用于在仿真开始时对寄存器进行初始化、执行其他代码。在综合时，`initial` 块会被忽略，不起任何作用，且为 reg 指定的初始值也会被忽略。也就是说，如下的代码都是不起作用的。

```verilog
reg v = 6; // 综合时，初始值被忽略
reg m;
initial begin
    m = 1; // 综合时，initial 块被忽略
end
```

如果你想在模块开始运行时，对寄存器进行一些初始化，请使用 reset 信号控制复位，并在 Testbench 开始的部分提供一个 reset 信号。例如，上面的代码正确写法为：

```verilog
always @(posedge clk) begin
    if (reset) begin
        v <= 6;
        m <= 1;
    end
end
```

Testbench 正确的写法：

```verilog
reg clk = 0;
reg reset = 0; // 只有在tb中可以直接赋初值
always #5 clk = ~ clk; // 创建周期为10的时钟
initial begin
    reset = 1
    # 10 // 延时一个时钟周期
    reset = 0;
    // 接下来开始你的测试
end
```

#### 一个寄存器只能在一个 always 块中赋值一次

Verilog 综合时，寄存器通常会被综合为 D 触发器：

![image-20220917215548536](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917215548536.png)

可以看到，D 触发器只有一个时钟输入、一个数据输入。因此，每个寄存器只能属于一个时钟域（“时钟域”指驱动触发器更新的时钟所表示的“范围”）。例如，以下代码会使 `a` 寄存器属于两个时钟域，因此是不可综合的：

```verilog
// 以下代码不可综合
reg a;
wire b, c;

always @(posedge clk_1) begin
    a <= b;
end

always @(posedge clk_2) begin
    a <= c;
end
```

除了注意时钟域的归属外，我们也需保证在每个时钟周期中，寄存器被至多赋值一次，不能重复赋值。例如，以下的代码是不可综合的：

```verilog
reg n;

always @(posedge clk) begin
    if (a)
        n <= 1'b1;

    // 一些其他代码

    n <= 1'b0;
end
```

需要注意的是“赋值一次”的含义。如果使用 `if` / `else` / `case` 语句进行条件判断，在**不同且互斥**的情况下对同一个寄存器进行赋值，是完全合法的。例如下面的代码：

```verilog
reg d;
wire a, b, c;

always @(posedge clk) begin
    if (c)
        d <= b;
    else
        d <= a;
end
```

虽然这里出现了两条对 `d` 进行赋值的语句，但这两条语句是“互斥”的，并不会对 `d` 重复赋值。上面的代码会被综合成如下的硬件：

![image-20220917215830947](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917215830947.png)

#### 组合逻辑相关注意事项

我们一般会将代码分为“时序逻辑”和“组合逻辑”。时序逻辑使用 `@(posedge clk)` 来表达，而组合逻辑使用 `@(*)` 来表达。在编写组合逻辑时，依照以下准则编写代码，可避免综合后产生奇怪的故障。

1. 在时序逻辑中，永远使用非阻塞赋值（`<=`）；在组合逻辑中，永远使用阻塞赋值（`=`）；
2. 每个组合逻辑运算结果仅在一个 `always @(*)` 中修改；
3. 在 `always @(*)` 中，为每个运算结果赋初值，避免 latch 的产生。

一段示例代码如下。

```verilog
// 注意以下 count_n 并不是一个寄存器，而是由组合逻辑生成的运算结果；count 才是实际存放计数值的寄存器。
reg [4:0] count_n, count;
wire add, set;
wire [4:0] set_value;

always @(*) begin
    count_n = count; // 修改了 count_n，因此先赋初值
    if (set)
        count_n = set_value;
    if (add)
        count_n = count_n + 1;
    // 阻塞赋值类似于 C 语言，按顺序执行，以最后赋值的为准
end

always @(posedge clk) begin
    if (reset)
        count <= 0;
    else
        count <= count_n;
end
```

> ## Verilog中避免Latch的产生：
>
> （输出结果为reg类型的要注意）
>
> ### Latch 的含义
>
> 锁存器（Latch），是电平触发的存储单元，数据存储的动作取决于输入时钟（或者使能）信号的电平值。仅当锁存器处于使能状态时，输出才会随着数据输入发生变化。
>
> 当电平信号无效时，输出信号随输入信号变化，就像通过了缓冲器；当电平有效时，输出信号被锁存。激励信号的任何变化，都将直接引起锁存器输出状态的改变，很有可能会因为瞬态特性不稳定而产生振荡现象。
>
> 锁存器示意图如下：
>
> ![image-20220917220556177](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917220556177.png)
>
> 触发器（flip-flop），是边沿敏感的存储单元，数据存储的动作（状态转换）由某一信号的上升沿或者下降沿进行同步的（限制存储单元状态转换在一个很短的时间内）。
>
> 触发器示意图如下：
>
> ![image-20220917220616243](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917220616243.png)
>
> 寄存器（register），在 Verilog 中用来暂时存放参与运算的数据和运算结果的变量。**一个变量声明为寄存器时，它既可以被综合成触发器，也可能被综合成 Latch，甚至是 wire 型变量。**但是大多数情况下我们希望它被综合成触发器，但是有时候由于代码书写问题，它会被综合成不期望的 Latch 结构。
>
> Latch 的主要危害有：
>
> - 1）输入状态可能多次变化，容易产生毛刺，增加了下一级电路的不确定性；
> - 2）在大部分 FPGA 的资源中，可能需要比触发器更多的资源去实现 Latch 结构；
> - 3）锁存器的出现使得静态时序分析变得更加复杂。
>
> Latch 多用于门控时钟（clock gating）的控制，一般设计时，我们应当避免 Latch 的产生。
>
> ### if 结构不完整
>
> 组合逻辑中，不完整的 if - else 结构，会产生 latch。
>
> 例如下面的模型，if 语句中缺少 else 结构，系统默认 else 的分支下寄存器 q 的值保持不变，即具有存储数据的功能，所以寄存器 q 会被综合成 latch 结构。
>
> **避免此类 latch 的方法主要有 2 种，一种是补全 if-else 结构，或者对信号赋初值。**
>
> 但是在时序逻辑中，不完整的 if - else 结构，不会产生 latch。
>
> 这是因为，**q 寄存器具有存储功能，且其值在时钟的边沿下才会改变**，这正是触发器的特性。
>
> 在组合逻辑中，当条件语句中有很多条赋值语句时，每个分支条件下赋值语句的不完整也是会产生 latch。
>
> 其实对每个信号的逻辑拆分来看，这也相当于是 if-else 结构不完整，相关寄存器信号缺少在其他条件下的赋值行为。
>
> 这种情况也可以通过补充完整赋值语句或赋初值来避免 latch。
>
> ### case 结构不完整
>
> case 语句产生 Latch 的原理几乎和 if 语句一致。在组合逻辑中，当 case 选项列表不全且没有加 default 关键字，或有多个赋值语句不完整时，也会产生 Latch。
>
> 当然，消除此种 latch 的方法也是 2 种，将 case 选项列表补充完整，或对信号赋初值。
>
> 补充完整 case 选项列表时，可以罗列所有的选项结果，也可以用 default 关键字来代替其他选项结果。
>
> ### 原信号赋值或判断
>
> 在**组合逻辑中**，如果一个信号的赋值源头有其信号本身，或者判断条件中有其信号本身的逻辑，则也会产生 latch。**因为此时信号也需要具有存储功能，但是没有时钟驱动。**此类问题在 if 语句、case 语句、问号表达式中都可能出现、例如：
>
> ```verilog
>  //signal itself as a part of condition
>     reg a, b ;
>     always @(*) begin
>         if (a & b)  a = 1'b1 ;   //a -> latch
>         else a = 1'b0 ;
>     end
>    
>     //signal itself are the assigment source
>     reg        c;
>     wire [1:0] sel ;
>     always @(*) begin
>         case(sel)
>             2'b00:    c = c ;    //c -> latch
>             2'b01:    c = 1'b1 ;
>             default:  c = 1'b0 ;
>         endcase
>     end
> 
>     //signal itself as a part of condition in "? expression"
>     wire      d, sel2;
>     assign    d =  (sel2 && d) ? 1'b0 : 1'b1 ;  //d -> latch
> ```
>
> 避免此类 Latch 的方法，就只有一种，即在组合逻辑中避免这种写法，信号不要给信号自己赋值，且不要用赋值信号本身参与判断条件逻辑。
>
> 例如，如果不要求立刻输出，可以将信号进行一个时钟周期的延时再进行相关逻辑的组合。上述第一个产生 Latch 的代码可以描述为：
>
> ```verilog
>  	reg   a, b ;
>     reg   a_r ;
>    
>     always (@posedge clk)
>         a_r  <= a ;
>        
>     always @(*) begin
>         if (a_r & b)  a = 1'b1 ;   //there is no latch
>         else a = 1'b0 ;
>     end
> ```
>
> ### 敏感信号列表不完整
>
> 如果组合逻辑中 always@() 块内敏感列表没有列全，该触发的时候没有触发，那么相关寄存器还是会保存之前的输出结果，因而会生成锁存器。
>
> 这种情况，把敏感信号补全或者直接用 always@(*) 即可消除 latch。
>
> ### 小结
>
> 总之，为避免 latch 的产生，在组合逻辑中，需要注意以下几点：
>
> - 1）if-else 或 case 语句，结构一定要完整
> - 2）不要将赋值信号放在赋值源头，或条件判断中
> - 3）敏感信号列表建议多用 always@(*)



#### 使用工具检查代码是否存在问题

在编写 Verilog 代码过程中，如果我们没有注意以上的要求，写出的代码虽然可以进行仿真，但可能无法综合为硬件，或者综合出不符合仿真结果的硬件。我们可以利用 ISE 或 VCS 对代码进行编译检查，以辅助我们找出可能存在的问题的代码。

在 ISE 中，编写完成代码后，运行 `Synthesize - XST`，将代码进行综合，正确编写的代码可以成功通过，而有问题的代码可能会报错。如下图所示，这是在两个 always 块中对同一个变量赋值的错误代码综合后 ISE 给出的报错信息，提示存在 `multiple drivers` 即多次赋值问题。

![image-20220917220105048](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917220105048.png)

在 ISE 中，如果代码成功通过综合，可以通过`Synthesize - XST / View RTL Schematic` 功能来查看综合出来的硬件电路图，可以直观地观察到代码对应的硬件结构。

由于 VCS 不是综合工具，暂无法提供类似功能。后续我们会提供相应的工具进行可综合性测试。

#### 不要使用乘除法 - 用位运算来代替乘除法

FPGA 及 ASIC 硬件中，实现乘法和除法的代价是较高的，需要专门的乘法器、除法器，逻辑门的数量较多。在编写 Verilog 代码时，若非必要，则不要使用乘除法。

**很多乘除法操作可以使用移位或位拼接来代替。**移位运算在硬件中实现非常直接，使用的逻辑门数量也较少。使用位移运算代替乘除法的方法如下：

- 乘以2的n次方：
  - 左移 n*n* 位，例如：`a * 8` 可替换为 `a << 3`；
  - 在变量后面拼接 n*n* 个 00，例如 `a * 8` 可替换为 `{a, 3'b0}`。
- 除以2的n次方：
  - 右移 n*n* 位，例如 `a / 8` 可替换为 `a >> 3`；
  - 取变量的高位，例如 `a / 8` 可替换为 `a[7:3]`（若 `a` 一共有 8 位）。
- 求模2的n次方 - 取 n位以后的低位，例如：`a % 8` 可以替换为 `a[2:0]`。

在使用移位运算符时，请注意移位运算符的优先级问题。

### Verilog代码规范

#### 命名

##### VC-001 信号名称采用 `snake_case`，`PascalCase` 或者 `camelCase`

- `snake_case`，即变量名全小写，单词之间用下划线分隔。
- `PascalCase`，即单词的首字母大写以区分单词。
- `camelCase`，即变量第一个单词小写，后续单词首字母大写。

例如：

```verilog
module DM(
  input wire mem_write, //写入使能信号
  //或者
  input wire MemWrite, //写入使能信号
  //或者
  input wire memWrite, //写入使能信号
  ...//省略其他信号
);
...//省略内容
endmodule
```

无论使用何种方法，请统一整个工程的命名方式。

##### VC-002 信号极性为低有效用 `_n` 后缀表示

对于复位和使能信号，例如 `rst` 和 `we`，如果添加了 `_n` 后缀，表示值为 0 时生效（低有效，Active Low），值为 1 时不生效。

如果没有添加 `_n` 后缀，表示值为 1 时生效（高有效，Active High），值为 0 时不生效。详细解释见下面的表格：

![image-20220917225023830](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917225023830.png)

当代码中需要混合使用 `rst` 和 `rst_n` 的时候，采用以下的方式来转换：

```verilog
module test(
  input rst_n
);
  wire rst;

  // GOOD
  assign rst = ~rst_n;

  // GOOD
  // Verilog
  always @(*) begin
    rst = ~rst_n;
  end
endmodule
```

##### VC-003 多路选择器标明规格

后续开发中可能会使用多种多路选择器，表明多路选择器的规格对于代码可读性的提升有显著提升。 例如，4 选 32 位 1 的 `MUX`，模块的命名如下：

```verilog
module Mux4_1_32(
  input wire[1:0] sel,
  input wire[31:0] in0,
  input wire[31:0] in1,
  input wire[31:0] in2,
  input wire[31:0] in3,
  output wire[31:0] out
);
```

##### VC-004 魔数的命名

编写状态机的时候，各个状态一定要命名，之后调用的时候使用命名，而不是数字，减少代码中 magic number 的出现。建议使用 `parameter`，`localparam` 或者宏定义命名。 例如：

```verilog
// GOOD
localparam sInit = 2'd0;
// or
parameter sInit = 2'd0;
// or
`define sInit 2'd0
```

如果仿真工具不支持在波形中显示为对应的状态名称，可以采用以下的方法：

```verilog
`ifndef SYNTHESIS
  reg [39:0] state_string; // 40 bits = 5 byte

  always @ (*) begin
    case(state)
      sInit: state_string = "sInit";
      sIdle: state_string = "sIdle";
      sWork: state_string = "sWork";
      sDone: state_string = "sDone";
      default: state_string = "?????";
    endcase
  end
`endif
```

此时在仿真波形中，`state_string` 信号就可以看到状态的名称了。

另外，之后 P4 的学习，涉及到对指令的解析，也尽量使用宏定义等方式代替数字。 例如：

~~~verilog
// GOOD
`define swop 6'b101011
...
assign sw = (`swop == op);

// NOT GOOD
assign sw = (6'b101011 == op);
```endcase
~~~

#### 组合逻辑的编写

##### VC-005 信号仅在一个 always 块中赋值

通常情况下，一个信号只会在一个 `always` 块中赋值。如果一个信号在多个 `always` 块中赋值，其结果是不可预测的。

##### VC-006 组合逻辑采用 `always @(*)` 块或者 `assign` 编写

组合逻辑的 `always` 块，使用以下的写法：

```verilog
// Verilog
always @(*) begin
  c = a + b;
end

// GOOD
assign c = a + b;
```

##### VC-007 组合逻辑 `always` 块中仅使用阻塞赋值

表示组合逻辑的 `always` 块中所有的赋值请使用阻塞赋值（`=`）。使用非阻塞逻辑并不能模拟出实际组合逻辑的行为.

##### VC-008 组合逻辑 `always` 块中保证每个分支都进行赋值

如果组合逻辑中存在分支没有被赋值，那么实际综合中会综合出锁存器，FPGA 的底层基本组件并不存在锁存器，锁存器是使用查找表和寄存器组成，资源开销大。因此，如果使用了条件语句 `if` 或者 `switch`，需要保证信号在每个可能的分支途径下都进行了赋值，其中 `switch` 语句一定要写 `default` 分支，并对信号赋值。

```verilog
// GOOD
always @(*) begin
  if (reset_n) begin
    c = a + b;
  end else begin
    c = 1'b0;
  end
end

// BAD
always @(*) begin
  if (reset_n) begin
    c = a + b;
  end
end

// GOOD
  always @(*) begin
    case(status)
     `S0:
      begin
        wire1 = //...
        wire2 = //...
      end
      //...
      default:
      begin
        wire1 = //...
        wire2 = //...
      end
    endcase
  end

// BAD
  always @(*) begin
    case(status)
     `S0:
      begin
        wire1 = //...
        wire2 = //...
      end
      //... 
      `S7:
      begin
        wire1 = //...
      end
    endcase
  end
```

另外，组合逻辑的 `always` 块中不要列举敏感信号。

```verilog
// BAD
always @ (b, c) begin
  a = b + c;
end
```

#### 时序逻辑的编写

##### VC-009 时序逻辑在 `always @(posedge clock)` 中实现

当需要表示时序逻辑时，不能使用组合逻辑的写法，一定使用以下的写法：

```verilog
// Verilog
always @(posedge clock) begin
  c <= a + b;
end
```

##### VC-010 时序逻辑 `always` 块中仅使用非阻塞赋值

时序逻辑 `always` 块中，所有的赋值请使用非阻塞赋值（`<=`）。

##### VC-011 不要使用下降沿触发，特殊协议除外

通常情况下，请不要使用下降沿触发：

```verilog
// BAD: do not use negedge
always @ (negedge clock) begin
end
```

##### VC-012 不要使用非时钟 / 复位信号的边沿触发

通常情况下，不要使用除了时钟和复位以外的信号做边沿触发。

```verilog
// BAD: do not use non-clock/reset signals
always @ (posedge signal) begin
end
```

##### VC-013 时序逻辑中不要使用时钟信号

在时序逻辑中，请不要在敏感列表以外的地方使用时钟信号：

```verilog
// BAD
always @ (posedge clock) begin
  if (clock) begin
    a <= 1;
  end
end
```

##### VC-014 使用同步复位，而不是异步复位

对于 FPGA，请使用同步复位，因为异步复位容易受到毛刺的影响。 代码如下：

```verilog
// Verilog
always @(posedge clock) begin
  if (reset) begin
    c <= 1'b0;
  end else begin
    c <= a + b;
  end
end
```

当然，如果题目要求那就按照题目要求来写。

#### 模块的编写和实例化

##### VC-015 不要在内部模块中使用 `inout`

FPGA 内部的模块之间请不要使用 `inout`，仿真环境除外。

##### VC-016 模块内部变量的定义和声明尽量统一

对于大模块的开发会使用大量的 `wire` 和 `reg` 变量，这些变量尽量统一在一个地方定义，后续增量开发在相应地方添加变量，而不是在需要使用的时候在旁边随手定义。另外，以免出现调用未声明变量的问题，可以添加宏定义 `default_nettype`。

```verilog
// GOOD
  wire RegWrite, MemtoReg, MemWrite, Busy, MdutoGRF;
  wire [5:0] ALUop;
  wire [4:0] Rs, Rt, Rd, WriteReg, shamt, ExcCode, ExcCodetmp, OvResult;
  wire [31:0] Imm, RD1, RD2, SrcA, SrcB, ALUResult, WriteData, PC;
// Bad
  wire RegWrite;
  //...... 省略代码
  wire [31:0] Imm, RD1, RD2, SrcA, SrcB;
  //...... 省略代码
```

##### VC-017 模块的实例化多换行

后期开发中，一个模块可能有几十个端口。如果模块实例化的时候，端口挤在一起，可读性就会非常差，建议一个端口换一行，`input` 和 `output` 分开写，例如：

```verilog
// 模块实例化
// GOOD
    test int_test(
        .clk(clk),
        .reset(reset),
        .addr(addr),
        .we(we),
        .data_in(data_in),
        .data_out(data_out)
    );
// NOT GOOD
    test int_test(.clk(clk),.reset(reset),.addr(addr),.we(we),.data_in(data_in),.data_out(data_out));
```

#### 代码风格

##### VC-018 符号两侧空格部分规则

- 单目运算符与变量间不添加空格，如 `~`、`!`。
- 双目运算符（除逗号外）和三目运算符两侧添加空格，如 `+`、`=`、`<`、`&&`。
- 分号和逗号要紧附前面内容，不应添加空格。
- 避免连续使用多个空格。

```verilog
// GOOD
assign d =  a + {1'b0, b};
// BAD  
assign d=a+{1'b0,b};
```

##### VC-019 换行的使用

- 对于不同逻辑的代码块建议换行分开，并加上相应注释区分
- 对于同一逻辑，但是表达式复杂的语句，使用换行进行语义分割，如

```verilog
// GOOD
assign d = (op == 0) ? a + b :
           (op == 1) ? a - b :
           (op == 2) ? a & b :
                       a | b;
// BAD
assign d = (op == 0) ? a + b :(op == 1) ? a - b :(op == 2) ? a & b : a | b;
```

##### VC-020 模块缩进

对于对称的关键字，比如 `begin & end`，`case & endcase` 等，采用缩进的方式对其进行**优化对齐**。

```verilog
// GOOD
always @(*)begin
  if ()  begin
  end
end
// BAD
always @(*)begin
if ()  begin
end
end
```

##### VC-021 显式声明位宽

关于数据位宽有两点说明：

- 使用常数时，**声明数据位宽**，避免连线时出现位宽不一致的 Bug。
- 如果模块要使用数据的某一部分位，如 `instruction[25:21]`，使用 `wire` 变量直接截取，重新赋予一个合理的命名，减少字面量以增强可读性。

##### VC-022 模块抽象

对重复使用的复杂代码进行抽象，而不是简单的复制粘贴

### Verilog开发与调试综述：

- **需求分析：**使用文档辅助工具协同进行架构设计

- 需求实现：

  根据设计文档高效完成模块代码编写

  - 编写代码
  - 关注代码风格
  - 浅谈编辑器

- 仿真与调试：

  对已有模块进行测试

  - 生成 Testbench
  - 使用 ISim
  - 错误样例

- **综合工程：**将抽象逻辑转化为实际电路

### 需求：简单 32 位 ALU

**设计要求：**

![image-20220917230840631](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917230840631.png)

- ALU 模块需要支持加、减、与、或运算。
- 每个时钟上升沿到来时，如果使能信号有效，则更新运算结果，否则保持不变。
- 在两个时钟上升沿之间，保持输出结果不变。
- 对模块进行充分仿真测试。

### 需求分析

我们希望大家在拿到一道题目，尤其是比较复杂的工程化项目时，不要急于上手写代码，而要预先根据需求进行**架构设计**。这一点在后续的**多模块开发**过程中尤为重要，是奠定工程整体质量的关键步骤。

**端口定义**

本道题目已经对模块的输入和输出端口进行了约束，我们只需要对端口合理命名后整合到文档中即可。

![image-20220917231100272](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917231100272.png)

**组合逻辑**

ALU 模块首先要并行计算出四种运算的结果，然后需要通过 op 输入信号的选择来确定每个周期的输出数据，在前面的教程中，我们了解到，在组合逻辑中对多个信号进行选择可以使用多路选择器，由此我们将其都加入设计文档中。

![image-20220917231243072](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917231243072.png)

**时序逻辑**

我们需要在在每个时钟信号的上升沿更新输出的值，也就意味着我们的模块需要具备存储功能，由此引入时序逻辑。然后要判断使能信号有效性，再进一步决定输出。

![image-20220917231321292](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220917231321292.png)

很多时候，我们都需要反复斟酌一个模块存在的必要性。将复用频繁的电路独立抽离为一个模块，将功能简单且仅在一个地方使用的电路融入到其父级模块中。避免过度设计，同时又要有合理的抽象，在设计时需要认真取舍。

希望大家能够在后续越来越复杂的 Project 中养成先进行架构设计再编码实现的习惯，好的工程架构往往能够令我们事半功倍。

### 需求实现：编写代码

现在我们已经设计好了 ALU 模块的**输入输出端口**、**组合逻辑**与**时序逻辑**三部分内容，可以开始着手用代码将设计实现。

在前面的章节中，我们已经了解了 Verilog 的基本语法、题目编写的方法，这部分便不再赘述。

计小组同学对照自己刚刚写好的设计文档，利用已经学习到的知识，很快完成了编码：

```verilog
`timescale 1ns / 1ps

module alu(
    input [31:0] a,
    input [31:0] b,
    output reg [31:0] c,
    input clk,
    input en,
    input [1:0] op
    );

wire [31:0] d;
assign d=op==0?a+b:op==1?a-b:op==2?a&b:a|b;
always @(posedge clk) begin
if (en) begin
c<= d;
end
else begin
c<=c;
end
end
endmodule
```

相信同学们也注意到，计小组同学的代码非常难以阅读，这也引出了 Verilog 的代码风格问题，我们将在下一节介绍。

### 需求实现：关注代码风格

下面我们将对计小组同学的 ALU 代码进行改善，使其的可读性大大增加。

#### 符号两侧空格部分规则

多数代码规范对运算符两侧的空格有明确的约束，在此只列举一部分较为影响代码观感的进行说明。

- 单目运算符与变量间不添加空格，如 `~`、`!`。
- 双目运算符（除逗号外）和三目运算符两侧添加空格，如 `+`、`=`、`<`、`&&`。
- 分号和逗号要紧附前面内容，不应添加空格。
- 避免连续使用多个空格。

根据这几点规则，我们可以将 `assign` 语句改成如下样式：

```verilog
assign d = op == 0 ? a + b : op == 1 ? a - b : op == 2 ? a & b : a | b;
```

#### 换行的使用

修改后的 `assign` 语句已经没有了拥挤的感觉，但很明显仍然不适合阅读——我们很难迅速地判别出每个条件所对应的语句，因此需要考虑在合适的地方进行**换行处理**。

在不违背语法规则的前提下，推荐在两个较高级的表达式间换行，具体情况具体分析，保证换行后最适宜阅读逻辑为最佳。

```verilog
assign d = (op == 0) ? a + b :
           (op == 1) ? a - b :
           (op == 2) ? a & b :
                       a | b;
```

此外，为了代码阅读者更高效地理解代码，可以增加括号来使同一行代码间降低耦合度；为了代码的美观性，也可以使用空格令各行代码对齐；对于处理不同逻辑的不同代码块，尽可能使用空行将其分开，对自身调试、他人阅读都有很大的帮助。

#### 模块缩进

计小组的代码还有一个较为突出的风格问题，那就是最后的几行几乎堆砌满了 `end` 相关的语句，直观上无法很快定位每一个 `end` 所对应的语句块，因此采用缩进的方式对其进行**优化对齐**。

经过一定的优化后，代码不再有拥挤的感觉：

```verilog
`timescale 1ns / 1ps

module alu (
    input [31:0] a,
    input [31:0] b,
    output reg [31:0] c,
    input clk,
    input en,
    input [1:0] op
    );

    wire [31:0] d;
    assign d = (op == 0) ? a + b :
               (op == 1) ? a - b :
               (op == 2) ? a & b :
                           a | b;

    always@(posedge clk) begin
        if (en) begin
            c <= d;
        end
        else begin
            c <= c;
        end
    end

endmodule 
```

#### 合理命名

作为一名测试者，如果代码中充满了没有任何意义的 “a”、“b” 样式的命名，毫无疑问会给调试增加不小的阻碍，你甚至不知道出现 bug 的端口或变量的作用是什么。

关于变量与端口命名，有如下建议：

- 符合其自身的功能，尽量做到“**望文生义**”。
- 杜绝拼音与英文的命名混用，推荐使用英文单词的缩写，使名称长度合理。
- 不同单词间有明显的边界划分，如使用下划线，常见的“大驼峰”、“小驼峰”命名法等。

#### 添加注释

注释的作用就不需要再做过多的赘述了，其可以将抽象的计算机语言转化为人类交流所依赖的语言，能够很大程度上提高代码的阅读效率，快速帮助编者与读者理清各部分代码的作用。

#### 常量定义

一些使用或改动频繁的字面量往往可以使用**宏或 `parameter`**（下一章节将介绍宏的使用）等 Verilog 语法特性进行定义，在增量开发时保证最快寻找到定义位置，且对代码的改动最小，如我们后面设计 CPU 中的指令存储器模块时：

```verilog
parameter ROM_MAX = 4096;
reg [31:0] rom[ROM_MAX - 1, 0];
```

而在多模块开发中，我们可以专门编写一个宏定义的头文件，使用 ``include`，方便**其他所有的模块**跨文件引用其中的常量。

#### 数据位宽

关于数据位宽有两点说明：

- 使用常数时，**声明数据位宽**，避免连线时出现位宽不一致的 Bug。
- 如果模块要使用数据的某一部分位，如 `instruction[25:21]`，使用 `wire` 变量直接截取，重新赋予一个合理的命名，减少字面量以增强可读性。

#### 避免复制粘贴代码

我们提出这部分，并不是在说**不要复制别人的代码**（那样的后果想必大家都清楚），而是在说**不要复制粘贴自己的代码**。

在后续的开发中，大家可能会遇到非常多带有重复部分的代码段，于是想减少一下编写的工夫，对其进行复制粘贴，然后再做微小调整。这样做会让粘贴的代码带有极大的隐患，你很可能因为看错某个变量的一个字母，或者粘贴到了重复度高的错误位置没有察觉，导致工程出现难以预料的 Bug。

一个规避这种风险的方法是，对想要复制的代码块进行抽象，独立为一个模块或者使用宏对其定义，然后谨慎编写不同的部分。

计小组同学对照自己刚刚编写的代码，对代码风格的重要性开始有初步的意识了，于是她完成了 ALU 代码风格的优化，并开始准备进行测试工作。

```verilog
`timescale 1ns / 1ps

module alu(
    input [31:0] input_a,
    input [31:0] input_b,
    input [1:0] op,
    input clk,
    input en,
    output reg [31:0] result
);

    wire [31:0] temp_result;

    assign temp_result = (op == 2'd0) ? (input_a + input_b) :
                         (op == 2'd1) ? (input_a - input_b) :
                         (op == 2'd2) ? (input_a & input_b) :
                                        (input_a | input_b);

    always @(posedge clk) begin
        if (en == 1'b1) begin
            result <= temp_result;
        end
        else begin
            result <= result;
        end
    end

endmodule
```

### 仿真与调试：生成 Testbench

FPGA 的基本开发流程可分为**功能定义与器件选取**、**设计输入**、**功能仿真**、**综合优化**、**综合后仿真**、**实现与布局布线**、**时序仿真**和**板级运行验证**，在 P8 之前，我们只会接触到前三个步骤。

Testbench 并不一定需要自动生成，依照给出的项目模板来手动编写 Testbench 也是非常简单的。

#### **解析测试文件**

下面我们对生成的 alu_tb 模块进行解析。

```verilog
module alu_tb;

    // Inputs
    reg [31:0] input_a;
    reg [31:0] input_b;
    reg [1:0] op;
    reg clk;
    reg en;

    // Outputs
    wire [31:0] result;

    // Instantiate the Unit Under Test (UUT)
    alu uut (
        .input_a(input_a), 
        .input_b(input_b), 
        .op(op), 
        .clk(clk), 
        .en(en), 
        .result(result)
    );

    initial begin
        // Initialize Inputs
        input_a = 0;
        input_b = 0;
        op = 0;
        clk = 0;
        en = 0;

        // Wait 100 ns for global reset to finish
        #100;
```

- 标注有“Inputs”和“Outputs”注释的地方，是我们模块输入输出端口的转化，**其中仿真模板将输入用`reg`变量替代，便于我们直接对其值进行设置。**
- 下面标注有“uut”的部分是对我们要测试的模块进行实例化，具体含义将在后文进行讲解。
- **`initial`语句块**是我们**需要修改的部分**，使用关键字“#”开头的延迟控制语句进行**时间控制**，将输入端口在不同的时间赋予我们期望的数据。请注意，该语句声明的是**延迟时间**，而不是整个仿真过程的时间戳。

Testbench 本质与一个模块相同，我们也可以为其添加临时变量，组合逻辑等内容来辅助仿真。

#### **编写测试逻辑**

我们开始编写测试逻辑，当模块包含时钟信号这类以固定频率变化的输入时，我们要使用 `always`，如我们欲将时钟周期设置为 10ns，可以写 `always #5 clk = ~clk;`。

```verilog
    initial begin
        // Initialize Inputs
        input_a = 0;
        input_b = 0;
        op = 0;
        clk = 0;
        en = 0;

        // Wait for 10 ns ans set en to 1
        #10;
        en = 1'b1;

        // Test Add amd Sequential Logic
        input_a = 32'd19260817;
        input_b = 32'd99999999;

        // Test Sub and Whether Will Change within Clock High Level
        #8;
        op = 2'd1;

        // Test when en is 0, Whether Result Will Change
        #12;
        en = 1'b0;
        input_a = 32'd100;

    end

    always #5 clk = ~clk;
endmodule
```

上图对加法、减法、使能信号、时序逻辑进行了简单的测试，接下来我们便可以用 ISim 进行仿真了。

#### 模块实例化

我们将在这部分专门对**模块实例化**进行讲解。

在 Verilog 中，我们编写的模块都是抽象的逻辑，如果要将其交由**其他父模块调用**，就需要进行模块的**实例化**。简而言之，实例化就是抽象转化为具体的过程。

##### 模块实例化的方法

对电路元件模块实例化最常见的语法是：`<模块名> <实例名>(端口信号映射);`。

其中，端口信号映射方式也有两种：

- **位置映射：**`<模块名> <实例名>(信号 1, 信号 2, ...);`，其中信号 n 对应被实例化模块声明时排在第 n 位的端口。
- **名映射：**`<模块名> <实例名>(.端口名 a(信号 1), .端口名 b(信号 2), ...);`，其中信号 n 对应其前的端口名。

值得注意的是，在实例化元件时，`wire` 类型信号可以被连接至**任意端口**上，但 `reg` 类型的信号只能被连接至元件的**输入端口**上。在声明元件时，我们可以将任意端口声明为 `wire` 类型，但只能将输出端口声明为 `reg` 类型，否则会出现问题。

我们也可以悬空部分不需要连接的端口，下图的 uut_0、uut_1、uut_2 分别对应位置映射、名映射与悬空端口的实例。建议**每一行只连接一个端口信号**避免混乱。

```verilog
// Instance the Unit Under Test (UUT)
alu uut_0 (
    input_a,
    input_b,
    op,
    clk,
    en,
    result
);

alu uut_1 (
    .input_a(input_a),
    .input_b(input_b),
    .op(op),
    .clk(clk),
    .en(en),
    .result(result)
);

alu uut_2 (
    .input_a(input_a),
    .clk(clk),
    .result(result)
);
```

### 仿真与调试：错误样例

#### 部分仿真bug与调试方法

本部分将列举一些设计中会出现的 Bug，并给予解决方法。

**波形出现不定值 x：**这种情况在没有 `initial` 语句的情况下会出现在 `reg` 类变量的波形中，通常在第一次对 `reg` 变量修改后便会消失，可以添加 `initial` 语句对 `reg` 赋予初始值来避免。但如果整个仿真的过程中，某个 `reg` 变量一直维持不定值，需要检查时序逻辑的编写是否正确，是否有按照架构对相应变量进行赋值。

![image-20220918102933243](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918102933243.png)

**波形出现高阻 z：**电路存在没有连线的变量信号，会显示高阻。需要返回到代码中检查相应 `wire` 变量是否被正确的 `assign` 语句赋值，`reg` 变量连接的 `wire` 变量是否正确连线。

![image-20220918103012959](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918103012959.png)

### 综合工程

**综合**，是将硬件描述语言，翻译为物理电路的过程。

**Tips：**综合的速度普遍较慢，当工程规模较大时综合甚至会花费数小时。我们还需要注意语法检查通过并不代表综合能够通过。如何写出可综合的 Verilog 代码，将是未来学习过程中必须面对的问题。当然，为了前面 Project 能够有效通过测试，仍需要在抵达 P8 之前保留一部分不可综合的代码。

与仿真不同的常见综合化要求包括但不限于：

- 不使用 `initial`、`fork`、`join`、`casex`、`casez`、延时语句（例如 `#10`）、系统任务（例如 `$display`）等语句，具体可自行查阅学习。
- 用 `always` 过程块描述组合逻辑时，应在敏感信号列表中列出所有的输入信号（或使用星号*）。
- 用 `always` 过程块描述时序逻辑时，敏感信号只能为**时钟信号**。
- 所有的内部寄存器都应该能够被复位。
- 不能在一个以上的 `always` 过程块中对**同一个变量**赋值。而对同一个赋值对象不能既使用阻塞式赋值，又使用非阻塞式赋值。
- 尽量避免出现锁存器（latch），具体避免方法有许多。例如，如果不打算把变量推导成锁存器，那么必须在 `if` 语句或 `case` 语句的所有条件分支中都对变量明确地赋值。
- 避免混合使用上升沿和下降沿触发的触发器。
- ……

还有一些其他的细节要求，具体可以自行查阅学习。

---------------------------------------

## Verilog高级特性与自动化测试

### 编译预处理

Verilog HDL 语言和 C 语言一样也提供了编译预处理的功能。所谓编译预处理，可以类比 C 语言中的 `#define` 等语句。例如：

```verilog
#define BUFSIZE 256
char buf[BUFSIZE];
```

Verilog HDL 语言和 C 语言一样也提供了编译预处理的功能。在 Verilog HDL 语言中，为了和一般的语句相区别，这些预处理命令**以符号 ```（反引号，backtick）开头**。（位于主键盘左上角，其对应的上键盘字符为 `~`。注意这个符号不同于单引号。）这些预处理命令的有效作用范围为定义命令之后到本文结束或到其他命令定义替代该命令之处。由于计组实验中用到的预处理命令不多，所以在此仅对 ``define`, ``timescale` 做相应介绍。

1. 宏定义 `define

用一个指定的标识符(即名字)来代表一个字符串，它的一般形式为：

```verilog
`define 标识符(宏名) 字符串(宏内容)
```

如：

```verilog
`define signal string
```

它的作用是指定用标识符 signal 来代替 string 这个字符串，在编译预处理时，把程序中该命令以后所有的 signal 都替换成 string。

例如:

```verilog
`define WORDSIZE 8
module test reg[1:`WORDSIZE] data;
//相当于定义 reg[1:8] data;
endmodule
```

注意在引用宏名时也要必须在宏名前加上符号 ``` 表明该名字是经过宏定义的名字。

> 注意：编译器在编译时，会将宏“展开”（即替换）为对应的字符串。如果宏的内容较长，且展开次数较多（在代码中使用宏的次数较多），会带来较大的开销，导致仿真编译较慢，在线评测时可能会编译超时。因此，如果是希望定义类似“变量”的东西，在可以使用 `wire` 时，请使用 `wire`，而不是使用宏。

2. 文件包含 `include

所谓"文件包含"处理是一个源文件可以将另外一个源文件的全部内容包含进来，即将另外的文件包含到本文件之中。Verilog HDL 语言提供了 ``include` 命令用来实现"文件包含"的操作。其一般形式为:

```verilog
`include "文件名"
```

在编译的时候，需要对 ``include` 命令进行"文件包含"预处理：将 File2.v 的全部内容复制插入到 ``include "File2.v"` 命令出现的地方，即使 File2.v 被包含到 File1.v 中。在接着往下进行编译中，将"包含"以后的 File1.v 作为一个源文件单位进行编译。

例如：

```verilog
// aaa.v
module aaa(a, b, out);
  input a, b;
  output out;
  wire out;
  assign out = a ^ b;
endmodule

// bbb.v
`include "aaa.v"
module bbb(c, d, e, out);
  input c, d, e;
  output out;
  wire out_a;
  wire out;
  aaa aaa(.a(c), .b(d), .out(out_a));
  assign out = e & out_a;
endmodule
```

在经过编译预处理后，文件bbb.v被处理为：

```verilog
// bbb.v
module aaa(a, b, out);
  input a, b;
  output out;
  wire out;
  assign out = a ^ b;
endmodule
module bbb(c, d, e, out);
  input c, d, e;
  output out;
  wire out_a;
  wire out;
  aaa aaa(.a(c), .b(d), .out(out_a));
  assign out = e & out_a;
endmodule
```

关于"文件包含"处理的四点说明：

1. 一个 ``include` 命令只能指定一个被包含的文件，如果要包含 n 个文件，要用 n 个 ``include` 命令。注意下面的写法是非法的

   ```verilog
   `include"aaa.v""bbb.v"
   ```

2. ``include` 命令可以出现在 Verilog HDL 源程序的任何地方，被包含文件名可以是相对路径名，也可以是绝对路径名。例如：

   ```verilog
   `include"parts/count.v"
   ```

3. 可以将多个 ``include` 命令写在一行，在 ``include` 命令行，只可以出空格和注释行。例如下面的写法是合法的。

   ```verilog
   `include "fileB" `include "fileC" // including fileB and fileC
   ```

4. 如果文件 1 包含文件 2，而文件 2 要用到文件 3 的内容，则可以在文件 1 用两个 ``include` 命令分别包含文件 2 和文件 3，而且文件 3 应出现在文件 2 之前。

**补充：宏文件的引用**

在具体代码实践中，我们定义的宏常常在多个文件中都需要使用。如果在每个文件中都采用复制粘贴的方式，那么在修改的时候就会有很大的工程量，所以我们常常单独开一个文件进行宏的定义，在其他文件中对这个“宏文件”进行引用。

```verilog
// macro.v
`define WORDSIZE 8

// test1.v
`include "macro.v"

module test1 reg[1:`WORDSIZE] data;
  // ...
endmodule

// test2.v
`include "macro.v"

module test2 reg[1:`WORDSIZE] data;
  // ...
endmodule
```

`macro.v`是我们定义的一个“宏文件”，在其他需要使用到这个“宏文件”的文件头部，使用`include "macro.v"`就可以使用“宏文件”中定义的宏了。

3. 时间尺度 `timescale

``timescale` 命令用来说明跟在该命令后的模块的时间单位和时间精度。使用 ``timescale` 命令可以在同一个设计里包含采用了不同的时间单位的模块。例如，一个设计中包含了两个模块，其中一个模块的时间延迟单位为纳秒 (ns)，另一个模块的时间延迟单位为皮秒 (ps)。EDA 工具仍然可以对这个设计进行仿真测试。

``timescale` 命令的格式如下：

```verilog
`timescale[时间单位]/[时间精度]
```

例如：

```verilog
`timescale 1ns/1ps;
```

在这个命令之后，模块中所有的时间值都表示是 1ns 的整数倍。这是因为在 ``timescale` 命令中，定义了时间单位是 1ns。模块中的延迟时间可表达为带3位小数的实型数，因为 ``timescale` 命令定义时间精度为 1ps。

4. 条件编译命令 `ifdef, `else, `elsif, `endif, `ifndef

和 C 语言类似，这些条件编译编译指令用于包含 Verilog HDL 的可选行编译期间的源描述。 ``ifdef` 编译器指令检查 text_macro_name 的定义，如果定义了 text_macro_name，那么 ``ifdef` 指令后面的行被包含在内。如果未定义 text_macro_name 并且存在 ``else` 指令，则编译 ``else` 后的源描述。

``ifndef` 编译器指令检查 text_macro_name 的定义。如果未定义 text_macro_name，则包含 ``ifndef` 指令后面的行。如果定义了 text_macro_name 并且存在 ``else` 指令，则编译 ``else` 后的源描述。如果 ``elsif` 指令存在（注意不是 ``else`），编译器会检查 text_macro_name 的定义。如果定义存在，则包含 ``elsif` 指令后面的行。

``elseif` 指令等同于编译器指令序列 ``else `ifdef ...`endif`。该指令不需要相应的 ``endif`指令。该指令必须以 ``ifdef` 或 ``ifndef` 指令开头。

例如：

```verilog
module and_op (a, b, c);
output a;
input b, c;
`ifdef behavioral
  wire a = b & c;
`else
  and a1(a, b, c);
`endif
endmodule
```

### 系统任务

Verilog 中还提供了很多系统任务，类似于 C 中的库函数，使用这些系统任务可以方便地进行测试。由于计组实验中用到的系统任务相对较少，所以在此仅对 `$display`, `$monitor`, `$readmemh` 进行介绍。

#### 输出信息

格式：`$display(p1, p2, ..., pn)`;

这个系统任务的作用是**用来输出信息**，即将参数 p2 到 pn 按参数 p1 给定的格式输出。用法和 C 语言中的 printf 类似。下面用一个例子简单介绍其用法。

```verilog
module disp;
    reg[4:0] a;
    reg[4:0] b;
    initial begin
        a = 10;
        b = 20;
        $display("a = %d,b = %d\n",a,b);
    end
endmodule
```

其输出结果为： a = 10,b = 20

其中 `%d` 表示以十进制的形式输出，`\n` 为换行符。

在此说明几种常用的输出格式：

![image-20220918110035302](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918110035302.png)

#### 监控变量

格式：

- `$monitor(p1, p2, ..., pn);`
- `$monitor;`
- `$monitoron;`
- `$monitoroff;`

任务 `$monitor` 提供了**监控和输出参数列表中的表达式或变量值的功能**。其参数列表中输出控制格式字符串和输出列表的规则和 `$display` 中的一样。当启动带有一个或多个参数的 `$monitor` 任务时，仿真器则建立一个处理机制，**使得每当参数列表中变量或表达式的值发生变化时，整个参数列表中变量或表达式的值都将输出显示。**如果同一时刻，两个或多个参数的值发生变化，则在该时刻只输出显示一次。

`$monitoron` 和 `$monitoroff` 任务的作用是通过打开和关闭监控标志来**控制监控任务 `$monitor` 的启动和停止**，这样使得程序员可以很容易地控制 `$monitor` 何时发生。其中 `$monitoroff` 任务用于关闭监控标志，停止监控任务 `$monitor` ， `$monitoron` 则用于打开监控标志，启动 `$monitor` 监控任务。 `$monitor` 与 `$display` 的不同处还在于 `$monitor` 往往在 `initial` 块中调用，只要不调用 `$monitoroff`， `$monitor` 便不间断地对所设定的信号进行监视。

#### 读取文件到存储器

格式：

- `$readmemh("<数据文件名>", <存储器名>);`
- `$readmemh("<数据文件名>", <存储器名>, <起始地址>);`
- `$readmemh("<数据文件名>", <存储器名>, <起始地址>, <结束地址>);`

功能： `$readmemh` **函数会根据绝对/相对路径找到需要访问的文件**，按照 ASCII 的解码方式将文件字节流解码并读入容器。文件中的内容必须是十六进制数字 0~f 或是不定值 x，高阻值 z（字母大小写均可），不需要前导 0x，不同的数用空格或换行隔开。假设存储器名为 arr，起始地址为 s，结束地址为 d，那么文件中用空格隔开的数字会依次读入到 arr[s],arr[s+1]... 到 arr[d]。**假如数字的位数大于数组元素的位数，那么只有低位会被读入，剩下的高位会被忽略。**

此系统任务用来从文件中读取数据到存储器中，类似于 C 语言中的 fread 函数。

例如：

```verilog
module im;
    reg[31:0] im_reg[0:2047];
    initial begin
            $readmemh("code.txt",im_reg);
    end
endmodule
```

仿真后即可将 code.txt 中的内容读入 `im_reg` 存储器中。

#### fsdb 相关参数（仅 VCS）

在 VCS 中，默认情况下，为了加快仿真速度，仿真器不会记录任何波形信号。我们需要使用 `$fsdbDumpvars` 系统任务，告知仿真器我们需要记录哪些信号。

该命令最简单的用法，只需一行，告知系统我们需要所有波形。我们计组的设计较小，导出所有波形也不会影响仿真速度，因此使用这个命令就足够。

```verilog
initial begin
    $fsdbDumpvars();
end
```

如果设计较复杂（例如有数百、数千个模块），记录所有波形会消耗较多性能，拖慢仿真速度。我们可以为该系统任务增加参数，只记录部分模块的波形，语法如下（其中括号内的竖线表示左右的写法选择一种，三个参数均为可选参数）：

```verilog
$fsdbDumpvars(
    [depth, | "level=",depth_var,]
    [instance, | "instance=", instance_var]
    [, "option"| ,"option, option_var"]
);
```

第一个参数指定要导出的深度：

- depth=0: 导出所有信号
- depth=1: 导出当前模块的信号
- depth=n: 导出当前模块和向下 n-1 级的信号

第二个参数指定要导出的模块名称；第三个参数是额外的选项。例如下面三种写法都是可以的：

```verilog
initial begin
    $fsdbDumpvars(0, system);
    $fsdbDumpvars(0, system, "+fsdbfile+novas.fsdb");
    $fsdbDumpvars(1, top.dut.u1);
end
```

除了 `$fsdbDumpvars` 指定要记录波形的范围外，我们也可以使用 `$fsdbDumpon` 和 `$fsdbDumpoff` 动态开关波形的记录。例如：

```verilog
initial begin
    $fsdbDumpvars();

    $fsdbDumpoff();
    #200;
    $fsdbDumpon();
    #300;
    $fsdbDumpoff();
    #100;
    $fsdbDumpon();
end
```

### 层次化事件队列

在前面的视频内容中，我们通过实例分析了阻塞赋值与非阻塞赋值的差异，细心的同学可能注意到了，在 Verilog 的语法中，以 **`begin` - `end` 为开头结尾的代码块被称作顺序块，也就是说从细节上理解-其中代码执行是按顺序进行的，但是实例中我们又称 `always` 块中的非阻塞赋值是“并发执行”的**，从概念上似乎出现了矛盾。其实，“并发执行”并不是运行规则，而是运行规则作用后的外在效果，真正的规则实际上是 Verilog 代码运行时的层次化事件队列。

同学们应该清楚，Verilog 是硬件描述语言，语句的执行顺序与 C 语言程序有很大差异。层次化事件队列是硬件仿真（Simulation）时，用于规定“不同事件执行的优先级关系”，在这里我们可以一般将一个事件理解为需要运行的一条语句（当然，有的语句由多个事件组成，例如非阻塞赋值需要被拆分为计算等号右边的值（RHS）和将结果赋予等号的左边变量（LHS），赋值事件在计算事件执行结束时才能加入队列。根据事件的优先级，Verilog 将其分为 4 个队列**（队列间的优先级不同，从上到下优先级依次递减，只有当优先级高的队列中所有任务完成后，才会继续完成优先级较低的任务）**

1. 动态事件队列（动态事件队列在队列内部执行顺序无硬性规定，但在同一个begin-end语句块中的语句应当严格按照源代码中的顺序执行；且多个非阻塞赋值应当按照语句执行顺序进行）
   - 阻塞赋值
   - 计算非阻塞赋值语句右边的表达式（RHS）
   - 连续赋值（如 `assign`）
   - 执行 `display` 命令
   - ……
2. 停止运行的时间队列（#0）（不推荐使用）
3. 非阻塞事件队列：更新非阻塞赋值语句 LHS（左边变量）的值。
4. 监控事件队列（执行 `monitor`，`strobe` 命令）

在大致了解 4 个队列后，同学们不妨尝试回答下列几个问题，加深理解：

1.为何“阻塞与非阻塞的区别”视频教程的代码中，阻塞赋值与非阻塞赋值最终的结果存在不同（请对比两种赋值方式在更新等号左边变量操作（LHS）的优先级）？

```verilog
module blocked_and_non_blocked(
    input clk,
    input a,
    output reg b_blocked,
    output reg c_blocked,
    output reg b_non_blocked,
    output reg c_non_blocked
    );

  // 阻塞赋值
  always @(posedge clk) begin
     b_blocked = a;
     c_blocked = b_blocked;
  end
  // 非阻塞赋值
  always @(posedge clk) begin
     b_non_blocked <= a;
     c_non_blocked <= b_non_blocked;
  end
endmodule
```

2.有时在 `always` 语句块中，使用 `display`、`strobe`、`monitor` 三种输出语句会得到不同结果，这是为什么？（例如下面的代码，同一时刻的 `display` 和 `monitor` 所输出的值为何相差 1？）

```verilog
module display_monitor_diff(
    input clk,
    output reg [3:0] out
    );

  initial begin
     out = 0;
     $monitor($time,"monitor out = %d",out);
  end

  always @(posedge clk) begin
     out <= out + 1;
     $display($time,"display out = %d",out);
  end
endmodule
```

```verilog
                   0 monitor out =  0
                  20 display out =  0
                  20 monitor out =  1
                  60 display out =  1
                  60 monitor out =  2
                  ......
```

3.在一个 `always` 语句块中同时使用阻塞赋值与非阻塞赋值存在什么风险（提示：请结合同一事件队列中的任务顺序无硬性规定这一特点）？由这个结论，在书写语句块时应该遵循哪些规范？

> 问题？还需要加深层次化队列的思考

### default nettype

在我们的编写过程中，如果不对某个变量显式声明而直接使用，该变量将会被隐式创建为**缺省类型**。

```verilog
module test(input temp);
    wire [3:0] a;
    assign a=4'b1000;
    assign b=a;
endmodule
```

打开仿真，会发现 b 的取值为 0，是一个位宽为 1 的变量。

Verilog 的默认缺省类型是 `wire`，如果我们不对某个变量显式声明类型或显式定义，而直接使用，该变量将会被默认地设为 `wire` 类型。上述示例中 temp 和 b 都是 `wire` 类型。

如果在使用该变量之后再对该变量显式声明，则变量的类型以之后显式声明的类型为准。

```verilog
module test(input wire[3:0] temp);
    wire [3:0] a;
    assign a=4'b1000;
    assign b=a;
    wire [3:0] b;
endmodule
```

在这个示例中，b 的值变成了 `4'b1000`。在语法检查或仿真时，可以看到编译器产生了如下警告：

> WARNING:HDLCompiler:35 - "test.v" Line N: \<b\> is already implicitly declared earlier.

这一语法特性容易造成一个问题：如果我们在对模块进行连接的时候，忘记对于某一个变量进行定义，或将变量名打错，如将 alu 打成 aiu，则该变量将会默认被定义为 1 位宽的 `wire` 类型，造成意料之外的 bug。

这时有一种解决方法：使用 ``default_nettype`。

```verilog
module test(input temp);
    wire [3:0] a;
    assign a=4'b1000;
    assign b=a;
endmodule
`default_nettype none
```

``default_nettype` 用于设置缺省类型，在代码文件的任意位置加入 ``default_nettype` 宏，都可以使得该代码文件中所有变量的缺省类型改变。若代码中有两个以上的 ``default_nettype` 宏，则将会以最后一条为准。

若需要取消缺省类型，即若不显式声明类型就会报错，则应该使用 ``default_nettype none`。以上的示例将会由于 temp 和 b 都未显式指定类型而报错。

### 函数：

函数用关键词 `function` 声明，并用 `endfunction` 结束，不允许输出端口声明（包括输出和双向端口），但可以有多个输入端口。函数只返回一个值到函数被调用的位置，并且在函数中返回值与函数名同名。

函数的定义如下所示：

```verilog
function [range] function_id; 
  input_declaration
  other_declarations 
  procedural_statement 
endfunction
```

在使用函数时有以下几点需要注意：

1. **函数定义**只能在模块中完成，不能出现在过程块(initial语句、always语句)中；
2. 函数至少要有一个输入端口；不能包含输出端口（output端口)和双向端口(inout端口)；
3. 在函数结构中， 不能使用任何形式的时间控制语句 （`#`、`wait` 等） ， 也不能使用 `disable` 中止语句；
4. 函数定义结构体中不能出现过程块语句（`always` 语句）；
5. 函数内部可以调用函数，但不能调用任务。

一个简单的例子：

```verilog
module comb15 (A, B, CIN, S, COUT);
  input [3:0] A, B;
  input CIN;
  output [3:0] S;
  output COUT;

  wire [1:0] S0, S1, S2, S3;

  function signed [1:0] ADD;

    input A, B, CIN;

    reg S, COUT;

    begin
      S = A ^ B ^ CIN;
      COUT = (A&B) | (A&CIN) | (B&CIN);
      ADD = {COUT, S};
    end
  endfunction

  assign S0 = ADD (A[0], B[0], CIN),
  S1 = ADD (A[1], B[1], S0[1]),
  S2 = ADD (A[2], B[2], S1[1]),
  S3 = ADD (A[3], B[3], S2[1]),
  S = {S3[0], S2[0], S1[0], S0[0]},
  COUT = S3[1];
endmodule
```

在函数调用中，有下列几点需要注意：

1. 函数调用可以在过程块中完成，也可以在 `assign` 这样的连续赋值语句中出现。
2. 函数调用语句不能单独作为一条语句出现，**只能作为赋值语句的右端操作数。**
3. 函数结构中**不能使用任何形式的时间控制语句**(#, wait等)，也不能使用disable中止语句

由于上面的约束，函数能做的事情也就没多少了，只能够算算数值什么的。

```verilog
 
modulefunction_total(
input clk,
input rst,
input [7:0] width,
output  reg[16:0] area
);
//   
function[15:0] circle(input [7:0]diameter);
  begin
   circle= (24'd201 * {16'h0, diameter}*{16'h0, diameter})/256;
  end
endfunction
     
function[15:0] square(input [7:0] width);
begin
square= {8'h0, width}*{8'h0, width};
  end
endfunction
// 
function[16:0] total(input [7:0] width);
begin
total= {1'b0, square(width)} + {1'b0, circle(width)};
  end
endfunction
///     
always @ (posedge clk or negedge rst)
if(!rst)
area<= 17'd0;
else
   area<= total(width);
endmodule
```

总的来说，verilog函数的内容只能是组合逻辑，不能是时序逻辑。但函数可以在assign和always中作为赋值操作的右值被调用。

### Icarus Verilog 与自动化测试

我们可以利用 Icarus Verilog 这款轻量级的仿真软件运行对我们的 Verilog 文件进行仿真。（**注意⚠️：最终仿真的结果依旧以 ISE 为准，Icarus Verilog 在某些情况下会与 ISE 产生不同结果**）

除了体量小速度快的特点，iverilog 还可以在命令行中进行功能仿真，我们在 Windows 中利用 bat 批处理文件，或者 Linux 中使用 shell 进行批处理文件的编写，利用几条命令，就可以完成 Verilog 的仿真和其标准输出的转储操作。

以下是可能用到的命令：

```bash
iverilog -o [target_file] source_file...
```

将源文件编译为 vvp 格式的目标文件，利用 vvp 文件，可以启动 Verilog 文件的功能仿真。

```bash
vvp target_file
```

运行 vvp 格式的目标文件，运行后 vvp 命令将输出功能仿真时产生的各个输出信息。此时可以使用管道或重定向保存输出信息，以便于我们进行下一步的对拍。

Icarus Verilog 在大部分情况下都与 ISE 的结果相同，但是在使用中发现，在 `always @*` 块中使用阻塞语句，将会导致二者的结果出现不同，因此，最好按照标准使用非阻塞语句。其他情况下基本二者行为一致，但测评依旧以 ISE 的输出为准。课程组提供的 VCS 的行为与 ISE 更近似，请同学们考虑选用。

补充：

我们在 ISE 中查看波形图，不是感到运行太慢、电脑吃不消，就是时不时的 ISE 会产生一些未知的 BUG，在这里，我们推荐 gtkwave 和 Scansio 两款波形查看软件，他们同样体量较小，运行速度快，并且在多个平台都可运行，如果你是 MacOS 系统，不妨试一试他们。

### MARS与自动化测试

##### 利用 MARS 命令行进行 CPU 自动化模拟

如下为常用命令行参数：

![image-20220918152636663](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918152636663.png)

MARS 常用命令如下：

1. `java -jar mars.jar mc CompactDataAtZero a dump .text HexText hexcode.txt fibonacci.asm`，将 MIPS 文件 fibonacci.asm 编译但不运行，选取其中的 `.text` 段生成 HexText（即十六进制 ASCII 文件）写入 hexcode.txt 中。这在我们后续将代码和数据填入 Verilog 中有很大的作用。

   mc 为内存配置设置，CompactDataAtZero 即为我们需要的内存配置。

   a 为编译但不模拟。

   dump 为导出内存信息，.text 选择了导出的内存字段，HexText为十六进制的ASCII格式。

2. `java -jar mars.jar db mc CompactDataAtZero nc fibonacci.asm`，将 MIPS 文件 fibonacci.asm 编译并运行。

   db 为开启 delay branch。

   nc 为禁止显示版权信息，此处为了获得干净的输出信息。



## 补充：

### verilog中层次化的事件队列

在IEEE1364—1995 Verilog标准的5.3节定义了层次化的事件队列在逻辑上分为用于当前仿真时间的4个不同的队列，用于下一段仿真时间的若干附加队列。

（1）动态事件队列（下列事件执行的顺序随机安排）

① 阻塞赋值；

② 计算阻塞赋值右边的表达式；

③ 连续赋值（对wire型变量用assign赋值）；

④ 执行$display命令；

⑤ 计算原语的输入与输出变化。

（2）停止运行的时间队列： #0 延时阻塞赋值。

（3）非阻塞事件队列：更新非阻塞赋值语句LHS（<=左边的变量）的值。

（4）监控事件队列：

① 执行$monitor命令；

② 执行$strobe命令。

（5）其他指定的PLI命令队列：其他PLI命令。

![image-20220918154735190](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918154735190.png)

动态事件就是活跃事件，它们的优先级最高，在当前的仿真时间中，活跃事件最先被执行。

上图可以看出，阻塞赋值是活跃事件，当前仿真时间中它最先被执行，**阻塞赋值执行完RHS（右式计算）后，立刻赋值给LHS。并且在执行阻塞赋值期间，不允许其他语句干扰。**对于非阻塞赋值，RHS是活跃事件，最先执行，但执行完后并不急于赋值，而是在事件队列列表中等待，等到后期仿真快要结束时再执行。

阻塞赋值特点：

（1）阻塞赋值可以认为只有一个步骤操作，即计算RHS并更新LHS，此时不允许有来自任何其他的Verilog语句的干扰。

（2）在同一个always块中，只有前面一条阻塞赋值语句执行完之后，才开始执行下一条。（这就是所谓阻塞的概念）。

非阻塞赋值特点：

非阻塞赋值可以看成是两个步骤：①在赋值开始时刻，计算非阻塞赋值RHS表达式；②在赋值结束时刻，更新非阻塞赋值LHS表达式。

 使用阻塞赋值不能进行自触发的振荡器

```verilog
module learn(
output clk);
reg clk;
initial #10 clk = 0;
always @(clk) #10 clk = ~clk;
endmodule
```

![image-20220918154852775](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918154852775.png)

在initial块中，经过10个单位时间的的延时，clk被立即阻塞赋值为0。当clk=0发生时，触发@（clk）,经过10个单位的延迟，计算RHS（~clk）得到1，并立即更新LHS的值，clk被赋值为1。由于此过程不许被打扰，所以当always回到判断触发条件@（clk）时，此时clk电平已经为1，但是对于always而言，并没有感知到clk从0到1的变化，因此对于always而言，认为该clk并没有发生变化，所以就阻塞在那里，不触犯@（clk）条件。所以，clk翻转一次后不再变化。

使用非阻塞赋值的自触发振器

```verilog
module learn(
output clk);
reg clk;
initial #10 clk = 0;
always @(clk) #10 clk <= ~clk;
endmodule
```

![image-20220918154934742](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918154934742.png)

 @（clk）第一次触发后，非阻塞赋值的RHS表达式便计算出来了，并把值付给LHS的事件安排在了更新事件的队列中。在非阻塞赋值更新事件队列被激活前，又遇到@（clk）触发语句，并且always块再次对clk的值变化产生反应。当非阻塞LHS的值在同一时刻被更新时，@（clk）再次触发。



[仿真器](https://so.csdn.net/so/search?q=仿真器&spm=1001.2101.3001.7020)首先按照仿真时间对事件进行排序，然后再在当前仿真时间里按照事件的优先级顺序进行排序。

活跃事件是[优先级](https://so.csdn.net/so/search?q=优先级&spm=1001.2101.3001.7020)最高的时间，**在活跃事件之间，它们的执行顺序是随机的。**看起来像并行运行。

两个缩写：RHS(right-hand-side) 和LHS(left-hand-side)。

前者指等式右边的[表达式](https://so.csdn.net/so/search?q=表达式&spm=1001.2101.3001.7020)或者变量(RHS expression or RHS variable)，后者指等式左边的表达式或者变量(LHS expression or LHS variable)。

由上表可知，阻塞赋值属于活跃事件，会立刻执行，这就是阻塞赋值“计算完毕，立即更新”的原因。**此外，由于在分层事件队列中，只有将活跃事件中排在前面的事件调出，并执行完毕后，才能够执行下面的事件，这就可以解释阻塞赋值的第二个特点。**

同样由上表可知，非阻塞赋值的RHS计算属于活跃事件，而非阻塞赋值的更新事件排在非活跃事件之后，**因此只有仿真队列中所有的活跃事件和非活跃事件都执行完毕后，才轮到非阻塞赋值更新事件，这就是非阻塞赋值必须分两拍完成的原因。**

**阻塞赋值特点：**

1、RHS的表达式计算和LHS的赋值更新，这两个动作之间不能插入其他动作，即所谓计算完毕，立即更新。

2、所谓阻塞赋值就是在一个”begin…end”块中的多个阻塞赋值语句，只有上一句完全执行完毕后，才会执行下一语句。

**非阻塞赋值()程序的执行**：

非阻塞赋值，RHS的计算表达式和LHS的赋值更新分两个节拍执行。首先，应该是RHS表达式计算，得到新值后并不立即赋值，而是放在事件队列中等待，直到当前仿真时刻的后期才执行。

![image-20220918161549770](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918161549770.png)

### Verilog中reg和wire的使用

#### wire （组合逻辑）

wire用来连接模块实例化的输入和输出端口；

wire用作实际模块声明中输入和输出；

wire 元素必须由某些东西驱动，并且在没有被驱动的情况下，无法存储值；

wire 元素不能用在 always模块中 = 或者 <= 的左边；

wire元素是assign语句左侧 唯一的合法类型；

wire 元素是在基于Verilog的设计中连接两片的无状态方式;

wire 只能用在 组合逻辑。

```verilog
// wire 的一些正确使用
        wire A,B,C,D,E ; //1-bit wide
        wire [8:0] Wide; //9-bit
        assign A= B & C;
        always @(B or C) 
        begin
         I=B | C;
        end
        mymodule mymodule_instance(.In(D),Out(E));
```

#### **reg**(组合和时序逻辑)

reg 和 wire类似，但它可以用来存储信息（状态），就像寄存器

reg可以连接到模块实例化的输入端口；

reg 不能连接到模块实例化的输出端口；

reg 可以用作实际模块声明中的输出；

reg 不能 用作实际模块声明中的输入；

reg是always 模块中 = 或者 <= 左侧的唯一正确类型；

reg是initial模块中 = 左侧唯一的合法类型；

reg**不能用在assign的左边**；

reg 当与always @（posedge Clock）块结合使用时，reg可用于创建寄存器。

```verilog
//reg一些合法使用
wire A,B;
reg I,J,K;  // 1-bit
reg [8:0] Wide; // 9-bit
always @(A or B) begin
    I=A | B; // using a reg as the left - hand side of an always

initial begin // using a reg in an initial block
    J=1'b1;
    #1
    J=1'b0;
end

always @(posedge Clock) begin
// using a reg to create a positive -edge - triggered register
    K<=I;
end
```

#### 什么时候wire 和reg 可以互换？

1. 两者都可以出现在assign语句和 always 模块中=或<=的右侧;
2. 两者都可以连接到模块实例的输入端口;

#### 其他

 不指定就默认为1位wire类型。专门指定出wire类型，可能是多位或为使程序易读。wire只能被assign连续赋值，reg只能在initial和always中赋值。wire使用在连续赋值语句中，而reg使用在过程赋值语句中。

​    在**连续赋值语句**中，表达式右侧的计算结果可以立即更新表达式的左侧。在理解上，相当于一个逻辑之后**直接连了一条线**，这个逻辑对应于表达式的右侧，而这条线就对应于wire。在过程赋值语句中，表达式右侧的计算结果**在某种条件的触发下放到一个变量当中**，而这个变量可以声明成reg类型的。根据触发条件的不同，过程赋值语句可以建模不同的硬件结构：如果这个条件是**时钟的上升沿或下降沿**，那么这个硬件模型就是一个**触发器**；如果这个条件是**某一信号的高电平或低电平**，那么这个硬件模型就是一个**锁存器**；如果这个条件是**赋值语句右侧任意操作数的变化**，那么这个硬件模型就是一个**组合逻辑。**

**输入端口可以由wire/reg驱动，但输入端口只能是wire；输出端口可以使wire/reg类型，输出端口只能驱动wire；若输出端口在过程块中赋值则为reg型，若在过程块外赋值则为net型。用关键词inout声明一个双向端口, inout端口不能声明为reg类型，只能是wire类型；输入和双向端口不能声明为寄存器类型。**

从仿真的角度来说，HDL语言面对的是编译器（如Modelsim等），相当于软件思路。
		这时：
		wire对应于连续赋值，如assign
		reg对应于过程赋值，如always，initial

从综合的角度来说，HDL语言面对的是综合器（如DC等），要从电路的角度来考虑。
这时：
	1、wire型的变量综合出来一般是一根导线；
	2、reg变量在always块中有两种情况：
		(1)、always后的敏感表中是（a or b or c）形式的，也就是不带时钟边沿的，综合出来还是组合逻辑
		(2)、always后的敏感表中是（posedge clk）形式的，也就是带边沿的，综合出来一般是时序逻辑，会包含触发器（Flip－Flop） 

在设计中，输入信号一般来说你是不知道上一级是寄存器输出还是组合逻辑输出，那么对于本级来说就是一根导线，也就是wire型。而输出信号则由你自己来决定是寄存器输出还是组合逻辑输出，wire型、reg型都可以。但一般的，整个设计的外部输出（即最顶层模块的输出），要求是寄存器输出，较稳定、扇出能力也较好。

### Verilog中的任务Task和函数Function

利用任务和函数可以把一个很大的程序模块分解成许多较小的任务和函数便于理解和调试。

学会使用task和function语句可以简化程序的结构，使程序明白易懂，是编写较大型模块的基本功。

#### **task和function说明语句的不同点**

任务和函数有些不同，主要的不同有以下四点：

- 函数只能与主模块共用同一个仿真时间单位，而任务可以定义自己的仿真时间单位。
- 函数不能启动任务，而任务能启动其它任务和函数。
- 函数至少要有一个输入变量，而任务可以没有或有多个任何类型的变量。
- 函数返回一个值，而任务则不返回值。

函数的目的是通过返回一个值来响应输入信号的值。任务却能支持多种目的，能计算多个结果值，这些结果值只能通过被调用的任务的输出或总线端口送出。**Verilog HDL模块使用函数时是把它当作表达式中的操作符**，这个操作的结果值就是这个函数的返回值。下面让我们用例子来说明：

例如，定义一任务或函数对一个16位的字进行操作让高字节与低字节互换，把它变为另一个字(假定这个任务或函数名为: switch_bytes)。

任务返回的新字是通过输出端口的变量，因此16位字字节互换任务的调用源码是这样的：

```verilog
 switch_bytes(old_word,new_word);
```

任务switch_bytes把输入old_word的字的高、低字节互换放入new_word端口输出。

而函数返回的新字是通过函数本身的返回值，因此16位字字节互换函数的调用源码是这样的：

```verilog
new_word = switch_bytes(old_word);
```

#### **一 task语句**

如果传给任务的变量值和任务完成后接收结果的变量已定义，就可以用一条语句启动任务。任务完成以后控制就传回启动过程。如任务内部有定时控制，则启动的时间可以与控制返回的时间不同。任务可以启动其它的任务，其它任务又可以启动别的任务，可以启动的任务数是没有限制的。不管有多少任务启动，只有当所有的启动任务完成以后，控制才能返回。

***\**任务的定义\*\****

定义任务的语法如下：

任务:

```verilog

task <任务名>;
     <端口及数据类型声明语句>
     <语句1>
     <语句2>
     .....
     <语句n>
endtask
```

这些声明语句的语法与模块定义中的对应声明语句的语法是一致的。

***任务的调用及变量的传递\***

启动任务并传递输入输出变量的声明语句的语法如下：

任务的调用：

```verilog
<任务名>(端口1，端口2，...，端口n);
```

下面的例子说明怎样定义任务和调用任务：

任务定义：

```verilog

task my_task;
    input a, b;
    inout c;
    output d, e;
    …
    <语句> //执行任务工作相应的语句
    …
    c = foo1; //赋初始值
    d = foo2; //对任务的输出变量赋值t
    e = foo3;
endtask
```

任务调用：

```verilog
my_task(v,w,x,y,z);
```

**任务调用变量(v,w,x,y,z)和任务定义的I/O变量(a,b,c,d,e)之间是一一对应的。当任务启动时，由v,w,和x.传入的变量赋给了a,b,和c，而当任务完成后的输出又通过c,d和e赋给了x,y和z。**

任务定义时需注意以下事项：

（1）在第一行“task”语句中不能列出端口名列表。

（2）任务中可以有延时语句、敏感事件控制语句等事件控制语句。

（3）任务可以没有或可以有一个或多个输入、输出和双向端口。

（4）任务可以没有返回值，也可以通过输出端口或双向端口返回一个或多个返回值。

（5）任务可以调用其它的任务或函数，也可以调用该任务本身。

（6）任务定义结构内不允许出现过程块（initial或always过程块）。

（7）任务定义结构内可以出现disable终止语句，这条语句的执行将中断正在执行的任务。在任务被中断后，程序流程将返回到调用任务的地方继续向下执行。

#### **二 function语句**

“<函数名>”是给被定义函数取的名称。这个函数名在函数定义结构内部还代表着一个内部变量，函数调用后的返回值是通过这个函数名变量传递给调用语句的。

**函数的目的是返回一个用于表达式的值。**

***从函数返回的值\***

函数的定义蕴含声明了与函数同名的、函数内部的寄存器。如在函数的声明语句中<返回值的类型或范围>为缺省,则这个寄存器是一位的，否则是与函数定义中<返回值的类型或范围>一致的寄存器。

函数的定义把函数返回值所赋值寄存器的名称初始化为与函数同名的内部变量。

函数的使用规则

与任务相比较函数的使用有较多的约束，下面给出的是函数的使用规则：

1) 函数的定义不能包含有任何的时间控制语句，即任何用＃、@、或wait来标识的语句。
2) 函数不能启动任务。
3) 定义函数时至少要有一个输入参量。
4) 在函数的定义中必须有一条赋值语句给函数中的一个内部变量赋以函数的结果值，该内部变量具有和函数名相同的名字。

A.在进行函数定义时需要注意以下几点：

（1）与任务一样，函数定义结构只能出现在模块中，而不能出现在过程块内。

（2）函数至少必须有一个输入端口。

（3）函数不能有任何类型的输出端口（output端口）和双向端口（inout端口）。

（4）在函数定义结构中的行为语句部分内不能出现任何类型的时间控制描述，也不允许使用disable终止语句。

（5）与任务定义一样，函数定义结构内部不能出现过程块。

（6）在一个函数内可以对其它函数进行调用，但是函数不能调用其它任务。

（7）在第一行“function”语句中不能出现端口名列表。

（8）在函数声明的时候，在Verilog HDL的内部隐含地声明了一个名为function_identifier（函数标识符）的寄存器类型变量，函数的输出结果将通过这个寄存器类型变量被传递回来。

B.函数调用时要注意以下几点：

（1）函数的调用不能单独作为一条语句出现，它只能作为一个操作数出现在调用语句内。

（2）函数的调用既能出现在过程块中，也能出现在assign连续赋值语句中。

（3）函数定义中声明的所有局部寄存器都是静态的，即函数中的局部寄存器在函数的多个调用之间保持它们的值。

最后总结一下任务与函数的区别：

![image-20220918165148172](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918165148172.png)

示例：模块（内部定义了函数）

```verilog
`timescale 1ns / 1ps
module reg_test(
    input clk,
    input [3:0] address,
    input [7:0] data_in,
    output [7:0]    data_out,
    input   write_en,
    input   read_en,
    input   resetb
);
    
reg [7:0]   reg0,reg1,reg2,reg3;
reg [7:0]   read_data;
//定义函数    
function    [7:0] dev_reg_nxt;
    input [3:0] address;
    input [3:0] reg_offset;
    input   write_en;
    input [7:0] data_in;
    input [7:0] dev_reg;
        begin
            dev_reg_nxt=(address==reg_offset && write_en)? data_in:dev_reg;
        end
endfunction
    
assign data_out=read_data;
always@(posedge clk or negedge resetb)begin
    if(!resetb)
        begin
            reg0<=8'b0000_0000;
            reg1<=8'b0000_0000;
            reg2<=8'b0000_0000;
            reg3<=8'b0000_0000;
            read_data<=8'b0000_0000;
        end
    else
        begin
            reg0<=dev_reg_nxt(address,4'b0000,write_en,data_in,reg0);//调用函数
            reg1<=dev_reg_nxt(address,4'b0001,write_en,data_in,reg1);
            reg2<=dev_reg_nxt(address,4'b0010,write_en,data_in,reg2);
            reg3<=dev_reg_nxt(address,4'b0011,write_en,data_in,reg3);
        end
end
        
always@(*)begin
    if(read_en)begin
        case(address)
            4'b0000:read_data=reg0;
            4'b0001:read_data=reg1;
            4'b0010:read_data=reg2;
            4'b0011:read_data=reg3;
        endcase
    end
end
endmodule
```

 上面模块的testbench（内部定义了任务）

```verilog
`timescale 1ns / 1ps
module tstbench();
 
reg clk,write_en,read_en,resetb;
reg [3:0] address;
reg [7:0] data_in;
wire [7:0] data_out;  
     
reg_test test_function(
        .clk(clk),
        .address(address),
        .data_in(data_in),
        .data_out(data_out),
        .write_en(write_en),
        .read_en(read_en),
        .resetb(resetb)
        );
 
initial begin
     clk=1'b0;
     forever begin
        #5 clk=~clk;
     end 
end    
 
initial begin
    resetb=1'b1;
    #10    resetb=1'b0;
    #10    resetb=1'b1;
end
 
initial begin
    address=4'b0000;
    data_in=8'b0000_0000;
    write_en=1'b0;
    read_en=1'b0;
end
//定义任务，此任务执行完成占用三个时钟周期      
task reg_write;
    input [3:0] address_task;
    input [7:0] data_in_task;
    begin
        @(posedge clk);
            #1 address=address_task;
        @(posedge clk);
            #1 write_en=1'b1;
            data_in=data_in_task;
        @(posedge clk);
            #1;
            write_en=1'b0;
            address=4'hF;
            data_in=4'h0;     
    end
endtask
//定义任务，此个任务执行完成占用四个时钟周期 
task reg_read;
    input [3:0] address_task;
    input [7:0] expected_data;
    begin
        @(posedge clk);
                #1 address=address_task;
        @(posedge clk);
                #1 read_en=1'b1;
        @(posedge clk);
                #1 read_en=1'b0;
                address=4'hF;
        @(posedge clk);
                if(expected_data===data_out)
                $display("data match:expected_data=%h   ,actual data=%h   ",expected_data,data_out);
                else
                $display("ERROR !!! data nomatch:expected_data=%h   ,actual data=%h   ",expected_data,data_out);
    end
endtask
 
initial begin
#100;
        reg_write(4'b0000,8'hA5);//执行写任务
        reg_read(4'b0000,8'hA5);//执行读任务并与预期数据做比对       
        reg_write(4'b0001,8'hA2);
        reg_read(4'b0001,8'hA2);
        reg_write(4'b0010,8'hA3);
        reg_read(4'b0010,8'hA3);
        reg_write(4'b0011,8'hA1);
        reg_read(4'b0011,8'hA1);
//        $finish;
end
endmodule
 
```

以上的仿真波形： 

<img src="C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918165313570.png" alt="image-20220918165313570" style="zoom:150%;" />

### 
