# 编程——将高级语言结构转换成MIPS汇编代码。

## 一.算数/逻辑指令

### 1.逻辑指令

MIPS逻辑位操作包括 add、or、xor、和 nor。

andi、ori、xori

- and指令可以用于屏蔽位（将不用的位置0）
- or指令可用于组合来自两个寄存器的位
- not指令可以由A NOR $0 = NOT A给出

### 2.移位指令

MIPS移位操作包括逻辑左移指令(sll)、逻辑右移指令(srl)、和算数右移指令(sra)。

可变移位指令：可变逻辑左移指令(sllv)、可变逻辑右移指令(srlv)、可变算数右移指令(srav)。

### 3.生成常数

addi指令可以用于16位常数赋值。

为了赋值32位常数，可以先使用一条装入高位立即数指令(lui),接着使用一条or立即数指令(ori)。

### 4.乘法和除法指令

乘法和除法指令与其他算术指令有些不同。两个32位数相乘，产生一个64位乘积。两个32位数相除，产生一个32位的商和一个32位的余数。

MIPS体系结构由两个用于存放乘法和除法结果的特殊用途寄存器hi和lo。将乘法结果的高32位存放在hi中，低32位存放在lo中。类似的，将除法结果的商存放在lo中，余数存放在hi中。

MIPS提供另一种乘法指令，他生成存储在一通用寄存器中的32位结果。

```
mul $s1, $s2, $s3
```

把s2和s3中的值相乘并把32位结果存储在s1中。

## 二.分支

为了顺序执行指令，程序计数器执行一条指令后递增4。条件分支指令执行一次测试，只有当测试结果位true是才执行分支语句，无条件分支指令称为跳转指令，他总执行分支语句。

### 1.条件分支

MIPS的条件分支指令：beq和bne。当两个寄存器中的值相等时，beq执行分句，当两个值不相等时，bne执行分支程序。

## 2.跳转指令

程序可以使用三种跳转指令完成无条件分支或跳转(jump)。这三种指令分别是跳转指令(j)、跳转和链接指令(jal)、及跳转寄存器指令(jr)。跳转指令(j)直接跳转到标号指定的指令。跳转和链接指令(jal)与j类似，但他保存返回地址。跳转寄存器指令(jr)跳转到寄存器所保存的地址。

## 三.条件语句

### 1.if语句

高级语言

```c
if(i==j)
	f = g + h;
f = f - i;
```

MIPS汇编语言

```
#  $s0 = f, $s1 = g, $s2 = h, $s3 = i, $s4 = j

bne $s3, $s4, L1	#if i!=j ,skip if block
	add $s0, $s1, $s2	#if block: f=g+h
L1:
	sub $s0, $s0, s3 	#f = f - i
```

### 2.if/else语句

高级语言代码

```c
if(i==j)
	f = g + h;
else
	f = f - i;
```

MIPS汇编代码

```
#  $s0 = f, $s1 = g, $s2 = h, $s3 = i, $s4 = j
	bne $s3, $s4, else		# if i!=j branch to else
	add $s0, $s1, $s2		# if block: f = g + h
	j L2				   # skip past the else block
else:
	sub $s0, $s0, $s3		# else block: f = f - i
L2:
```

### 3.switch/case语句

相当于多条嵌套的 if/else 语句

高级语言代码

```c
switch (amount){
	case 20:	fee = 2;break;
	case 50:	fee = 3;break;
	case 100:	fee = 5;break;
	default:	fee = 0;
}
//equicalent function using if/else statements
if (amount == 20) fee = 2;
else if (amount == 50) fee = 3;
else if (amount == 100) fee = 5;
else fee = 0;
```

MIPS汇编代码：

```
# $s0 = amount, $s1 = fee
case 20: 
	addi $t0, $0, 20	    # $t0 = 20
	bne $s0, $t0, case50	# amount == 20 ? if not
						  # skip to case50
	addi $s1, $0, 2			# if so ,fee = 2
	j done				   # and break out of case
case 50:
	addi $t0, $0, 50
	bne $s0, $t0, case100
	
	addi $s1, $0, 3
	j done
case 100:
	addi $t0, $0, 100
	bne $s0,$t0, default
	
	addi $s1, $0, 5
	j done
default: 
	add $s1, $0, 0
done:
```

