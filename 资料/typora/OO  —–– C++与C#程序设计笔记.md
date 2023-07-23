# OO  —–– C++与C#程序设计笔记

## 第一次作业

### 第一部分：训练目标

- 学会构建构造方法

  Java 类使用变量定义数据状态，使用方法定义行为。除此之外，类还提供了一种称为构造方法(constructor)的特殊方法，用于创建新对象。作为类的方法，构造方法虽然可以完成任何动作，但是构造方法的目标是为了完成初始化，因此构造方法的实现代码具有显著的特征，即对类中定义的成员变量进行初始化。按照变量的类型要区分两种情况：

  - 原子类型的成员变量：一般是直接使用编程语言内置的数据类型所声明的变量，如 int ，boolean 等。这种成员变量一般可以直接赋值，如 `int price = 10`。
  - 复合类型的成员变量：一般是使用编程语言类库或用户自定义的类(class)来声明的成员变量，如 `ArrayList myList` 。**这种成员变量无法直接指定一个赋值结果，通常需要调用相应类型的构造方法来获得相应的初始值**，如 `myList = new ArrayList()` 。

- **对类进行封装，理解封装的作用**

  封装是面向对象方法的一个重要特征，强调对外隐藏对象的内部属性和实现细节，仅对外提供公共访问方式。这样做的优点是提高类的可复用性、安全性。

  关键字 private 、protected 和 public 可以进一步对类的成员（包括变量和方法）的可见范围，被 private 修饰的成员只能在本类中使用；protected修饰的成员可以在本类及其直接子类使用；public修饰的成员可以在任意类中直接使用。一般而言，根据封装原则，如果没有特别的针对性考虑，建议对所有的成员变量使用private进行限定。

- **建立一个对象的集合，实现向集合中增加对象和访问集合中对象的操作，学习容器的使用和选择，熟悉对容器的操作**

### 第二部分：预备知识

关于 Java：

> Java 是一门十分强大的语言，具有跨平台、安全等特点。Java语言得到广泛使用的一个重要原因是提供了丰富的类库，作为初学者你要养成多查阅和使用 Java 所提供的类的习惯，不要重复造轮子。在 pre 训练中，大家可以使用 Java 提供的相关容器，如 ArrayList、HashMap 等，会取得事半功倍的效果。

关于面向对象：

> 面向对象是一种主流的软件开发方法，也是一种思维方式，其核心是识别类，并在类之间建立层次式的协作关系。面向对象思维需要逐步养成，是本课程的核心目标。作为 pre ，主要还是通过一些小的迭代式练习来初步感受面向对象方法。因为 pre 训练题目相对比较简单，通过传统上的结构化程序开发方法也可以实现代码，甚至可以只写一个函数就能完成任务，但希望大家不要这样做。千里之行，始于足下，希望大家从一开始就体会面向对象开发的特点，这也是 pre 训练的重要目标。

关于容器：

> 在 Java 中，我们有更强大的“数组”——**容器**，它提供了更多管理多个对象的方法。和数组相比，容器可以动态控制容量、方便地增加或删除元素、方便地对元素进行进一步的管理

在开始前，你需要先了解 java 的基础语法，包括表达式、for 循环、if 语句、输入输出、类，并学会编译运行 java 程序，Java 语言的很多成分都和 C 语言相似。

在任务迭代的开发过程中，会不可避免的涉及到变量名更改的问题，请尽量利用 IDEA 提供的重命名工具(Shift + F6)，不要自己手动一个个改。

### 第三部分：基本概念

#### 一、从 C 到 Java

其实无论是 while、for 还是 if， switch，Java 与 C 基本上都是相同的，在你的简单预习中相信也发现了这个现象。在我们正式开始完成任务之前，再在 Java 与 C 关联的方面做一些简单介绍。

如下例子所示，Java 中的方法 `public static void main(String[] argv)`，就相当于C语言的入口函数`main()` 。这个 `main` 是你 Java 主程序的执行入口，当运行 Java 程序时，你可以理解成将会从此处开始执行。事实上，可以有多个类中包含 `main` 方法，**我们可以手动制定一个类中的 `main` 方法作为程序入口。**因此，`main`方法所在类的类名称也可以不是例子中的 `MainClass`。 为了方便评测，在 OO 课程中，**请你保证 `main` 方法出现且只出现在一个类中**。

```java
import java.util.Scanner;

public class MainClass {
    public static void main(String[] argv) {
        Scanner scanner = new Scanner(System.in);
        int a = scanner.nextInt();
        double b = scanner.nextDouble();
        String c = scanner.next();
        System.out.println("Hello world!");
        System.out.println(c + b + a);
    }
}
```

现在我们可以运行一下这个程序，程序中已经包含了 Java 程序的输入输出方式，你也可以换一些组合方式，来进一步体会 Java 的输入输出。

#### 二、构造一个类

现在我们要正式开始本次任务了，在本次任务中我们希望构造一个 **Bottle** 类，来表示冒险者需要的物品，要求 **Bottle** 类包含以下属性：标识(`id`: 整数)，名字(`name`: 字符串)，价格(`price`: 整数)，容量(`capacity`: 浮点数)，和表达瓶子是否装满的标志量(`filled`: 布尔值)。 从某种意义上来说，只包含属性的类其实与 C 语言的结构体是很相似的。

以 Bottle 举例，构造一个类，代码如下：

```java
class Bottle {
    private int id;
    private String name;
    private long price;
    private double capacity;
    private boolean filled;

    public int getId(){
        return this.id;
    }

    public void setId(int id){
        this.id=id;
    }
}
```

我们会发现，所有属性均是私有的，外部完全看不到它们，这时，如果在主类中声明了一个 Bottle 的实例 bottle ，无法对 name 进行 `bottle.name` 的操作。如前所述，面向对象开发强调封装和私有保护，**我们一般不允许把属性定义成 public 的。**面向对象方法的基本特点是私有化保护内部数据，**暴露对数据的必要操作接口**，多数情况下可以提供 `setter-getter` 方法。但是需要注意，如果某个属性的取值不能允许外部进行无限制的修改，就不能提供公开的 setter 方法。

那么现在就是练习时间啦，请你把所有属性都私有化，将 Bottle 封装起来！并配置好相应的方法让他们能够被外部更改和访问。

> 小 tips：在之后的作业中，如果你对很多变量都需要重复实现 get 和 set 方法，挨个输入比较麻烦，有兴趣搜索一下 IDEA 的**一键生成方法（generate) 功能**，高效编码

#### 三、实例化

我们现在拥有了一个 Bottle 类，那么问题来了，怎么在 MainClass 里引用他呢？我们可以把 Bottle 想像成一个像 `int`、`char` 一样的变量类型。那么我们就可以使用这条语句：`Bottle bottle;` 来声明一个`Bottle`变量了。在Java中，声明的对象**变量**就像是C语言中的一个结构体**指针**，**如果你不对其初始化那么这个变量就会指向一个 `null` 量，代表这是一个空指针**，此时还没有任何内存空间被分配用于存储一个Bottle的信息，你还需要使用**构造函数**实例化一个对象。 代码如下：

