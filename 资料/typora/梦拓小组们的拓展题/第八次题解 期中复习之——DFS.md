# 期中复习之——DFS

又是复习(预习)递归函数~

PS：这道题超级超级超级经典。

## 全排列问题

### 题目描述

按照字典序输出自然数 $1$ 到 $n$ 所有不重复的排列，即 $n$ 的全排列，要求所产生的任一数字序列中不允许出现重复的数字。

### 输入格式

一个整数 $n$。

### 输出格式

由 $1 \sim n$ 组成的所有不重复的数字序列，每行一个序列。

每个数字保留 $5$ 个场宽。

### 样例 #1

#### 样例输入 #1

```
3
```

#### 样例输出 #1

```
1    2    3
    1    3    2
    2    1    3
    2    3    1
    3    1    2
    3    2    1
```

### 提示

$1 \leq n \leq 9$。

![image-20221011110053057](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221011110053057.png)

![image-20221011110113736](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221011110113736.png)

## 代码：

```c
#include  <stdio.h>
int check[15];
int a[10];
void dfs(int m);
//遍历、递归函数，对箱子里扑克牌的数量递归
int n;
int cnt;

int main()
{
	scanf("%d",&n);
	dfs(1);
	return 0;
}

void dfs(int m){
	int i;
	if(m<=n){        	//箱子还没满，开始递归
	for(i=1;i<=n;i++){	//从1到n按顺序遍历
		if(!check[i]){	//如果没有被标记
			a[m]=i;		//把扑克牌i放到第m个箱子里
			check[i]=1;	//把这个扑克牌标记(已选过)
			dfs(m+1);	//继续挑第m+1个箱子里的扑克牌
			check[i]=0;	//把扑克牌i拿出来，探索其他的情况
		}
	}
	}
	else{			   //箱子满了，把结果print出来
		for(i=1;i<=n;i++){
			printf("%5d",a[i]);		//注意printf函数的小细节
		}
		printf("\n");
	}
}
```

DFS本质是把所有情况全都递归的遍历一遍，大家还是要好好理解这个递归的过程