## 四.循环语句

### 1.while循环

与if/else语句类似。while循环的汇编代码的测试条件与高级语言代码相反。如果那个相反的条件位true、那么while循环就停止。

高级语言代码

```c
int pow = 1;
int x = 0;

while(pow != 128){
pow = pow * 2;
x = x + 1;
}
```

MIPS汇编代码

```
# $s0 = pow, $s1 = x
addi $s0, $0, 1			# pow = 1
addi $s1, $0, 0			# x = 0

addi $t0, $0, 128		# t0 = 128 for comparison
while:				   
	beq $s0, $t0, done	# if pow == 128, exit while loop
	sll $s0, $s0, 1		# pow = pow * 2
	addi $s1, $s1, 1	# x = x + 1
	j while
done:
```

### 2.for循环

高级语言代码;

```c
int sum = 0;
for(i=0;i!=10;i++){
sum = sum + i;
}

//equivalent to the following while loop
int sum = 0;
int i=0;
while(i!=10){
sum = sum + i;
i = i + 1;
}
```

MIPS汇编代码

```
# $s0 - i, $s1 = sum
	add $s1, $0, $0			# sum = 0
	addi $s0, $0, 0			# i = 0
	addi $t0, $0, 10		# $t0 = 10
for:
	beq $s0, $t0, done		# if i==10, branch to done
	add $s1, $s1, $s0		# sum = sum + i
	addi $s0, $s0, 1		# increment i
	j for
done:
```

### 3.量值比较

目前为止，例子使用beq和bne指令执行相等或不相等的比较和分支。MIPS为量值比较提供了小于设置指令(slt)。当 rs < rt 时，slt 将 rd 置为1，否则，rd 为 0

使用slt的循环

高级语言代码

```c
int sum = 0;
for(i=1;i<101;i=i*2)
	sum = sum + i;
```

MIPS汇编代码

```
# $s0 = i, $s1 = sum

addi $s1, $0, 0			# sum = 0
addi $s0, $0, 1			# i = 1
addi $t0, $0, 101		# $t0 = 101

loop:
slt $t1, $s0, $t0		#if(i<101) $t1 = 1, else $t1 = 0 
beq $t1, $0, done		#if( $t1 == 0) branch to done
add $s1, $s1, $s0		#sum = sum + i
sll $s0, $s0, 1			#i = i * 2
j loop

done:
```

## 五.数组

### 1.数组下标

![image-20221008182631092](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221008182631092.png)

这个存储器中有一个包含5个整数的数组，下标的范围从0到4。这种情况下，数组存储在处理器主存中从基地址0x10007000开始的连续区域中。基地址制定了第一个元素array[0]的地址。

下面的代码将数组array中的前两个元素乘以8，然后将他们存储到该数组中。

高级语言代码

```c
int array[5];
array[0] = array[0] * 8;
array[1] = array[1] * 8;
```

MIPS汇编代码

```
# s0 = base address of array
lui $s0, 0x1000			# $s0 = 0x10000000
oir $s0, $s0, 0x7000	# $s0 = 0x10007000 

lw $t1, 0($s0)			# $t1 = array[0]
sll $t1, $t1, 3			# $t1 = $t1 << 3
sw $t1, 0($s0)			# array[0] = $t1

lw $t1, 4($s0)			# $t1 = array[1]
sll $t1, $t1, 3			# $t1 = $t1 << 3
sw $t1, 4($s0)			# array[1] = $t1
```

下面的代码示例用一个for循环将及地址为0x23B8F000的数组中的所有1000个元素乘以8

高级语言代码

```c
int i;
int array[1000];
for(i=0;i<1000;i++)
array[i] = array[i] * 8;
```

MIPS汇编代码