```java
public class MainClass {
    public static void main(String[] argv) {
        Bottle bottle = new Bottle();  //new Bottle() 即构造函数
        bottle.setName("Cola");
        bottle.setPrice(3);
        bottle.setId(1);
        bottle.setCapacity(100.0);
        bottle.setFilled(true);
    }
}
```

构造函数的用途是在你需要创建一个对象的时候完成一些初始化工作，并给对象的所有属性赋予初始值。

虽然 Java 语言默认为每个类提供一个缺省的构造方法，但是你并不确定这个缺省构造方法把每个属性设置成什么初值。对于上述的 Bottle **缺省构造方法**而言，把 id 初始化为 0，把 price 初始化为 0，把 name 初始化为 null，把 capacity 初始化为0.0，把 filled 初始化为 false。我们建议显式方式来实现自己所需的合适的构造方法，确保得到的对象初始状态直观可见且可控。

**在类中以"public 类名(参数列表)"的方式定义的函数就是构造函数。**

这里举一个长方体类的例子：

```java
public class CuboidBox {
    private double length;
    private double width;
    private double height;

    // 构造函数
    public CuboidBox(double length, double width, double height) {
        this.length = length;
        this.width = width;
        this.height = height;
    }
}
```

那么现在就是练习时间啦，请你为 Bottle 写一个构造函数，要求该构造函数可以传入四个参数为 Bottle 赋值，同时需要初始化是否装满变量 `filled` 为 `true`。

#### 四、容器

既然我们拥有了装满药水的瓶子对象，那么自然也要拥有能够持有与使用它的人，这就是我们的冒险者。然而冒险者从来都得准备充分：在怪物面前若只有一瓶恢复药水，难免会疲于招架。因此冒险者可能会携带多个瓶子。那么，应当如何管理这些瓶子对象呢？

能够想到，数组可以完成这样的管理。不过在 Java 中，我们有更强大的“数组”——**容器**，它提供了更多**管理多个对象的方法**。以 `ArrayList` 为例，一个冒险者身上的瓶子可以以如下方式管理：

```java
public class MainClass {
    public static void main(String[] args) {
        // 以 ArrayList 为例，展示容器的用法

        // 1. 创建容器。大部分容器都会随着元素的加入自动扩容。
        ArrayList<Bottle> bottles = new ArrayList<>();

        // 2. 加入一个元素
        Bottle bottle = new Bottle();
        bottles.add(bottle);

        // 3. 判断元素是否在容器内
        if (bottles.contains(bottle)) {
            System.out.println("We have such a bottle!");
        }

        // 4. 遍历容器中的所有元素
        for (Bottle item : bottles) {
            System.out.println(item.getName());
        }

        // 5. 输出容器规模
        System.out.println(bottles.size());

        // 6. 删除一个元素
        bottles.remove(bottle);
    }
}
```

对于经常需要使用或添加药水瓶子的冒险者来说，使用容器是不二之选。注意到冒险者和瓶子同样是对象，并拥有一个 ID 与名字，因此我们可以将冒险者也封装为一个类：

```java
class Adventurer {
    private int id;
    private String name;
    private ArrayList<Bottle> bottles;
}
```

除了 `ArrayList` 外，还有 `HashMap`、`TreeMap`、`HashSet`、`TreeSet`等常用容器。

### 第四部分：题目描述

先介绍 pre1的背景故事。

想象你是一个冒险者，现在正在一个新的星球上进行探险，这个过程中你需要努力收集各种物品来不断增强自身能力值。在第一个 task 中你需要完成两个任务：

- 对基本物品 Bottle 和冒险者 Adventurer 进行建模
- 利用容器的知识，管理多个冒险者

首先，你需要构造一个 **Bottle** 类，来表示冒险者需要用到的瓶子类，要求 **Bottle** 类包含属性：ID，名字，价格，容量，和表达瓶子是否装满的标志量。

接着，再构造一个**Adventurer**类，用来表示冒险者类，要求**Adventurer**类包含属性：ID，名字，承载多个Bottle的容器。

在这个问题中，你需要管理多个冒险者。初始时，你没有需要管理的冒险者。接下来会有 1212 个操作：

1. 加入一个需要管理的冒险者
2. 给某个冒险者增加一个瓶子
3. 删除某个冒险者的某个瓶子
4. 更新某个冒险者所持有的某个瓶子的价格
5. 更新某个冒险者所持有的某个瓶子是否装满
6. 查询某个冒险者所持有的某个瓶子的名字
7. 查询某个冒险者所持有的某个瓶子的价格
8. 查询某个冒险者所持有的某个瓶子的容量
9. 查询某个冒险者所持有的某个瓶子是否装满
10. 输出某个冒险者所持有的某个瓶子的字符串描述
11. 查询某个冒险者所持有瓶子的价格之和
12. 查询某个冒险者所持有瓶子价格的最大值

操作1-5不需要任何输出，只需要对操作 6-12 进行输出回答。

### 第五部分：输入/输出格式

第一行一个整数 m*m*，表示操作的个数。

接下来的 m*m* 行，每行一个形如 `{type} {attribute}` 的操作，`{type}` 和 `{attribute}` 间、若干个 `{attribute}` 间使用若干个空格分割，操作输入形式及其含义如下：

![image-20220918235626301](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918235626301.png)

#### 一、数据范围与操作限制

##### 变量约束

![image-20220918235722331](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220918235722331.png)

##### 操作约束

- **保证所有冒险者与瓶子的 ID 两两不同。**
- 操作 2-12：保证冒险者 ID 一定存在。
- 操作 3-10：冒险者一定持有该 ID 的瓶子。
- 操作 11：若冒险者不持有任何瓶子，则输出 0。
- 操作 12：冒险者持有至少一个瓶子。
- 操作数满足 1 \leq m \leq 20001≤*m*≤2000。

#### 二、测评方法

输出数值时，你的输出数值需要和正确数值相等。

**假设你的输出值 $x_{out}$和正确数值 $x_{std}$之间的绝对或相对误差小于等于$10^{-5}$，则认为是相等的，即满足**
$$
\frac{|x_{std}-x_{out}|}{max(1,|x_{std}|)}\leq10^{-5}
$$

### 第六部分：提示

- 容器部分大家需要熟悉对容器的操作，题目中限制了所有对象（冒险者、瓶子）的 ID 不会相同，思考一下，哪种容器会更加适合本次任务？或者说哪些容器呢？
- 在本次作业中我们有求和操作，尽管我们将输入数据限制在 long 的范围内，但是在求和时可能会超出精度范围。请你查阅 Java 相关资料，来看看在 Java 中是如何解决超过普通数据类型数据范围的精度问题的。
- Java 中有些特别的类用于处理大数运算，如 `BigInteger`，`BigDecimal`。
- 数据类型的边界可以使用类中的常量，例如`Long.MIN_VALUE`表示`long`类型（或`Long`类型）的最小值。
- 操作10要求输出特定的`Bottle`类型实例的属性，建议在 `Bottle` 类中**重写`toString` 方法**，示例如下：

```java
class Bottle {
    private int id;
    // ...
    @Override public String toString() {
        return "The bottle's id is " + id + ".";
    }
}

class Main {
    public static void main(String[] argv) {
        Bottle bottle = new Bottle();
        // ...
        System.out.println(bottle);
    }
}
/**
 * 输出:
 * The bottle's id is 1.
 */
```

