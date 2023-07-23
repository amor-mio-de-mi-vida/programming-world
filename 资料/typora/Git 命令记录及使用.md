# Git 命令记录及使用

## 基本概念

1.工作区：仓库的目录。工作区时独立于各个分支的。

2.暂存区数据暂时存放的区域，类似于工作区写入版本库前的缓存区。暂存区时独立于各个分支的。

3.版本库：存放所有已经提交到本地仓库的代码版本。

4.版本结构：树结构，树种每个结点代表一个代码版本。

## 常用指令与流程、

### 准备工作：

#### 创建一个新的仓库：

先创建一个新的文件夹，在文件夹种打开git bash

```bash
git init
```

将本地的仓库和服务器关联

```bash
git remote add origin 服务器地址
```

删除关联：

```bash
git remote rm origin
```

拉取服务器最新的版本

```bash
git pull origin master
```

对该仓库的代码进行修改

之后再按老流程提交

```bash
git add . #将文件添加到暂存区
git commit -m "简介" #将暂存区的内容提交到当前分支
git push -u origin master (如果第一次push用这个代码)
git push	
```

### 分支：

#### 查看分支：

```bash
git branch`
```

绿色的或最后代表当前所在的分支

#### 创建新分支

```bash
git branch "分支名"
```

这里的分支最好用动词+名词的形式来创建，这样可以使leader查看你的分支可以通过名字就知道你这个分支改的什么内容。

#### 切换到你的新分支

```bash
git checkout "分支名"
```

#### 查看当前修改的文件

```bash
git status
```

1.如果无修改，只有三行

2.修改了文件，就会出现下面的：change not staged for commit

![1663252375277](C:\Users\W\Desktop\typora\photo\1663252375277.png)

此时的修改仅仅是你本地的文件改动了，并没有保存到你新建的分支中，所以是红色的。

如果你不想要改动，可以通过命令来还原：

```bash
git checkout --改动的文件（记得加上路径）
git checkout -- . #表示还原所有修改了的文件
```

#### 将修改的文件添加到新的分支

```bash
git add .
```

. 表示添加所有的修改到新分支，你也可以通过红色字体路径来添加

```bash
git add tools/voise.sh
```

添加完成后，再使用git status 查看修改的文件，就显示为绿色的了，表示添加成功了

添加文件的时候出现问题：已经添加文件，但是git status识别不到，导致提交后没有新加的文件。

解决办法：使用命令强制添加

```bash
git add -f 文件名
```

#### 添加分支的描述信息

```bash
git commit -m "描述信息（你这个分支是干嘛的，实现了什么功能，或者解决了什么问题）"
```

后续可能用到的参数

```bash
git commit --amend
```

使用这个参数是对你上次提交分支的内容进行修改。

> git commit -- amend 就是解决一些很小的改动，使得你不想再重现创建一个commit而设定。如果你的commit已经push到了远程仓库，那么使用-amend修改commit后，gitpush时一定要使用--force-with-lease参数。否则就会报错。

使用该参数后的提交方法：

```bash
git push --force-with-lease 目标分支名 你的分支名
```

#### 提交到远程

```bash
git push 目的分支名 你的分支名
```

## 具体的流程图：

![微信图片_20220915214331](C:\Users\W\Desktop\typora\photo\微信图片_20220915214331.png)



![微信图片_20220915211300](C:\Users\W\Desktop\typora\photo\微信图片_20220915211300.jpg)



## 打开SpinalHDL教程的方法：

```bash
: git clone https://github.com/jijingg/Spinal-bootcamp
: cd Spinal-bootcamp
: jupyter notebook &
```

用jupyter-notebook打开文件。

## 遇到的问题：

1.ssh密钥的问题

2.Unable to create 'C:/Users/W/.git/index.lock': File exists.问题

这个仓库中似乎正运行着另一个Git进程。例如：通过 ‘git commit’ 命令打开的某个编辑器。‘

打开的文件夹不对。

3.warning: adding embedded git repository问题

将默认隐藏的.git文件夹删掉就好。

4.第一次提交之前要先pull一下？将远程分支与当前分支合并。