```
# $s0 = array base address, $s1 = i
#initialization code
lui $s0, 0x23B8			# $s0 = 0x23B80000
ori $s0, $s0, 0xF000	# $s0 = 0x23B8F000
addi $s1, $0, 0			# i = 0
addi $t2, $0, 1000		# $t2 = 1000

loop:
slt $t0, $s1, $t2		# i < 1000?
beq $t0, $0, done		# if not, then done
sll $t0, $s1, 4			# $t0 = i * 4 (byte offset)
add $t0, $t0, $s0		# address of array[i]
lw $t1, 0($t0)			# $t1 = array[i]
sll $t1, $t1, 3			# $t1 = array[i] * 8
sw $t1, 0($t0)			# array[i] = array[i] * 8
addi $s1, $s1, 1		# i = i + 1
j loop					#repeat

done:
```

数组下标为变量 i 而不是常数，所以不能使用lw中的立即数偏移量，相反可以计算第 i 个元素的地址，并将他储存在 $t0 中。需要注意的是，每个数组元素为一个字(32为int), 但存储器是以字节寻址的。

### 2.字节和字符

范围在[-128,127]中的数可以存储在一个单独字节中，而不需要一个完整的字。c语言使用char类型来表示字节或字符。

MIPS提供了装入字节和存储字节指令来操作字节或字符类型数据：装入无符号字节(lbu),装入字节(lb)和存储字节(sb)。下图描述了这三种指令。

![image-20221008185001088](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221008185001088.png)

存储字节指令(sb)将32位寄存器中的最低有效字节存储到存储器中的指定字节地址。sb指令将 s3的最低有效字节存储到存储器地址3中，它用0x9B取代0xF7。s3最高有效字节部分被忽略

## 六.函数调用

函数的输入和输出分别称为参数(arguments)和返回值(return value)。函数将计算返回值并且不会产生其他非预期的不良影响。

当一个函数调用其他函数时，调用函数(caller)和被调用函数(callee)必须要在参数和返回值上保持一致。MIPS系统的惯例是:调用函数在调用前要将4个参数分别放在 a0 - a3 中，被调用函数在完成前将返回值放在 v0 - v1中。

被调用函数不能影响调用函数的功能。简单地说，这意味着被调用函数必须知道当他完成后要返回到哪里，而且它不能破坏调用函数用到的寄存器和内存。调用函数将返回地址(return address) 存储在 ra 寄存器中，与此同时，它使用jal指令跳转到被调用函数入口。被调用函数不能覆盖调用函数所需要的任何体系结构状态和内存。具体的说，被调用函数必须保证存储寄存器 s0 - s7 和 ra 以及用于存放临时变量的栈(stack)不被修改。

### 1.函数的调用和返回

jal 指令和 jr 指令是函数调用中两个很重要的指令。jal 指令完成两种功能：1）将下一条指令( jal 后面的指令)的地址存储到返回地址寄存器 ra 中；2）跳转到目标指令。

高级语言代码

```c
int main(){
simple();
...
}

//void means the function returns no value
void simple(){
	return;
}
```

MIPS汇编代码

```
0x00400200	main:     jal simple   # call function
0x00400204			 ...

0x00401020  simple:   jr $ra	   # return
```

### 2.输入参数和返回值

根据MIPS惯例，程序使用 a0 - a3 保存输入参数， 使用 v0 - v1 保存返回值。

高级语言代码

```c
int main(){
	int y;
	...
	y = diffofsums(2,3,4,5);
	...
}

int diffofsums(int f, int g, int h, int i){
    int result;

	result = (f + g) - (h + i);
	return result;
}
```

MIPS汇编代码：

```
# $s0 = y
main:
	...
	addi $a0, $0, 2			# argument 0 = 2
	addi $a1, $0, 3			# argument 1 = 3
	addi $a2, $0, 4			# argument 2 = 4
	addi $a3, $0, 5			# argument 3 = 5
	jal diffofsums			# call function
	add $s0, $v0, $0		# y = returned value
	...

# $s0 = result
diffofsums:	
	add $t0, $a0, $a1		# $t0 = f + g
	add $t1, $a2, $a3		# $t1 = h + i
	sub $s0, $t0, $t1	    # result = (f + g) - (h + i)
	add $v0, $s0, $0	    # put result calue in $v0
	jr $ra				   #return to caller
```