`java.io.PrintStream.println(Object x)`函数将调用`String.valueOf(x)`以获得实例`x`的字符串描述，而`java.lang.String.valueOf(Object x)`函数将在`x`不为`null`时返回`x.toString()`，因此我们可以通过重写`toString`方法使`println`函数输出自定义格式的“实例属性”。

## 第二次作业

### 第一部分：训练目标

- 学习继承、了解设计模式中的工厂模式
- 学习方法的重写，Java的多态机制和Java的异常处理机制
- 学习Git工具中的远程仓库、打标签、重置的相关知识。

### 第二部分：预备知识

#### 一、继承

继承就是定义子类继承父类的特征和行为，使得子类可以拥有父类的属性和方法，从而起到代码复用的目的。

举个例子，假设我们有一个类 `Hero` 表示英雄，其包含生命值，保护盾值与魔法值这三个属性，并包含“徒手攻击”这一方法：

```java
public class Hero {
    int healthPoint;
    int defensePoint;
    int magicalPoint;

    public void attackWithHand() {
        /**/
    }
}
```

假设我们还想设计一个类，比如“骑士”类 `Knight` ，他是英雄的一种，其也拥有生命，保护盾和魔法值，也拥有“徒手攻击”方法，除此之外其还拥有“手枪攻击”这个方法。如果从头开始实现这个类的话，需要编写如下代码：

```java
public class Knight {
    int healthPoint;
    int defensePoint;
    int magicalPoint;

    public void attackWithHand() {
        /**/
    }

    public void attackWithPistol() {
        /**/
    }
}
```

注意到骑士相比较英雄只多了一个使用手枪攻击的方法，其他部分都一样，所以我们可以认为骑士是一种特殊的英雄。其实我们还可能要设计“牧师“、“游侠”等类，其也拥有英雄类拥有的那些属性和方法，除此之外它们各自可能还有一些其他方法，则我们也可以将两个类也认为是特殊的英雄。倘若我们仍然直接编写代码，则又要写很多重复代码。如果直接复制粘贴，还要修改类名以及构造方法等地方，且假如第一个版本的方法写错了，后面复制粘贴的都会出错，修改时要处处做修改，非常麻烦。

这个时候，继承就登场了。使用继承，我们可以让类 `A` 去得到类 `B` 已有的属性和方法，接下来类 `A` 就只需要专注于编写其特有部分的代码了。

使用继承来编写骑士类的例子如下：

```java
public class Knight extends Hero {
    // 公共的属性和方法不需要重复编写

    // 只需要编写Knight特有的手枪攻击方法
    public void attackWithPistol() {
        /**/
    }
}
```

在Java中，我们使用 `extends` 关键字表示继承，`A extends B` 意味着 `A` 继承了 `B` ，`A` 是 `B` 的子类， `A` 得到了 `B` 的属性和方法。

从语义上来说，在 `A` 和 `B` 类型满足 *is-a* 关系（A is a B），即`A` 类型是 `B` 类型的一种时，可以使用继承来在程序表述。在本例中可以说 Knigh is a Hero，因此我们使用继承关系。

#### 二、向上转型

在建立了继承关系之后，可以使用**父类型**去引用通过**子类型**创建的对象。这里涉及两个重要的概念，对象与对象引用。一般而言，**对象是一个类的实例化结果**，对应内存中的一个数据结构。对象引用则是使用一个变量来指向内存中的这个数据结构（即对象）。

如我们可以使用上面的 Knight 类来构造一个对象：`new Knight()`，这条语句返回一个创建的对象。我们同时需要声明一个对象引用来指向返回的对象，否则可能就找不到这个对象了。所以，一般代码都会这么写：`Knight knt = new Knight()` 。

在建立了继承关系之后，我们也可以使用 Hero 类来声明一个对象引用，并指向类型为 Knight 的对象：`Hero h = new Knight()`。从程序类型的角度，这个表达方式称为向上的类型转换，简称**向上转型** (up cast)。向上转型的例子如下：

```java
public class Main {
    public static void main(String[] args) {
        Hero hero1 = new Knight();
        hero1.attackWithHand();
    }
}
```

因为 Knight 类提供了 `attackWithPistol()` 方法，因此通过 `new Knight()` 创建的对象是拥有手枪攻击这个能力。这里同学们可能会马上想到：能否通过上面例子中的 hero1 来调用这个方法呢？ 如下面的代码所示：

```java
public class Main {
    public static void main(String[] args) {
        Hero hero1 = new Knight();
        // 编译错误
        hero1.attackWithPistol();
    }
}
```

很不幸，上面的代码会出现编译错误，编译器认为 Hero 类中没有定义 `attackWithPistol()` 方法。这就带来了一个问题，明明所指向的对象拥有相应的方法，但是却不能调用。其原因是我们进行了向上转型，使用 Hero 类型的变量来引用它，这往往表明程序设计者此时只关心在 Hero 类这个层次能够看到的方法（否则就应该使用 Knight 来声明一个引用）。

#### 三、向下转型

Java 语言提供了一个特殊的关键词 `instanceof` 用来判断一个对象引用所指向的对象的创建类型是否为特定的某个类，一般写为 `obj instanceof A`，其中 obj 为一个对象引用，A 为一个类型（类或接口），这个表达式的取值结果为布尔型，如果 obj 的创建类型为 A，则结果为 true，否则为 false。在这个表达式取值为 true 的情况下，可以使用**向下转型** (down cast) 来使用一个 A 类型的对象来引用obj： `A ao = (A)obj` 。注意，实际上 obj 所指向对象的创建类型永远不会发生变化，转型的只是对象引用类型。下面例子给出了相应的向下转型场景：

```java
public class Main {
    public static void main(String[] args) {
        A[] list = new A[20];
        Scanner input = new Scanner(System.in);
        int cnt = 0;
        // 先构造10个对象，放到数组list中
        for (int i = 0; i < 10; i++) {
            int t = input.nextInt();
            if (t % 3 == 0) {
                list[cnt] = new A();
            } else if (t % 3 == 1) {
                list[cnt] = new B();
            } else {
                list[cnt] = new C();
            }
            cnt++;
        }
        // 我们想调用list中所有C类型对象的c()方法
        for (int i = 0; i < cnt; i++) {
            // 先判断是不是C类型的对象，A instanceof B会返回true 或者 false
            if (list[i] instanceof C) {
                // 如果是，就向下转型，使用这个对象原本的类型的引用去指向它
                // 如果不是却还强行向下转型，则会出现错误
                C ref = (C) list[i];
                // 然后调用其c()方法
                ref.c();
            }
        }
    }
}
```

值得注意的是，在 `instanceof` 返回真的时候使用向下转型，才能保证向下转型的安全性，否则运行时会触发错误。

#### 四、对象方法的重写和复用

有时候，你会发现具有继承关系的类的某些行为具有递进关系，比如在下方代码中 Course类 和 OOCourse类 之间具有继承关系，OOCourse与Course有部分**相同**行为（即Course中定义且被OOCourse继承的行为），但OOCourse也会有自己的**特有**行为。

