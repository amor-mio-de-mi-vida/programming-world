# Typora快捷键一览

----------------------------------------------------------

## 基本功能快捷键

### 字体操作快捷键

| 功能     | 快捷键      |
| -------- | ----------- |
| 字体加粗 | Ctrl+B      |
| 下划线   | Ctrl+U      |
| 倾斜     | Ctrl+I      |
| 删除线   | Alt+Shift+5 |

### 插入功能快捷键

| 功能                           | 快捷键       |
| ------------------------------ | ------------ |
| 插入图片（本地图片可直接拖入） | Ctrl+Shift+I |
| 插入表格                       | Ctrl+T       |
| 插入有序列表                   | Ctrl+Shift+[ |
| 插入无序列表                   | Ctrl+Shift+] |
| 插入超链接                     | Ctrl+K       |
| 插入代码片                     | Ctrl+Shift+` |
| 插入代码块                     | Ctrl+Shift+K |
| 插入公式块                     | Ctrl+Shift+M |
| 插入引用块                     | Ctrl+Shift+Q |

### 标题段落快捷键

markdown支持六级标题，可以使用Ctrl+数字 指定不同层次的标题和段落

| 功能                    | 快捷键   |
| ----------------------- | -------- |
| 一级标题                | Ctrl+1   |
| 二级标题                | Ctrl+2   |
| 三-六级标题（以此类推） | Ctrl+3-6 |
| 提升标题级别            | Ctrl+‘+' |
| 降低标题级别            | Ctrl+'-' |
| 段落（正文）            | Ctrl+0   |

--------------------------------------------------

## 拓展操作快捷键

### 表格扩展快捷键

在表格中，可以使用鼠标拖动行或者列，达到交换行和列的目的

同时也可以使用快捷键操作

| 功能       | 快捷键                       |
| ---------- | ---------------------------- |
| 下方插入行 | Ctrl+Enter                   |
| 上移该行   | Alt+↑                        |
| 下移该行   | Alt+↓                        |
| 左移该列   | Win键+←                      |
| 右移该列   | Win键+→                      |
| 删除该行   | Ctrl+Shift+退格键(BackSpace) |

左右移动表格列的快捷键与WinDows系统自带的快捷键冲突，导致失效

-------------------------------------

## 视图操作快捷键

### 侧边栏

| 功能             | 快捷键         |
| ---------------- | -------------- |
| 大纲视图         | Ctrl+Shift+1   |
| 文件列表视图     | Ctrl+Shift+2   |
| 文件树视图       |                |
| 显示/隐藏侧边栏  |                |
| 放大视图         | Ctrl+Shift+‘+’ |
| 缩小视图         | Ctrl+Shift+‘-’ |
| 恢复原来大小视图 | Ctrl+Shift+9   |

### 编辑模式

| 功能                                     | 快捷键 |
| ---------------------------------------- | ------ |
| 源代码模式                               | Ctrl+/ |
| 专注模式（当前编辑行为黑，其他行为灰色） | F8     |
| 打字机模式（光标始终在屏幕中央位置）     | F9     |

### 其他

| 功能           | 快捷键    |
| -------------- | --------- |
| 全屏           | F11       |
| 应用内窗口切换 | Ctrl+Tab  |
| 开发者工具     | Shift+F12 |

-----------------------------------------

## 搜索扩展快捷键

| 功能       | 快捷键   |
| ---------- | -------- |
| 查找/搜索  | Ctrl+F   |
| 替换       | Ctrl+H   |
| 查找下一个 | F3       |
| 查找上一个 | Shift+F3 |

## 基本操作快捷键

### 选择操作

| 功能             | 快捷键       |
| ---------------- | ------------ |
| 全选             | Ctrl+A       |
| 选择当前行/句    | Ctrl+L       |
| 选择当前格式文本 | Ctrl+E       |
| 选择当前单词     | Ctrl+D       |
| 删除当前单词     | Ctrl+Shift+D |

### 跳转操作

| 功能           | 快捷键    |
| -------------- | --------- |
| 跳转到文首     | Ctrl+Home |
| 跳转到所选内容 | Ctrl+J    |
| 跳转到问末     | Ctrl+End  |

### 粘贴复制操作

| 功能                   | 快捷键       |
| ---------------------- | ------------ |
| 普通复制               | Ctrl+C       |
| 普通粘贴               | Ctrl+V       |
| 剪切                   | Ctrl+X       |
| 复制为MarkDown标记语法 | Ctrl+Shift+C |
| 粘贴为纯文本           | Ctrl+Shift+V |

### 文件操作

| 功能               | 快捷键       |
| ------------------ | ------------ |
| 新建               | Ctrl+N       |
| 新建窗口           | Ctrl+Shift+N |
| 打开文件           | Ctrl+O       |
| 快速打开           | Ctrl+P       |
| 保存               | Ctrl+S       |
| 另存为             | Ctrl+Shift+S |
| 偏好设置           | Ctrl+，      |
| 关闭               | Ctrl+W       |
| 重新打开关闭的文件 | Ctrl+Shift+T |

---------------------------------------

## 其他

| 功能     | 快捷键 |
| -------- | ------ |
| 清楚样式 | Ctrl+\ |
| 增加缩进 | Ctrl+] |
| 减少缩进 | Ctrl+[ |

-------------------------------------

![微信图片_20220915211246](C:\Users\W\Desktop\typora\photo\微信图片_20220915211246.png)

![微信图片_20220915211256](C:\Users\W\Desktop\typora\photo\微信图片_20220915211256.png)



## Markdown数学公式语法

### 行内与独行

1.行内公式：将公式插入到本行内，符号：$公式内容$,如：

```markdown
$xyz$
```

2.独行公式：将公式插入到新的一行内，并且居中，符号：$$公式内容$$，如：

```markdown
$$xyz$$
```

### 上标，下标与组合

1. 上标符号，符号：`^`，如：$x^4$
2. 下标符号，符号：`_`，如：$x_1$
3. 组合符号，符号：`{}`，如：${16}_{8}O{2+}_{2}$

### 汉字、字体与格式

1.汉字形式，符号：`\mbox{}`，如：$V_{\mbox{初始}}$

2.字体控制，符号：`\displaystyle`，如：$\displaystyle \frac{x+y}{y+z}$

3.下划线符号，符号：`\underline`，如：$\underline{x+y}$

4.标签，符号`\tag{数字}`，如：$\tag{11}$

5.上大括号，符号：`\overbrace{算式}`，如：$\overbrace{a+b+c+d}^{2.0}$

6.下大括号，符号：`\underbrace{算式}`，如：$a+\underbrace{b+c}_{1.0}+d$

7.上位符号，符号：`\stacrel{上位符号}{基位符号}`，如：$\vec{x}\stackrel{\mathrm{def}}{=}{x_1,\dots,x_n}$

例子：
$$
\vec{x}\stackrel{\mathrm{def}}{=}({x_1,\dots,x_n})
$$

### 占位符

1.两个quad空格，符号：`\qquad`，如：$x \qquad y$

2.quad空格，符号：`\quad`，如：$x \quad y$

3.大空格，符号`\`，如：$x \  y$

4.中空格，符号`\:`，如：$x : y$

5.小空格，符号`\,`，如：$x , y$

6.没有空格，符号``，如：$xy$

7.紧贴，符号`\!`，如：$x ! y$

### 定界符与组合

1.括号，符号：`（）\big(\big) \Big(\Big) \bigg(\bigg) \Bigg(\Bigg)`，如：$（）\big(\big) \Big(\Big) \bigg(\bigg) \Bigg(\Bigg)$

2.中括号，符号：`[]`，如：$[x+y]$

3.大括号，符号：`\{ \}`，如：${x+y}$

4.自适应括号，符号：`\left \right`，如：$\left(x\right)$，$\left(x{yz}\right)$

5.组合公式，符号：`{上位公式 \choose 下位公式}`，如：${n+1 \choose k}={n \choose k}+{n \choose k-1}$

6.组合公式，符号：`{上位公式 \atop 下位公式}`，如：$\sum_{k_0,k_1,\ldots>0 \atop k_0+k_1+\cdots=n}A_{k_0}A_{k_1}\cdots$
$$
{n+1 \choose k}={n \choose k}+{n \choose k-1}
$$

$$
\sum_{k_0,k_1,\ldots>0 \atop k_0+k_1+\cdots=n}A_{k_0}A_{k_1}\cdots
$$

### 四则运算

1.加法运算，符号：`+`，如：$x+y=z$

2.减法运算，符号：`-`，如：$x-y=z$

3.加减运算，符号：`\pm`，如：$x \pm y=z$

4.减加运算，符号：`\mp`，如：$x \mp y=z$

5.乘法运算，符号：`\times`，如：$x \times y=z$

6.点乘运算，符号：`\cdot`，如：$x \cdot y=z$

7.星乘运算，符号：`\ast`，如：$x \ast y=z$

8.除法运算，符号：`\div`，如：$x \div y=z$

9.斜法运算，符号：`/`，如：$x/y=z$

10.分式表示，符号：`\frac{分子}{分母}`，如：$\frac{x+y}{y+z}$

11.分式表示，符号：`{分子} \voer {分母}`，如：${x+y} \over {y+z}$

12.绝对值表示，符号：`||`，如：$|x+y|$
$$
{x+y} \over {y+z}
$$

### 高级运算

1.平均数运算，符号：`\overline{算式}`，如：$\overline{xyz}$

2.开二次方运算，符号：`\sqrt`，如：$\sqrt x$

3.开方运算，符号：`\sqrt[开方数]{被开方数}`，如：$\sqrt[3]{x+y}$

4.对数运算，符号：`\log`，如：$\log(x)$

5.极限运算，符号：`\lim`，如：$\lim^{x \to \infty}_{y \to 0}{\frac{x}{y}}$

6.极限运算，符号：`\displaystyle \lim`，如：$\displaystyle \lim^{x \to \infty}_{y \to 0}{\frac{x}{y}}$

7.求和运算，符号：`\sum`，如：$\sum^{x \to \infty}_{y \to 0}{\frac{x}{y}}$

8.求和运算，符号：`\displaystyle \sum`，如：$\displaystyle \sum^{x \to \infty}_{y \to 0}{\frac{x}{y}}$

9.积分运算，符号：`\int`，如：$\int^{\infty}_{0}{xdx}$

10.积分运算，符号：`\displaystyle \int`，如：$\displaystyle \int^{\infty}_{0}{xdx}$

11.微分运算，符号：`\partial`，如：$\frac{\partial x}{\partial y}$

12.矩阵表示，符号：`\begin{matrix} \end{matrix}`，如：$\left[ \begin{matrix} 1 &2 &\cdots &4\5 &6 &\cdots &8\\vdots &\vdots &\ddots &\vdots\13 &14 &\cdots &16\end{matrix} \right]$
$$
\displaystyle \int^{\infty}_{0}{xdx}
$$

$$
\left[ \begin{matrix} 1 &2 &\cdots &4\5 &6 &\cdots &8\\vdots &\vdots &\ddots &\vdots\13 &14 &\cdots &16\end{matrix} \right]
$$

### 逻辑运算

1.等于运算，符号：`=`，如：$x+y=z$

2.大于运算，符号：`>`，如：$x+y>z$

3.小于运算，符号：`<`，如：$x+y<z$

4.大于等于运算，符号：`\geq`，如：$x+y \geq z$

5.小于等于运算，符号：`\leq`，如：$x+y \leq z$

6.不等于运算，符号：`\neq`，如：$x+y \neq z$

7.不大于等于运算，符号：`\ngeq`，如：$x+y \ngeq z$

8.不大于等于运算，符号：`\not\geq`，如：$x+y \not\geq z$

9.不小于等于运算，符号：`\nleq`，如：$x+y \nleq z$

10.不小于等于运算，符号：`\not\leq`，如：$x+y \not\leq z$

11.约等于运算，符号：`\approx`，如：$x+y \approx z$

12.恒定等于运算，符号：`\equiv`，如：$x+y \equiv z$

### 集合运算

1.属于运算，符号：`\in`，如：$x \in y$

2.不属于运算，符号：`\notin`，如：$x \notin y$

3.不属于运算，符号：`\not\in`，如：$x \not\in y$

4.子集运算，符号：`\subset`，如：$x \subset y$

5.子集运算，符号：`\supset`，如：$x \supset y$

6.真子集运算，符号：`\subseteq`，如：$x \subseteq y$

7.非真子集运算，符号：`\subsetneq`，如：$x \subsetneq y$

8.真子集运算，符号：`\supseteq`，如：$x \supseteq y$

9.非真子集运算，符号：`\supsetneq`，如：$x \supsetneq y$

10.非子集运算，符号：`\not\subset`，如：$x \not\subset y$

11.非子集运算，符号：`\not\supset`，如：$x \not\supset y$

12.并集运算，符号：`\cup`，如：$x \cup y$

13.交集运算，符号：`\cap`，如：$x \cap y$

14.差集运算，符号：`\setminus`，如：$x \setminus y$

15.同或运算，符号：`\bigodot`，如：$x \bigodot y$

16.同与运算，符号：`\bigotimes`，如：$x \bigotimes y$

17.实数集合，符号：`\mathbb{R}`，如：`\mathbb{R}`

18.自然数集合，符号：`\mathbb{Z}`，如：`\mathbb{Z}`

19.空集，符号：`\emptyset`，如：$\emptyset$

### 数学符号

1.无穷，符号：`\infty`，如：$\infty$

2.虚数，符号：`\imath`，如：$\imath$

3.虚数，符号：`\jmath`，如：$\jmath$

4.数学符号，符号`\hat{a}`，如：$\hat{a}$

5.数学符号，符号`\check{a}`，如：$\check{a}$

6.数学符号，符号`\breve{a}`，如：$\breve{a}$

7.数学符号，符号`\tilde{a}`，如：$\tilde{a}$

8.数学符号，符号`\bar{a}`，如：$\bar{a}$

9.矢量符号，符号`\vec{a}`，如：$\vec{a}$

10.数学符号，符号`\acute{a}`，如：$\acute{a}$

11.数学符号，符号`\grave{a}`，如：$\grave{a}$

12.数学符号，符号`\mathring{a}`，如：$\mathring{a}$

13.一阶导数符号，符号`\dot{a}`，如：$\dot{a}$

14.二阶导数符号，符号`\ddot{a}`，如：$\ddot{a}$

15.上箭头，符号：`\uparrow`，如：$\uparrow$

16.上箭头，符号：`\Uparrow`，如：$\Uparrow$

17.下箭头，符号：`\downarrow`，如：$\downarrow$

18.下箭头，符号：`\Downarrow`，如：$\Downarrow$

19.左箭头，符号：`\leftarrow`，如：$\leftarrow$

20.左箭头，符号：`\Leftarrow`，如：$\Leftarrow$

21.右箭头，符号：`\rightarrow`，如：$\rightarrow$

22.右箭头，符号：`\Rightarrow`，如：$\Rightarrow$

23.底端对齐的省略号，符号：`\ldots`，如：$1,2,\ldots,n$

24.中线对齐的省略号，符号：`\cdots`，如：$x_1^2 + x_2^2 + \cdots + x_n^2$

25.竖直对齐的省略号，符号：`\vdots`，如：$\vdots$

26.斜对齐的省略号，符号：`\ddots`，如：$\ddots$

### 希腊字母

![微信图片_20220916000554](C:\Users\W\Desktop\typora\photo\微信图片_20220916000554.png)