根据MIPS惯例， 调用程序(main)将程序参数从左到右放入输入寄存器 a0 - a3 中。 被调用程序(diffofsums)将返回值存储到返回寄存器 v0 中。

返回64位值(例如一个双精度浮点数)的函数将使用两个返回寄存器 v0 和 v1。当调用多于 4 个参数的函数时， 多出来的输入参数将放入栈中，这个问题我们下面讨论。

###  3.栈

栈指针 sp是一个特定的MIPS寄存器，此寄存器指向栈顶（栈的最低可访问内存)。

![image-20221008195142694](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221008195142694.png)

栈的一个重要应用是保存和恢复函数使用的寄存器。函数应该计算返回值，但不应该产生其他负面影响。尤其是，除了包含返回值的寄存器 v0 （和 v1 , 如果结果为64位数）外，其他任何寄存器都不应该被修改。

在函数修改寄存器前，他要将寄存器保存在栈中，然后在返回前从栈中恢复这些寄存器。有以下步骤

1. 创建栈空间来存储一个或多个寄存器的值
2. 将寄存器的值存储在栈中
3. 使用寄存器执行函数
4. 从栈中恢复寄存器的原始值
5. 回收栈空间

diffofsums改进版

MIPS汇编代码

```
# s0 = result
diffofsums:
	addi $sp, $sp, -12		# make space on stack to store three registers
	sw $s0, 8($sp)		    # save $s0 on stack
	sw $t0, 4($sp)		    # save $t0 on stack
	sw $t1, 0($sp)		    # save $t1 on stack
	add $t0, $a0, $a1		# $t0 = f + g
	add $t1, $a2, $a3		# $t1 = h + i
	sub $s0, $t0, $t1		# result = (f + g) - (h + i)
	add $v0, $s0, $0		# put return value in $v0
	lw $t1, 0($sp)		    # restore $t1 from stack
	lw $t0, 4($sp)		    # restore $t0 from stack
	lw $s0, 8($sp)		    # restore $s0 from stack
	addi $sp, $sp, 12	    # deallocate stack space
	jr $ra				   # return to caller
```

模块化的原则告诉我们，每个函数应该只访问自己的栈框架而不应该访问其他函数的栈框架。

### 4.受保护寄存器

受保护的寄存器包括 s0 - s7。函数必须保存和恢复任何需要使用的受保护寄存器，但是可以随意改变不受保护的寄存器。

当一个函数调用另一个函数时，前者称为调用函数，后者称为被调用函数。被调用函数必须保存和恢复他要用到的受保护寄存器。被调用函数有可能改变任何不受保护寄存器，因此如果调用函数需要其不受保护寄存器中的有效数据不被改变，那么他在函数调用之前需要保存不受保护寄存器，而且还需要再调用之后恢复这些寄存器。这种情况下，受保护寄存器也可以称为被调用者保存寄存器，不受保护寄存器称为调用者保存寄存器。

注意：s0 - s7常用于保存函数中的局部变量，所以他们必须被保存。ra 也要被保存，这样函数才能知道返回到哪里。t0 - t9 用于在向局部变量赋值前保存临时结果，这些计算结果一般在函数调用之前完成，所以他们不受保护，调用函数一般不需要保存他们。a0 - a3 经常在调用函数的过程中被覆盖，因此，如果被调用的函数返回后，调用函数根据它自身参数执行，则 a0 - a3 必须由调用函数保存。v0 - v1 不用被保护，因为被调用函数将返回结果放入这些寄存器中。

![image-20221008201900131](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221008201900131.png)

### 5.递归函数的调用

在调用其他函数前，他们需要把不受保护寄存器保存在栈中，然后在调用后再恢复这些寄存器。具体来说，被调用函数保存需要修改的任意受保护寄存器( s0 - s7  和 ra )。 