为了确保不论是使用Course对象引用，或者OOCourse对象引用来访问OOCourse对象时都能够顺利调用相应的方法，我们期望这两个类中实现的特定方法同名。**这种让子类重新实现一个在父类中已经实现的方法是面向对象的一个重要机制，称为方法重写。**方法重写获得的直接好处是让子类与父类**在相应方法的调用上保持了一致性。**

更通俗的说，重写方法与父类方法在行为上具有相似功能，但子类重写的方法**一般额外增加一些行为。**举例而言，设Course中实现了一个显示课程信息的方法(displayInfo)，我们希望OOCourse重新实现这个方法，从而能够多显示一些特有的信息。在程序编写方面，**一般会为重写方法标上一个@Override标签**。看下面的例子：

```java
class Course {
    void displayInfo() {
        System.out.println("老师上课，同学完成作业，最终老师会给一个成绩");
    }
}
class OOCourse extends Course {
    @Override
    void displayInfo() {
        System.out.println("老师上课，同学完成作业，最终老师会给一个成绩");
        System.out.println("还有研讨课，强测互测等任务，学期结束还会有颁奖典礼");
    }
}
```

我们可以看到OOCourse重写的`displayInfo`方法中的第一句话与Course中`displayInfo`方法的语句完全相同。通常，我们不希望出现重复编写代码（又称为代码拷贝）的现象。Java语言提供了一个重要的关键词**super**，它实际指代的是当前对象从父类继承得到的内容，因此通过super.displayInfo()可以确保调用的是Course实现的displayInfo方法。请看下面的例子：

```java
class OOCourseAlpha extends Course {
    @Override
    void displayInfo() {
        super.displayInfo(); // 调用了类Course中定义的方法
        System.out.println("还有研讨课，强测互测等任务，学期结束还会有颁奖典礼");
    }
}
```

#### 五、多态

前面提到，**如何判断实际调用的是子类重写的方法，还是父类实现的方法。**其实，这与对象**引用的类型**无关，而是取决于被引用对象的**创建类型**。请看下面的代码示例：

```java
Course c1 = new Course();
Course c2 = new OOCourseAlpha();
c1.displayInfo();
c2.displayInfo();
```

其中通过c1调用的实际是Course类实现的displayInfo方法，而通过c2调用的则是OOCourseAlpla类重写的displayInfo方法，但实际上c1和c2的**引用类型都是Course。** 上面我们提到的这个特性，就叫做多态。

#### 六、异常处理

程序运行时，发生了不被期望的事件，它阻止了程序按照程序员的预期正常执行，这就是异常。运行出错后，Java 提供了一种优秀的解决办法：异常处理机制。

异常处理机制采取显式的方式来处理异常，包括两个方面：

- 引入了专门的表达和处理方式，代码上一目了然就能看出是异常处理；
- 一旦发生异常，会强迫程序执行**进入异常处理分支**。

在Java语言中，**每个异常都被封装为Exception**，**异常有抛出和捕捉两种处理方式。**所谓抛出，就是使用Java提供的throw关键词来产生一个Exception或者其某个子类的对象；而捕捉则是通过catch关键词来捕捉在一段代码的执行过程中所抛出的相关异常对象。

课程推荐使用异常处理机制来区分处理显著不同于一般情况下的数据状态。使用异常处理可以让你的代码更加清晰易于理解，降低出现 bug 的可能性。请阅读《Core Java》的第七章相关内容了解异常处理机制的使用方法，并参照本章最后 Tips 部分来使用异常处理优化你的代码。

### 第三部分：题目描述

本次作业需要在上次作业的基础上进行增量开发。

- 建立 Equipment 装备类。我们将 Task1 中的 Bottle 以及下面增加的所有药水类、武器类统称为 *“装备类”*，使所有装备类均继承自 Equipment 类（该类因而可称为基类， base class），请将所有装备都具有的属性定义在这个类里。同时，Task1 中每位冒险者拥有承载多个 Bottle 的容器，这里将承载 Bottle 的容器改为承载所有装备类的容器。
- 为冒险者新增一些属性如下：生命值 （`health`, 浮点数，默认值 100.0）、经验值（ `exp`, 浮点数，默认 0.0）、金钱数（ `money`, 浮点数，默认 0.0）。
- 增加药水 HealingPotion 和 ExpBottle 并继承 Bottle 的全部属性；添加“武器类” Sword 以及 RareSword 和 EpicSword，他们继承 Sword 全部属性。见下表：

![image-20220925103821864](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925103821864.png)

![image-20220925103839673](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925103839673.png)

- 为每一种装备设置一个**使用**方法，定义如下，设冒险者A使用了装备B：

![image-20220925103906163](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925103906163.png)

- 实现各项装备的查询和增删指令，设置如下操作：
  1. 加入一个冒险者
  2. 给某个冒险者添加某件装备（装备包括药水和武器）
  3. 删除某个冒险者拥有的某个装备
  4. 查询某个冒险者所拥有装备的价格之和
  5. 查询某个冒险者所拥有装备的价格最大值
  6. 查询某个冒险者拥有的装备总数
  7. 打印一个装备的全部属性，属性的输出顺序与输入创建该装备时给定的各参数顺序一致，具体格式详见下方 *属性打印方式*
  8. 某个冒险者使用其拥有的某个装备
  9. 打印某个冒险者的所有状态

### 第四部分：输入输出

第一行一个整数 m*m*，表示操作的个数。

接下来的 m*m* 行，每行一个形如 `{type} {attribute}` 的操作，`{type}` 和 `{attribute}` 间、若干个 `{attribute}` 间使用若干个空格分割，操作输入形式及其含义如下：

![image-20220925103944104](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925103944104.png)

![image-20220925103958143](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925103958143.png)

![image-20220925104012593](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925104012593.png)

#### 数据范围与操作限制

##### 变量约束

![image-20220925104031878](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925104031878.png)

##### 操作约束

- 操作数满足 1 \leq m \leq 20001≤*m*≤2000。
- **保证所有冒险者与装备的 ID 两两不同。**
- 操作2-9：冒险者 ID 一定存在。
- 操作 3,7,8：冒险者一定持有该 ID 的装备。
- 操作 4：若冒险者不持有任何装备，则输出 0。
- 操作 5：冒险者一定持有至少一个装备。

#### 测评方法

输出数值时，你的输出数值需要和正确数值相等。

**假设你的输出值 x_{out}\*x\**o\**u\**t\* 和正确数值 x_{std}\*x\**s\**t\**d\* 之间的绝对或相对误差小于等于 10 ^ {-5}10−5，则认为是相等的，即满足**
$$
\frac{|x_{std}-x_{out}|}{max(1,|x_{std}|)}\leq10^{-5}
$$

#### 输入样例

> ```
> 17
> 1 2 Co20ocvblT
> 1 30 Al8QnWnkS7
> 1 91 pqWY5UNcm4
> 2 91 1 26 6DlfOJGzfY 74 96.3964
> 2 2 6 35 yv58Ec49pK 2 65.161 68.6988
> 2 2 1 71 FEw7siBqbW 64 66.534
> 2 91 2 44 OLy4CqtmrO 45 60.135 13.2503
> 2 30 1 56 H2EvYaqUXD 0 64.7676
> 2 91 6 65 Wjsn3jVy6E 60 20.1061 23.1743
> 2 2 2 28 0WnMAYPzUH 37 27.0554 10.4833
> 3 30 56
> 4 30
> 5 91
> 6 91
> 7 91 65
> 8 2 35
> 9 91
> ```

