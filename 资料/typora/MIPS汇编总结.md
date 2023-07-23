# MIPS汇编总结

## end the program

```
# end the program
.macro end
	li $v0, 10
	syscall
.end_macro
```

## write & read 类

```
# printInt
.macro printInt(%src)
	move $a0, %src
	li $v0, 1
	syscall
.end_macro

# getInt
macro getInt(%des)
	li $v0, 5
	syscall
	move %des, %v0
.end_macro

#直接打印字符串
.macro printStr(%str)
	.data
		tmpLabel: .asciiz %str
	.text
		li $v0, 4
		la $a0, tmpLabel
		syscall
.end_macro
栗子：
printStr("Hello CO!");

# 打印特定地址的字符串
.macro printStrOf(%src)
	li $v0, 4
	la $a0, %src
	syscall
.end_macro
栗子：
.data
	myStr: .asciiz "Hello CO!"
.text
	printStrOf(myStr)
```

## push & pop类

```
# push
.macro push(%src)
	sw %src, 0($sp)
	addi $sp, $sp, -4
.end_macro
# pop
.macro pop(%des)
	addi $sp, $sp, 4
	lw	%des, 0($sp)
.end_macro
栗子:
	fun:
	push($ra)
	push($s0)
	push($t0)
	...
	pop($t0)
	pop($s0)
	pop($ra)
	jr $ra
```

## getIndex类

```
# get the vector index
.macro get_vector_index(%index, %x)
	sll		%index, %x, 2
.end_macro
# get the martix index
.macro get_martix_Index(%ans, %i, %j)
	li %ans, your_number_of_columns
	multu %i, %ans
	mflo %ans
	add %ans, %ans, %j
	sll %ans, %ans, 2
.end_macro
```



## 将常用的高级语言转化为汇编

1. **if 语句**

```
# 跳转语句 (condition) 
```