下面的例子是阶乘函数的递归写法。为了方便的表明函数地址，假定函数的起始地址为0x90.

高级语言代码实现

```c
int factorial(int n){
if(n <= 1)
	return 1;
else 
	return (n * factorial(n-1));
}
```

 MIPS汇编代码实现

```
0x90  factorial: addi $sp, $sp -8 		# make room on stack
0x94			sw $a0, 4($sp)		   # store $a0
0x98			sw $ra, 0($sp)		   # store $ra
0x9C			addi $t0, $0, 2		   # $t0 = 2
0xA0			slt $t0, $a0, $t0	   # n <= 1?
0xA4			beq $t0, $0, else	   # no : go to else
0xA8			addi $v0, $0, 1		   # yes : return 1;
0xAC			addi $sp, $sp, 8	   # restore $sp
0xB0			jr $ra				  # return
0xB4  else:		addi $a0, $a0, -1	    # n = n -1 
0xB8			jal factorial		   # recursive call
0xBC			lw $ra, 0($sp)		   # restore $ra
0xC0			lw $a0, 4($sp)		   # retstore $a0
0xC4			addi $sp, $sp, 8        # restore $sp
0XC8			mul $v0, $a0, $v0	   # n * factorial (n-1)
0XCC			jr $ra				  # return
```

注意函数开始前保存变量并申请栈空间，函数结束后恢复变量回收栈空间。

下图显示了执行 factorial (3) 时的情况。假定 sp 最初指向 0xFC 。函数创建两个字的栈空间来保存 a0 和 ra。在第一次调用时，factorial 将 a0 ( a0 中保存着 n = 3) 保存在 0xF8 中，将 ra 保存在 0xF4中，然后函数将 a0中的内容改变为 n = 2 并递归调用factorial(2), 使 ra 保存在0xBC。在第二次调用时，factorial 将 a0 ( a0 中保存着 n = 2)保存在 0xF0 中，将 ra 保存在 0xEC 中。这时，我们知道了 ra 中存储了0xBC。 然后将 a0 中的内容改变为 n = 1 并递归调用 factorial (1) 。在第三次调用时，factorial 将 a0 ( a0 中保存这 n = 1)保存在0xE8 中，将 ra 保存在 0xE4 中。这时， ra 存储的还是 0xBC 。factorial 的第三次调用返回保存在 v0 中的1，并且在返回到第二次调用前回收栈空间。第二次调用将 n 恢复为2， 将 ra 恢复为 0xBC ( ra 中已经是这个值了)， 然后回收栈帧， 返回 v0 = 2 * 1 = 2 给第一次调用。第一次调用将 n 恢复为3， 将 ra 恢复为调用函数的返回地址，回收栈帧， 返回 v0 = 3 * 2 = 6。下图显示了递归调用函数返回时栈的情况。当 factorial 返回到调用函数时栈指针指向它的初始位置 ( 0xFC ),指针上的栈空间的内容没有变化，而且所有受保护寄存器保存他们的初始值, v0 保存返回值6.

  ![image-20221009235507046](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221009235507046.png)

### 6.附加参数和局部变量

依照MIPS惯例，如果一个函数有4个以上的参数，则前4个参数像往常一样存储在寄存器中，额外的参数使用栈指针之上的空间保存在栈中。调用函数( caller )必须扩展栈空间来满足额外的参数，下图描述了调用多于4个参数的函数时栈的情况。

![image-20221010000830266](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221010000830266.png)

函数也可以声明局部变量和数组，局部变量在一个函数内部定义并且只能在该函数内部使用。局部变量存储在 s0 - s7中。如果有许多局部变量，他们也可以存储在这个函数的栈空间中。尤其是，局部数组存储在栈中。

 上图给出了被调用函数的栈框架帧。栈框架保存函数自己的参数、返回地址、函数要修改的保存器。他还存储局部数组和额外的局部变量。如果被调用函数有4个以上的参数，他可以从调用函数的栈帧中找到他们。访问额外的输入参数是一种特殊情况，在这种情况下函数可以访问不属于自己栈帧中的数据。=