#### 输出样例

> ```
> 0
> 74
> 3
> The epicSword's id is 65, name is Wjsn3jVy6E, sharpness is 20.1061, evolveRatio is 23.1743.
> Co20ocvblT used yv58Ec49pK and earned 65.161.
> yv58Ec49pK's sharpness became 4476.4825068.
> The adventurer's id is 91, name is pqWY5UNcm4, health is 100.0, exp is 0.0, money is 0.0.
> ```

### 第五部分：补充

- “装备类”包括 Bottle, HealingPotion 等 6 个不同的类，而冒险者需要拥有一个可以承载这 6 个装备类的容器。为了避免为 6 个装备类分别维护容器的麻烦，我们可以使用“向上转型”，在 Adventurer 类中统一维护一个承载 Equipment 的容器，并让 6 个装备类全部继承自 Equipment 类。由于“多态”的特性，在向上转型后对象仍然不会失去其原先的装备性质。
- 建议在Equipment类中定义一个`used`方法，该方法的代码用于描述这个装备被使用时会发生的效果，在所有的装备子类中都去重写这个`used`方法，另外还应该为所有需要打印描述字符串的类重写`toString`方法。在`Adventurer`类中定义`HashMap<Integer, Equipment>`类型的`equipments`属性（也可使用其他容器），表示冒险者拥有的全部装备，在执行操作8时，直接调用该装备对象的`used`方法（因为有多态机制，这里不需要强制转型，直接调用就可以保证行为正确）。
- 冒险者使用装备的过程中，是对冒险者属性和装备自身属性的读取，运算和修改。如何才能让装备类的方法可以读取并修改他的使用者的属性呢？为`used`方法传递一个冒险者作为参数是一个好主意。既然加了冒险者作为参数，那不妨把方法名从`used`改为`usedBy`，这会让你的代码看起来就像是英文句子一样，写出self-documenting code（自我解释型代码）。
- 在 `Bottle` 和它的子类在 `filled` 为 `false` 时被使用就可以看作是一种异常行为。于是你可以在 `Bottle.usedBy` 方法中抛出一个异常（使用 throw 语句），在 `HealingPotion.usedBy` 调用 `super.usedBy` 时，不处理这个异常而是将其继续抛出到上层，而在冒险者循环使用装备的代码中将其捕获并打印出错误信息。以下代码是 Bottle 类中推荐的 usedBy 实现方法：

```java
@Override
public void usedBy(Adventurer user) throws Exception { // 因为有一个 Adventurer 参数，所以方法名写作 "usedBy" 会比 "used" 更加易于理解。
    if (!isFilled()) {
        throw new Exception("Failed to use " + getName() + " because it is empty.");
    }
    user.setHealth(user.getHealth() + capacity / 10);
    setFilled(false);
    setPrice(getPrice().divide(BigInteger.TEN)); 

    System.out.println(user.getName() +
            " drank " + getName() +
            " and recovered " + capacity / 10 +
            ".");
}
```

以下代码是 Adventurer 类中用于完成操作 8 所推荐的 use 实现方法。

```java
public void use(Equipment equipment) {
    try {
        equipment.usedBy(this);
    } catch (Exception e) {
        System.out.println(e.getMessage());
    }
}
```

