Verilog查缺补漏

### 字符串表示方法

字符串是由双引号包起来的字符队列。字符串不能多行书写，即字符串中不能包含回车符。Verilog 将字符串当做一系列的单字节 ASCII 字符队列。例如，为存储字符串 "www.runoob.com", 需要 14*8bit 的存储单元。例如：

```verilog
reg [0: 14*8-1]       str ;
initial begin
    str = "www.runoob.com";
end  
```

reg型变量维无符号数，而integer型变量为有符号数。

### 算术操作符

如果操作数某一位为 X，则计算结果也会全部出现 X。例如：

```verilog
b = 4'b100x ;
c = a+b ;       //结果为c=4'bxxxx
```