- **设计模式**是软件开发人员经过相当长的实践总结出来的最佳设计方案，在面向对象设计与构造课程中，你将逐步了解和掌握几种基本的设计模式，包括工厂模式、单例模式、生产者-消费者模式等。

  现在，希望大家可以了解**工厂模式**，这是在继承和接口实现中常用的设计模式。

  大家可以参考[链接](https://d.buaa.edu.cn/https/77726476706e69737468656265737421e7e056d23525665f710ac7af9758/design-pattern/factory-pattern.html)中的介绍，也可以自行查阅资料。这将帮助你更轻松的完成日后的作业 :)

- 本次作业的“测验”部分仍然是与 Git 相关的题目，均可在 [Git Pro](https://www.progit.cn/) 中找到答案。测验题回答正误均没有影响，不计入成绩，我们将在提交正确或提交机会用尽时给出题目的详细解析，希望同学们阅读后能有所收获。

## 第三次作业：

### 第一部分：训练目标

- 学习接口相关知识，并实践如何使用接口来建立行为层次结构。
- 学会使用 Java 类库提供的类进行排序。
- 掌握容器的克隆方法，理解浅克隆 (Shallow copy) 和 深克隆 (Deep copy)
- 初步了解 git 分支的用法

### 第二部分：预备知识

#### 一、接口

前面我们提到了子类可以重写父类的方法，这使得子类的方法可以在父类的方法的基础上增加功能，或者实现一套和父类不同的新的功能。

倘若父类的**抽象程度**很高，以至于在父类中没有办法去编写一个实现具体功能的方法，我们可能会想是不是可以不写方法的具体实现语句，只定义方法签名呢？

比方说，正方形和圆形的面积计算很具体，假设为正方形和圆形建立了一个共同的抽象父类二维图形，此时如何去实现一个二维图形的面积呢？

比如下面的例子：

```java
class Square {
    private double length;
    public double getArea() { //你可以为一个正方形编写求面积方法
        return length * length;
    }
}

class Circle {
    private double radius;
    public double getArea() { //你可以为一个圆形编写求面积方法
        return radius * radius * Math.PI;
    }
}
```

很显然，我们无法为抽象的二维图形Shape类实现面积求解方法。此时，我们可以使用接口(Interface)来表示这个抽象的类，然后声明上述两个具体的类实现(implements)了这个接口：

```java
interface Shape {
    public double getArea(); // 你不能为抽象的`形状`概念定义求面积方法
}

class Square implements Shape {
    private double length;
    public Square(double length) {
        this.length = length;
    }
    @Override
    public double getArea() { //你可以为一个正方形编写求面积方法
        return length * length;
    }
}

class Circle implements Shape {
    private double radius;
    public Circle(double radius) {
        this.radius = radius;
    }
    @Override
    public double getArea() { //你可以为一个圆形编写求面积方法
        return radius * radius * Math.PI;
    }
}
```

之后，你可以用接口类型来引用实现了该接口的任意类型的实例化对象，并调用接口所声明的方法。需要注意的是，你不能用接口类型来**实例化**一个对象：

```java
class Main {
    public static void main(String[] args) {
        Shape myShape; // 声明一个Shape的变量， 这是还没有任何实例产生
        myShape = new Square(888); // 创建一个Square的实例，用myShape变量引用它。
        System.out.println(myShape.getArea());
        myShape = new Circle(888); // 创建一个Circle的实例，用myShape变量引用它。
        System.out.println(myShape.getArea());
        myShape = new Shape(); // Shape的概念过于抽象以至于实例化没有意义，这一行编译报错。

    }
}
```

需要注意的是，接口提供了行为的抽象机制。在上面的例子中，Square和Circle的**共性在于其行为操作**，因而使用接口是合适的。对于其他一些情况，**多个类之间可能即有共性的行为，也有共性的数据属性，此时使用类建立抽象层次更加合适。**

**在编程时，尽量使用高层次的引用（比如抽象类的引用和接口的引用），避免使用实际子类型的引用的方式，叫做面向抽象编程。**下面我们会通过本 Task 让大家体会这一点。

#### 二、浅克隆与深克隆

前面已经提到，在 Java 中，我们使用引用 (reference) 来操作一个对象。这表明，当我们在程序中写出形如 `Bottle bottle` 时，我们所声明的 `bottle` 变量只是一个引用，他可能会引用所有类型正确的实例。因此，如果我们需要对一个实例进行复制操作，**就需要仔细考虑复制的是引用还是实例。**请看下面的程序片段：

```java
class Main {
    public static void main(String[] args) {
        Shape shape1 = new Square(888);
        Shape shape2;
        shape2 = shape1; // 试图复制一个实例
        shape1.setArea(1); // 更改 shape1 引用的实例
        System.out.println(shape2.getArea()); // 输出 shape2 引用的实例
    }
}
/**
 * 输出：
 * 1
 **/
```

我们可以发现，上面的程序只是对引用进行了克隆。上面的程序首先创建了一个 Square 实例，并使用 shape1 引用它。之后声明变量 shape2，并让 shape2 引用了 shape1 所引用的实例。我们只创建了一个实例，shape1 和 shape2 均为同一个实例的引用。因此，通过 shape1 引用对实例进行的修改，也会在使用 shape2 访问该示例时体现。

上面的这种只克隆引用的克隆过程，称为 *浅克隆* (Shallow copy)。如果希望创造出一个“完整”的克隆，我们不仅要在编码时创建一个新的引用，还要创建一个新的实例：

```java
class Main {
    public static void main(String[] args) {
        Shape shape1 = new Square(888);
        Shape shape2 = new Square();
        shape2.setArea(shape1.getArea());
        System.out.println("shape1: " + shape1.getArea() +
            ", shape2: " + shape2.getArea());
        shape2.setArea(1);
        System.out.println("shape1: " + shape1.getArea() +
            ", shape2: " + shape2.getArea());
    }
}
/**
 * 输出：
 * shape1: 888, shape2: 888
 * shape1: 888, shape2: 1
 **/
```

这种克隆引用和实例的克隆过程，称为 *深克隆* (Deep copy)。

#### 三、容器中的克隆

我们已经了解到，Java 使用引用来操作实例，这导致克隆时既可以克隆引用，也可以克隆实例，即深克隆和浅克隆。在之前的两次作业中，同学们已经学会了容器的基本使用方法。容器提供了管理多个对象的方法，字如其名，容器中”容纳了若干个对象”。**在拷贝容器时，深拷贝和浅拷贝的区别将会加大。**

现在请大家思考，一个对象是否可以位于多个容器中呢？答案是肯定的，这是因为 Java 中只能使用引用来操作实例，容器也不例外；每个容器维护的是若干个实例的 **引用**。因此，如果我们希望将一个容器进行拷贝，我们有两种方法：

1. 使用浅拷贝，拷贝出的另一个容器管理的 *引用* 与原容器相同
2. 使用深拷贝，先拷贝出该容器中管理的所有实例，再依次添加至新容器中

假设现在有一个名为 `advs` 的 `ArrayList` 容器，该容器管理了若干个 `Adventurer` 类型对象的引用。由于 Java 中所有类型都继承于 `Object` 类，该类拥有 `clone` 方法，因此我们使用 `advs.clone()` 对该容器进行克隆：

```java
advs.get(0).setName("Old Name");
ArrayList<Adventurer> advsClone = advs.clone();
advsClone.get(0).setName("New Name");
System.out.println(advs.get(0).getName());
/**
 * 输出：
 * New Name
 **/
```

可以发现，这是一个浅克隆，只克隆了容器中的引用，而没有克隆 `Adventurer` 对象。在 [ArrayList.clone()](https://docs.oracle.com/en/java/javase/18/docs/api/java.base/java/util/ArrayList.html#clone()) 文档中已明确说明，该方法返回一个浅克隆的新容器：

> `public Object clone()` Returns a shallow copy of this ArrayList instance. (The elements themselves are not copied.)

如果希望对容器每个对象本身都进行克隆，则需要遍历该容器，克隆其中的每个对象，并添加至新容器中。

在对容器进行克隆操作时，需要特别注意是否需要进行深克隆。

### 第三部分：基本要求

本次作业是本单元最后一次作业，仍需在上一次作业的基础上进行增量开发。

在本任务中，我们允许**冒险者雇佣并使用另一个冒险者**，且**赋予冒险者价值的概念**，把装备和冒险者都看作是**价值体 `commodity`**。同时，我们还要对冒险者游戏增加**版本管理功能**，与 git 版本管理工具进行类比，可将冒险者游戏的状态视为需要管理的数据，每执行一条指令视为进行一次 commit，并实现简单的新建分支、检出分支功能。

### 第四部分：题目描述

- 增加 Commodity 接口，并使冒险者 Adventurer 类和装备 Equipment 类实现 Commodity 接口。接口中应定义冒险者和装备的共有方法，包括 `useBy` 方法等。
- 将原先的冒险者持有的 *装备* 的容器，更改为 *价值体* 的容器（即该容器可以容纳所有实现了 Commodity 接口的类）。
- 定义冒险者的**价值**为其拥有的所有价值体的价值之和，即冒险者的价值是其**装备**的价值及其**雇佣的冒险者**的价值的和。
- 增加冒险者之间的**雇佣关系**：冒险者 A 雇佣冒险者 B，可以认为是把冒险者 B 看成一种**价值体**。此时冒险者 A 拥有价值体冒险者 B，之后冒险者 A 便可以像使用其他装备一样**使用**冒险者 B。
- 定义**冒险者 A 使用冒险者 B**，其效果为冒险者 A **按照价值从大到小、价值相同则按价值体 `id` 从大到小的顺序** 依次使用**冒险者 B 的价值体**，价值体的价值指的是所有价值体在**本次使用前**的价值。我们规定：如果当前使用到了冒险者 B 雇佣的冒险者 C，则冒险者 C 要按照如上顺序使用其拥有的价值体，这些价值体将作用于最开始使用的冒险者，在此处即为冒险者 A。
- 新增**版本管理**功能：我们仿照 git 中的分支机制进行版本管理。将每一条执行的指令视为一次 commit，**初始状态下默认分支名称为`1`**，需要支持“创建分支并检出该分支”功能，以及“检出”功能。与 git 相同，每次 commit 都将移动当前 HEAD 指针所指向的分支指针，也就是说，假设当前处于 `br` 分支，执行了若干条指令（相当于在 `br` 分支上进行了若干条 commit）后，**`br` 分支也会发生更改**。

### 第五部分：输入/输出格式

第一行一个整数 m*m*，表示操作的个数。

接下来的 m*m* 行，每行一个形如 `{type} {attribute}` 的操作，`{type}` 和 `{attribute}` 间、若干个 `{attribute}` 间使用若干个空格分割，操作输入形式及其含义如下：

- 对 Task2 中的一些指令进行**少量修改**，**重点地方已经加粗**，并新增三条指令 10 ～ 12：

![image-20220925105638569](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925105638569.png)

`vars` 和 `equipment_type` 如下：

![image-20220925105708730](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925105708730.png)

属性打印方式表格：

![image-20220925105724634](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925105724634.png)

#### 一、数据范围与操作限制

##### 变量约束

![image-20220925105740137](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20220925105740137.png)

##### 操作约束

- 操作数满足 1 \leq m \leq 20001≤*m*≤2000。
- **保证所有价值体的 ID 两两不同。**
- 操作2-9：冒险者 ID 一定存在。
- 操作 3,7：冒险者一定持有该 ID 的价值体。
- 操作 4,6：冒险者不持有任何价值体，则输出 0。
- 操作 5：冒险者一定持有至少一个价值体。
- 操作 10：雇佣和被雇佣的冒险者均已存在，且不是同一个冒险者。
- 指令 11：新建的 branch_name 不与已有的 branch_name 重名。
- 指令 12：检出的 branch_name 之前一定被新建过。
- 冒险者的雇佣关系不会存在循环雇佣的情况，每个冒险者最多仅能被一个其他冒险者雇佣一次。
- 初始状态下位于 branch_name 为 `1` 的分支。

#### 二、测评方法

输出数值时，你的输出数值需要和正确数值相等。

**假设你的输出值 x_{out}\*x\**o\**u\**t\* 和正确数值 x_{std}\*x\**s\**t\**d\* 之间的绝对或相对误差小于等于 10 ^ {-5}10−5，则认为是相等的，即满足**
$$
\frac{|x_{std}-x_{out}|}{max(1,|x_{std}|)}\leq10^{-5}
$$

#### 三、输入输出示例

##### 样例1

输入：

> ```
> 19
> 1 2 Co20ocvblT
> 1 30 Al8QnWnkS7
> 1 91 pqWY5UNcm4
> 2 91 1 26 6DlfOJGzfY 74 96.3964
> 2 2 6 35 yv58Ec49pK 2 65.161 68.6988
> 2 2 1 71 FEw7siBqbW 64 66.534
> 2 91 2 44 OLy4CqtmrO 45 60.135 13.2503
> 2 30 1 56 H2EvYaqUXD 0 64.7676
> 2 91 6 65 Wjsn3jVy6E 60 20.1061 23.1743
> 2 2 2 28 0WnMAYPzUH 37 27.0554 10.4833
> 10 30 91
> 10 91 2
> 3 30 56
> 4 30
> 5 91
> 6 2
> 7 2 28
> 8 2
> 9 2    
> ```

输出：

```
282
103
3
The healingPotion's id is 28, name is 0WnMAYPzUH, capacity is 27.0554, filled is true, efficiency is 10.4833.
Co20ocvblT drank FEw7siBqbW and recovered 6.6534.
Co20ocvblT drank 0WnMAYPzUH and recovered 2.70554.
Co20ocvblT recovered extra 283.62987482.
Co20ocvblT used yv58Ec49pK and earned 65.161.
yv58Ec49pK's sharpness became 4476.4825068.
The adventurer's id is 2, name is Co20ocvblT, health is 382.98881482, exp is 10.0, money is 65.161.
```

##### 样例2

输入：

> ```
> 30
> 1 2 Co20ocvblT
> 1 30 Al8QnWnkS7
> 1 91 pqWY5UNcm4
> 2 91 1 26 6DlfOJGzfY 74 96.3964
> 2 2 6 35 yv58Ec49pK 2 65.161 68.6988
> 2 2 1 71 FEw7siBqbW 64 66.534
> 2 91 2 44 OLy4CqtmrO 45 60.135 13.2503
> 2 30 1 56 H2EvYaqUXD 0 64.7676
> 2 91 6 65 Wjsn3jVy6E 60 20.1061 23.1743
> 2 2 2 28 0WnMAYPzUH 37 27.0554 10.4833
> 11 2
> 10 30 91
> 10 91 2
> 3 30 56
> 4 30
> 5 91
> 6 2
> 7 2 28
> 8 2
> 9 2    
> 12 1
> 10 30 91
> 10 91 2
> 3 30 56
> 4 30
> 5 91
> 6 2
> 7 2 28
> 8 2
> 9 2
> ```

输出：

> ```
> 282
> 103
> 3
> The healingPotion's id is 28, name is 0WnMAYPzUH, capacity is 27.0554, filled is true, efficiency is 10.4833.
> Co20ocvblT drank FEw7siBqbW and recovered 6.6534.
> Co20ocvblT drank 0WnMAYPzUH and recovered 2.70554.
> Co20ocvblT recovered extra 283.62987482.
> Co20ocvblT used yv58Ec49pK and earned 65.161.
> yv58Ec49pK's sharpness became 4476.4825068.
> The adventurer's id is 2, name is Co20ocvblT, health is 382.98881482, exp is 10.0, money is 65.161.
> 282
> 103
> 3
> The healingPotion's id is 28, name is 0WnMAYPzUH, capacity is 27.0554, filled is true, efficiency is 10.4833.
> Co20ocvblT drank FEw7siBqbW and recovered 6.6534.
> Co20ocvblT drank 0WnMAYPzUH and recovered 2.70554.
> Co20ocvblT recovered extra 283.62987482.
> Co20ocvblT used yv58Ec49pK and earned 65.161.
> yv58Ec49pK's sharpness became 4476.4825068.
> The adventurer's id is 2, name is Co20ocvblT, health is 382.98881482, exp is 10.0, money is 65.161.
> ```

##### 样例3

输入：

> ```
> 23
> 1 2 Co20ocvblT
> 1 30 Al8QnWnkS7
> 12 1
> 1 91 pqWY5UNcm4
> 2 91 1 26 6DlfOJGzfY 74 96.3964
> 2 2 6 35 yv58Ec49pK 2 65.161 68.6988
> 2 2 1 71 FEw7siBqbW 64 66.534
> 2 91 2 44 OLy4CqtmrO 45 60.135 13.2503
> 12 1
> 2 30 1 56 H2EvYaqUXD 0 64.7676
> 2 91 6 65 Wjsn3jVy6E 60 20.1061 23.1743
> 2 2 2 28 0WnMAYPzUH 37 27.0554 10.4833
> 10 30 91
> 10 91 2
> 3 30 56
> 4 30
> 12 1
> 5 91
> 6 2
> 7 2 28
> 12 1
> 8 2
> 9 2
> ```

输出：

> ```
> 282
> 103
> 3
> The healingPotion's id is 28, name is 0WnMAYPzUH, capacity is 27.0554, filled is true, efficiency is 10.4833.
> Co20ocvblT drank FEw7siBqbW and recovered 6.6534.
> Co20ocvblT drank 0WnMAYPzUH and recovered 2.70554.
> Co20ocvblT recovered extra 283.62987482.
> Co20ocvblT used yv58Ec49pK and earned 65.161.
> yv58Ec49pK's sharpness became 4476.4825068.
> The adventurer's id is 2, name is Co20ocvblT, health is 382.98881482, exp is 10.0, money is 65.161.
> ```

##### 样例4

输入：

> ```
> 30
> 1 329740070 sEhDbhnEnd
> 1 1851576059 8N3Vj7BkdZ
> 1 1151146527 T3NZDh4jCz
> 2 1151146527 1 1604225601 1u4QtP6lL9 5038843478073893142 28.0905
> 2 1151146527 4 419039688 Yuu2onZU2y 2877398768860155635 93.917
> 2 1151146527 4 1151636326 Fv1RvYmP0E 7285089275503127969 95.7149
> 2 329740070 2 1154770639 eRNkZX7yE8 8269076524323616536 13.2538 24.7047
> 2 329740070 4 1710411361 N8Nav2fayl 2308642044102240255 72.4425
> 2 329740070 2 112570669 hFC53lbVRK 4601284869343090065 47.6571 43.1437
> 11 2
> 10 1851576059 1151146527
> 5 329740070
> 7 329740070 1154770639
> 6 1851576059
> 12 2
> 10 1151146527 329740070
> 8 329740070
> 2 329740070 2 698684406 xD5l7UCB4Y 3742687023378757769 79.1502 74.4603
> 6 329740070
> 11 3
> 9 329740070
> 2 1851576059 4 870218830 mgyVlRqvxp 2506134180893997996 67.5698
> 2 329740070 3 1851077531 fHDuxXvDW6 773490031336115588 94.9004 58.1943
> 6 329740070
> 12 2
> 1 909784950 6REvh6RB7v
> 2 329740070 4 264476603 ibOoPOPubU 278140469188458658 33.6055
> 8 1851576059
> 2 329740070 2 192707537 V8mhiYQmm6 5462659141869010887 80.353 52.5216
> 11 4
> ```

输出：

> ```
> 8269076524323616536
> The healingPotion's id is 1154770639, name is eRNkZX7yE8, capacity is 13.2538, filled is true, efficiency is 24.7047.
> 1
> sEhDbhnEnd drank eRNkZX7yE8 and recovered 1.32538.
> sEhDbhnEnd recovered extra 327.43115286.
> sEhDbhnEnd drank hFC53lbVRK and recovered 4.76571.
> sEhDbhnEnd recovered extra 2056.10362527.
> sEhDbhnEnd used N8Nav2fayl and earned 72.4425.
> 4
> The adventurer's id is 329740070, name is sEhDbhnEnd, health is 2479.62586813, exp is 10.0, money is 72.4425.
> 5
> 8N3Vj7BkdZ drank xD5l7UCB4Y and recovered 7.91502.
> 8N3Vj7BkdZ recovered extra 5893.54763706.
> 8N3Vj7BkdZ used N8Nav2fayl and earned 72.4425.
> Failed to use eRNkZX7yE8 because it is empty.
> Failed to use hFC53lbVRK because it is empty.
> 8N3Vj7BkdZ used ibOoPOPubU and earned 33.6055.
> 8N3Vj7BkdZ used Fv1RvYmP0E and earned 95.7149.
> 8N3Vj7BkdZ drank 1u4QtP6lL9 and recovered 2.80905.
> 8N3Vj7BkdZ used Yuu2onZU2y and earned 93.917.
> ```

### 第六部分：提示

- 冒险者和装备都是价值体，都可以**求价值**、**被使用**以及**字符串化**等，故一个推荐的设计方法是建立**价值体接口** ，接口中包含上述提到的三个方法，让冒险者 `Adventurer` 和装备 `Equipment` 都实现这个接口，这样在顶层逻辑中就只能看到**价值体**这一种类型，可使用该类型的引用去调用不同子类型对象的这三种方法，这种处理称为归一化处理，会在正式课程中有专门的论述和训练。
- 本次作业将会涉及到自定义排序，请学习如何给对象编写 `compareTo` 方法并实现 `Comparable` 接口，之后即可利用 `Collections.sort` 方法来给容器内对象进行排序，考虑到有许多知识同学们还没有学过，本章结尾会给出一个例子，同学们可以照猫画虎地完成，compareTo方法仅需要在Equipment类中定义，Equipment类的子类如果不重写该方法的话，将会与父类行为保持一致。

> 与 `Collections.sort` 会调用 `compareTo` 方法实现自定义排序，类似地，`TreeSet` 和 `TreeMap` 容器也会通过调用对象的 `compareTo` 方法，从而维护一个key对象有序的集合/映射。
>
> 另外，`HashSet` 和 `HashMap` 这两种容器会通过调用对象的 `hashCode` 方法和 `equals` 方法来将任意对象作为key来使用。**这个知识点非常重要，不过因为原理上与 compareTo 相似度较高便不在此处过多训练，请同学们务必弄懂其原理**。
>
> Java中许多内置的类，比如 `Integer` 和 `BigInteger` 等等都已经实现了`compareTo`、`hashCode`、`equals` 方法，所以你才可以直接把他们当作 `TreeMap` 和 `HashMap` 的key来使用。

```java
// Comparable接口的例子

import java.util.ArrayList;
import java.util.Collections;

class MainClass {
    public static void main(String[] args) {
        Score xiaoMing = new Score(120, 138);
        Score xiaoHong = new Score(125, 133);
        Score xiaoWang = new Score(119, 145);
        ArrayList<Score> scores = new ArrayList<>();
        scores.add(xiaoMing);
        scores.add(xiaoHong);
        scores.add(xiaoWang);

        Collections.sort(scores);

        for (Score score : scores) { // 如果你使用IDEA编写代码，可以试一试打出 "scores.for<TAB>" 这一串按键，快速补齐for循环
            System.out.println(score); // 试一试 "score.sout<TAB>"，自动补齐打印语句
        }
        /*
        运行结果如下，越大的对象越在后面（即升序排序）:
        Score{chinese=120, math=138}
        Score{chinese=125, math=133}
        Score{chinese=119, math=145}
         */
    }
}

class Score implements Comparable<Score> { // 后面尖括号里的类名基本上都会与前面的类名相同，表达“Score这个类可以与Score这个类相比较”这个意思。
    private final int chinese;
    private final int math;

    public Score(int chinese, int math) {
        this.chinese = chinese;
        this.math = math;
    }

    public int getSum() {
        return chinese + math;
    }

    /**
     * 自定义分数的比较规则，首先比较总分，总分相同比较语文，语文相同比较数学……
     */
    @Override
    public int compareTo(Score other) {
        if (this.getSum() < other.getSum()) { //首先比较总分，总分高的先录取
            return -1; // 返回 -1 代表 this 小于 other
        } else if (this.getSum() > other.getSum()) {
            return 1; // 返回 1 代表 this 大于 other
        }

        if (this.chinese < other.chinese) { //若总分一样，按语文分更高的先录取
            return -1;
        } else if (this.chinese > other.chinese) {
            return 1;
        }

        // 返回任何负值都代表this < other，于是可以这样子简写，
        // 下面三行关于math的比较和上面的五行关于chinese的比较是完全等价的。
        if (this.math != other.math) {
            return this.math - other.math; 
        }

        return 0; //返回0代表两被比较对象相等
    }

    @Override
    public String toString() {
        return "Score{" +
                "chinese=" + chinese +
                ", math=" + math +
                '}';
    }

}
```
