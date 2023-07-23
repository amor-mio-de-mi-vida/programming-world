# **Java基础教程**

## java基础语法

### Java 枚举

Java 5.0引入了枚举，枚举限制变量只能是预先设定好的值。使用枚举可以减少代码中的 bug。

例如，我们为果汁店设计一个程序，它将限制果汁为小杯、中杯、大杯。这就意味着它不允许顾客点除了这三种尺寸外的果汁。

**实例**

```java
class FreshJuice {   
	enum FreshJuiceSize{ SMALL, MEDIUM , LARGE }   		FreshJuiceSize size; 
}  
public class FreshJuiceTest {
	public static void main(String[] args){      		FreshJuice juice = new FreshJuice();
    juice.size = FreshJuice.FreshJuiceSize.MEDIUM;  
    } 
}
```

**注意：**枚举可以单独声明或者声明在类里面。方法、变量、构造函数也可以在枚举中定义。

![image-20221019202038708](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221019202038708.png)

## java基本数据类型

注意：

**char：**

- char 类型是一个单一的 16 位 Unicode 字符；
- 最小值是 **\u0000**（十进制等效值为 0）；
- 最大值是 **\uffff**（即为 65535）；
- char 数据类型可以储存任何字符；
- 例子：char letter = 'A';。

### 引用类型

- 在Java中，引用类型的变量非常类似于C/C++的指针。引用类型指向一个对象，指向对象的变量是引用变量。这些变量在声明时被指定为一个特定的类型，比如 Employee、Puppy 等。变量一旦声明后，类型就不能被改变了。
- 对象、数组都是引用数据类型。
- 所有引用类型的默认值都是null。
- 一个引用变量可以用来引用任何与之兼容的类型。
- 例子：Site site = new Site("Runoob")。

在 Java 中使用 final 关键字来修饰常量，声明方式和变量类似：

```java
final double PI = 3.1415927;
```

虽然常量名也可以用小写，但为了便于识别，通常使用大写字母表示常量。

![image-20221019202922953](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221019202922953.png)

## java变量类型

![image-20221019203239931](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221019203239931.png)

### 实例变量

- 实例变量的值应该至少被一个方法、构造方法或者语句块引用，使得外部能够通过这些方式获取实例变量信息；
- 实例变量可以声明在使用前或者使用后；
- 实例变量可以直接通过变量名访问。但在静态方法以及其他类中，就应该使用完全限定名：ObjectReference.VariableName。

### 类变量（静态变量）

- 类变量也称为静态变量，在类中以 static 关键字声明，但必须在方法之外。
- 无论一个类创建了多少个对象，类只拥有类变量的一份拷贝。
- 静态变量除了被声明为常量外很少使用，静态变量是指声明为 public/private，final 和 static 类型的变量。**静态变量初始化后不可改变。**
- 静态变量储存在静态存储区。经常被声明为常量，很少单独使用 static 声明变量。
- 静态变量在第一次被访问时创建，在程序结束时销毁。
- 与实例变量具有相似的可见性。但为了对类的使用者可见，大多数静态变量声明为 public 类型。
- 默认值和实例变量相似。数值型变量默认值是 0，布尔型默认值是 false，引用类型默认值是 null。变量的值可以在声明的时候指定，也可以在构造方法中指定。此外，静态变量还可以在静态语句块中初始化。
- 静态变量可以通过：*ClassName.VariableName*的方式访问。
- 类变量被声明为 public static final 类型时，类变量名称一般建议使用大写字母。如果静态变量不是 public 和 final 类型，其命名方式与实例变量以及局部变量的命名方式一致。

> ## Java 中静态变量和实例变量区别
>
> -  静态变量属于类，该类不生产对象，通过类名就可以调用静态变量。
> -  实例变量属于该类的对象，必须产生该类对象，才能调用实例变量。
>
> **在程序运行时的区别：**
>
> - 实例变量属于某个对象的属性，必须创建了实例对象，其中的实例变量才会被分配空间，才能使用这个实例变量。
> -  静态变量不属于某个实例对象，而是属于类，所以也称为类变量，只要程序加载了类的字节码，不用创建任何实例对象，静态变量就会被分配空间，静态变量就可以被使用了。
>
> **总之，实例变量必须创建对象后才可以通过这个对象来使用，静态变量则可以直接使用类名来引用。**
>
> 例如，对于下面的程序，无论创建多少个实例对象， 永远都只分配了一个 staticInt 变量，并且每创建一个实例对象，这个 staticInt 就会加 1；但是，每创建一个实例对象，就会分配一个 random， 即可能分配多个 random ，并且每个 random 的值都只自加了1次。
>
> ```java
> public class StaticTest {
>     private static int staticInt = 2;
>     private int random = 2;
> 
>     public StaticTest() {
>         staticInt++;
>         random++;
>         System.out.println("staticInt = "+staticInt+"  random = "+random);
>     }
> 
>     public static void main(String[] args) {
>         StaticTest test = new StaticTest();
>         StaticTest test2 = new StaticTest();
>     }
> }
> ```
>
> 执行以上程序，输出结果为：
>
> ```
> staticInt = 3  random = 3
> staticInt = 4  random = 3
> ```

![image-20221020144558500](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221020144558500.png)

**类变量赋值方法**

- 无final修饰，声明时赋值，构造器中赋值，静态语句块或静态方法赋值
- 有final修饰，声明时赋值，声明与赋值分开可在静态语句块中赋值

解释：**private static final double PI = 3.14;**

子类不可用（与类继承关系） + 非静态方法不可用（与方法的关系）+ 常量（与变量的关系） + 类型（与其他类型的关系）

> **是否可以从一个static方法内部无法对非static方法的调用?**
>
> ```
> public class Xix {
>     // 静态成员 
>     public static String string="static成员";
>     // 普通成员
>     public String string2="非static成员";
>     // 静态方法
>     public static void method(){
>         string="sss";
>         //string2="sss";编译报错,因为静态方法里面只能调用静态方法或静态成员
>         //method2();
>         System.out.println("这是static方法,static方法与对象无关");
>     }
> 
>     // 普通方法 
>     public void method2(){
>         string ="string1";
>         string2="string2";
>         method(); //非静态方法里面可以发出对static方法的调用
>         System.out.println("这是非static方法,此方法必须和指定的对象关联起来才起作用");
>     }
>     public static void main(String[] args) {
>         Xix x=new Xix();
>         x.method2();// 引用调用普通方法 
>         x.method();// 引用调用静态方法
>     }
> }
> ```
>
> 运行结果:
>
> ```
> 这是static方法,static方法与对象无关
> 这是非static方法,此方法必须和指定的对象关联起来才起作用
> 这是static方法,static方法与对象无关
> ```
>
> 结论:
>
> > 不可以。因为非static方法是要与对象关联在一起的，必须创建一个对象后，才可以在该对象上进行方法调用，而static方法调用时不需要创建对象，可以直接调用。也就是说，当一个static方法被调用时，可能还没有创建任何实例对象，如果从一个static方法中发出对非static方法的调用，那个非static方法是关联到哪个对象上的呢？这个逻辑无法成立，所以，一个static方法内部无法对非static方法的调用。

**静态方法里面只能调用静态方法或静态成员，非静态方法里面可以发出对static方法的调用**

## java修饰符

![image-20221020152339883](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221020152339883.png)

接口里的变量都隐式声明为 **public static final**,而接口里的方法默认情况下访问权限为 **public**。

声明为私有访问类型的变量只能通过类中公共的 getter 方法被外部类访问。

由于类的继承性，类所有的公有方法和变量都能被其子类继承。

### 受保护的访问修饰符-protected

protected 需要从以下两个点来分析说明：

- **子类与基类在同一包中**：被声明为 protected 的变量、方法和构造器能被同一个包中的任何其他类访问；
- **子类与基类不在同一包中**：那么在子类中，子类实例可以访问其从基类继承而来的 protected 方法，而不能访问基类实例的protected方法。

protected 可以修饰数据成员，构造方法，方法成员，**不能修饰类（内部类除外）**。

接口及接口的成员变量和成员方法不能声明为 protected。

子类能访问 protected 修饰符声明的方法和变量，这样就能保护不相关的类使用这些方法和变量。

下面的父类使用了 protected 访问修饰符，子类重写了父类的 openSpeaker() 方法。

```java
class AudioPlayer {   
protected boolean openSpeaker(Speaker sp) {     
	// 实现细节   
	} 
}  
class StreamingAudioPlayer extends AudioPlayer {   		protected boolean openSpeaker(Speaker sp) {
	// 实现细节   
	} 
}
```

如果把 openSpeaker() 方法声明为 private，那么除了 AudioPlayer 外，其他类将不能访问该方法。

如果把 openSpeaker() 声明为 public，那么所有的类都能够访问该方法。

如果我们只想让该方法对其所在类的子类可见，则将该方法声明为 protected。

> *protected 是最难理解的一种 Java 类成员访问权限修饰词，更多详细内容请查看* [Java protected 关键字详解](https://www.runoob.com/w3cnote/java-protected-keyword-detailed-explanation.html)*。*

### 访问控制和继承

请注意以下方法继承的规则：

- 父类中声明为 public 的方法在子类中也必须为 public。
- 父类中声明为 protected 的方法在子类中要么声明为 protected，要么声明为 public，不能声明为 private。
- 父类中声明为 private 的方法，不能够被子类继承。

### static 修饰符

- **静态变量：**

  static 关键字用来声明独立于对象的静态变量，无论一个类实例化多少对象，它的静态变量只有一份拷贝。 静态变量也被称为类变量。局部变量不能被声明为 static 变量。

- **静态方法：**

  static 关键字用来声明独立于对象的静态方法。静态方法不能使用类的非静态变量。静态方法从参数列表得到数据，然后计算这些数据。

### final 修饰符

**final 变量：**

final 表示"最后的、最终的"含义，变量一旦赋值后，不能被重新赋值。被 final 修饰的实例变量必须显式指定初始值。

final 修饰符通常和 static 修饰符一起使用来创建类常量。

**final 方法**

父类中的 final 方法可以被子类继承，但是不能被子类重写。

声明 final 方法的主要目的是防止该方法的内容被修改。

如下所示，使用 final 修饰符声明方法。

**final 类**

final 类不能被继承，没有类能够继承 final 类的任何特性。

### abstract 修饰符

**抽象类：**

抽象类不能用来实例化对象，声明抽象类的唯一目的是为了将来对该类进行扩充。

一个类不能同时被 abstract 和 final 修饰。如果一个类包含抽象方法，那么该类一定要声明为抽象类，否则将出现编译错误。

> **原因：**因为 abstract 是需要被子类继承覆盖的，否则毫无意义，而 final 作用是禁止继承的，两者相互排斥，所以不能共用。

抽象类可以包含抽象方法和非抽象方法。

**抽象方法**

抽象方法是一种没有任何实现的方法，该方法的具体实现由子类提供。

**抽象方法不能被声明成 final 和 static。**

> **原因：**因为 static 是类级别的不能被子类覆盖，而 abstract 需要被继承实现，两者相互矛盾。

**任何继承抽象类的子类必须实现父类的所有抽象方法，除非该子类也是抽象类。**

如果一个类**包含若干个抽象方法，那么该类必须声明为抽象类。**抽象类可以不包含抽象方法。

抽象方法的声明以分号结尾，例如：**public abstract sample();**。

> **从语法的角度来说，抽象类必须有构造方法，而接口严禁有构造方法，这本身也说明了它们性质的不同。**抽象类是一个类，别的类是用关键字 extends 来继承下来，并扩展的，有非常强的is-a的关系，这种关系一般来说符合里氏代换原则。而接口，是被其他类用关键字 implements 来实现接口定义的方法的。如果没什么区别，何必整出两个不同的关键字。  接口只是定义功能和行为规范，如果一个类实现了一个接口，那么这个类必须遵守这个接口的方法约定，但没有is-a的关系。把墙壁上的“小学生行为规范”想象成一个接口，那么是小学生必须遵守这个约定，但小学生不是“行为规范”。

### synchronized 修饰符

synchronized 关键字声明的方法同一时间只能被一个线程访问。synchronized 修饰符可以应用于四个访问修饰符。

 **实例**

```java
public synchronized void showDetails(){ 
.......
}
```

### transient 修饰符

序列化的对象包含被 transient 修饰的实例变量时，java 虚拟机(JVM)跳过该特定的变量。

该修饰符包含在定义变量的语句中，用来预处理类和变量的数据类型。

 **实例**

```java
public transient int limit = 55;   // 不会持久化 public int b; // 持久化
```

### volatile 修饰符

volatile 修饰的成员变量在每次被线程访问时，都强制从共享内存中重新读取该成员变量的值。而且，当成员变量发生变化时，会强制线程将变化值回写到共享内存。这样在任何时刻，两个不同的线程总是看到某个成员变量的同一个值。

一个 volatile 对象引用可能是 null。

 **实例**

```java
public class MyRunnable implements Runnable 
{
	private volatile boolean active;    
	public void run()    
    {        
		active = true;        
		while (active) // 第一行        
        {            
			// 代码        
		}    
	}    
	public void stop()    
    {
    	active = false; // 第二行    
    } 
}
```

通常情况下，在一个线程调用 run() 方法（在 Runnable 开启的线程），在另一个线程调用 stop() 方法。 如果 ***第一行\*** 中缓冲区的 active 值被使用，那么在 ***第二行\*** 的 active 值为 false 时循环不会停止。

但是以上代码中我们使用了 volatile 修饰 active，所以该循环会停止。

**static全局变量与普通的全局变量：**static全局变量只初使化一次，防止在其他文件单元中被引用;

**static局部变量和普通局部变量：**static局部变量只被初始化一次，下一次依据上一次结果值；

**static函数与普通函数：**static函数在内存中只有一份，普通函数在每个被调用中维持一份拷贝。

final以及final static修饰的变量的初始化方式：

> java访问权控制限修饰符：
>
> **final:**
>
> **修饰在类上：类不能被继承；**
>
> **修饰在方法上：方法继承下来不能被重写；**
>
> **修饰在属性上：属性只能被赋值一次，没有给默认值，必须在构造方法中赋值。**
>
> static:
>
> 修饰在成员变量和方法上，被其修饰的方法和变量从属于类优先加载在方法区，使其能被在堆内存中未加载的对象进行共享访问，static 不能修饰在局部语句块中的变量，因为无法共享该数据，被static修饰的方法不能在该方法里访问该类的对象属性和方法，因为当初始化static修饰的方法和成员变量语句块时，对象还未加载在堆内存，就还没有this对象的引用，就无法访问对象的方法和属性。如需访问必须先把对象new出来。
>
> synchronize：
>
> synchronize关键字修饰在方法上，在多线程中使用，该方法同一时间只能被一个线程访问，锁就是this
>
> transient:
>
> 修饰在包含定义变量的语句中将不会被序列化存储在硬盘
>
> volatile
>
> 修饰在成员变量上，在多线程中访问该变量，都会重新从线程中获取，使真实数据可见。

## java运算符

**在判断一个实例引用的类型时，使用的是实际类型，而不是声明的类型。**

子类是父类的类型，但父类不是子类的类型。

子类的实例可以声明为父类型，但父类的实例不能声明为子类型。

1. byte、short、int 类型的右移操作都是先将左操作数转换为int类型，然后执行右移操作，结果也是int类型。
2. long 类型的右移操作并没有对左操作数进行类型转换，结果也是long类型。

## java 循环结构

Java5 引入了一种主要用于数组的增强型 for 循环。

Java 增强 for 循环语法格式如下:

```java
for(声明语句 : 表达式)
{
   //代码句子
}
```

**声明语句：**声明新的局部变量，该变量的类型必须和数组元素的类型匹配。其作用域限定在循环语句块，其值与此时数组元素的值相等。

**表达式：**表达式是要访问的数组名，或者是返回值为数组的方法。

### 实例

### Test.java 文件代码：

```java
public class Test {
   public static void main(String[] args){
      int [] numbers = {10, 20, 30, 40, 50};
 
      for(int x : numbers ){
         System.out.print( x );
         System.out.print(",");
      }
      System.out.print("\n");
      String [] names ={"James", "Larry", "Tom", "Lacy"};
      for( String name : names ) {
         System.out.print( name );
         System.out.print(",");
      }
   }
}
```

以上实例编译运行结果如下：

```
10,20,30,40,50,
James,Larry,Tom,Lacy,
```

## Java Number & Math 类

**Java 中 int 和 Integer 的区别**

**1.** int 是基本数据类型，int 变量存储的是数值。Integer 是引用类型，实际是一个对象，Integer 存储的是引用对象的地址。

**2.**

```java
Integer i = new Integer(100);
Integer j = new Integer(100);
System.out.print(i == j); //false
```

因为 new 生成的是两个对象，其内存地址不同。

**3.**

int 和 Integer 所占内存比较：

Integer 对象会占用更多的内存。Integer 是一个对象，需要存储对象的元数据。但是 int 是一个原始类型的数据，所以占用的空间更少。

**4.** 非 new 生成的 Integer 变量与 **new Integer()** 生成的变量比较，结果为 false。

```java
/**
 * 比较非new生成的Integer变量与new生成的Integer变量
 */
public class Test {
    public static void main(String[] args) {
        Integer i= new Integer(200);
        Integer j = 200;
        System.out.print(i == j);
        //输出：false
    }
}
```

因为非 new 生成的 Integer 变量指向的是 java 常量池中的对象，而 **new Integer()** 生成的变量指向堆中新建的对象，两者在内存中的地址不同。所以输出为 false。

**5.** 两个非 new 生成的 Integer 对象进行比较，如果两个变量的值在区间 **[-128,127]** 之间，比较结果为 true；否则，结果为 false。

```java
/**
 * 比较两个非new生成的Integer变量
 */
public class Test {
    public static void main(String[] args) {
        Integer i1 = 127;
        Integer ji = 127;
        System.out.println(i1 == ji);//输出：true
        Integer i2 = 128;
        Integer j2 = 128;
        System.out.println(i2 == j2);//输出：false
    }
}
```

java 在编译 **Integer i1 = 127** 时，会翻译成 **Integer i1 = Integer.valueOf(127)**。

**6.** Integer 变量(无论是否是 new 生成的)与 int 变量比较，只要两个变量的值是相等的，结果都为 true。

```java
/**
 * 比较Integer变量与int变量
 */
public class Test {
    public static void main(String[] args) {
        Integer i1 = 200;
        Integer i2 = new Integer(200);
        int j = 200;
        System.out.println(i1 == j);//输出：true
        System.out.println(i2 == j);//输出：true
    }
}
```

包装类 Integer 变量在与基本数据类型 int 变量比较时，**Integer 会自动拆包装为 int**，然后进行比较，实际上就是两个 int 变量进行比较，值相等，所以为 true。

## Java String 类

![image-20221020230251743](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221020230251743.png)

### 创建格式化字符串

我们知道输出格式化数字可以使用 printf() 和 format() 方法。

String 类使用静态方法 format() 返回一个String 对象而不是 PrintStream 对象。

String 类的静态方法 format() 能用来创建可复用的格式化字符串，而不仅仅是用于一次打印输出。

如下所示：

```java
System.out.printf("浮点型变量的值为 " +                 	 "%f, 整型变量的值为 " + 
	 " %d, 字符串变量的值为 " +
	 "is %s", floatVar, intVar, stringVar);
```

你也可以这样写

```java
String fs; fs = String.format("浮点型变量的值为 " +                    "%f, 整型变量的值为 " +
			" %d, 字符串变量的值为 " +                   			 " %s", floatVar, intVar, stringVar);
```

**String 类的常见面试问题**

**面试题一：**

```
String s1 = "abc";            // 常量池
String s2 = new String("abc");     // 堆内存中
System.out.println(s1==s2);        // false两个对象的地址值不一样。
System.out.println(s1.equals(s2)); // true
```

**面试题二：**

```
String s1="a"+"b"+"c";
String s2="abc";
System.out.println(s1==s2);
System.out.println(s1.equals(s2));
```

java 中常量优化机制，编译时 **s1** 已经成为 **abc** 在常量池中查找创建，**s2** 不需要再创建。

**面试题三：**

```
String s1="ab";
String s2="abc";
String s3=s1+"c";
System.out.println(s3==s2);         // false
System.out.println(s3.equals(s2));  // true
```

先在常量池中创建 **ab** ，地址指向 **s1**, 再创建 **abc** ，指向 s2。对于 s3，先创建StringBuilder（或 StringBuffer）对象，通过 append 连接得到 **abc** ,再调用 toString() 转换得到的地址指向 **s3**。故 **(s3==s2)** 为 **false**。

**1.对整数进行格式化：%[index$][标识][最小宽度]转换方式**

格式化字符串由4部分组成，特殊的格式常以%index$开头，index从1开始取值，表示将第index个参数拿进来进行格式化，[最小宽度]的含义也很好理解，就是最终该整数转化的字符串最少包含多少位数字。剩下2个部分的含义：

标识：

-  '-' 在最小宽度内左对齐，不可以与"用0填充"同时使用
-  '#' 只适用于8进制和16进制，8进制时在结果前面增加一个0，16进制时在结果前面增加0x
-  '+' 结果总是包括一个符号(一般情况下只适用于10进制，若对象为BigInteger才可以用于8进制和16进制)
-  ' ' 正值前加空格，负值前加负号(一般情况下只适用于10进制，若对象为BigInteger才可以用于8进制和16进制)
-  '0' 结果将用零来填充
-  ',' 只适用于10进制，每3位数字之间用"，"分隔
-  '(' 若参数是负数，则结果中不添加负号而是用圆括号把数字括起来(同'+'具有同样的限制)

转换方式：

d-十进制 o-八进制 x或X-十六进制

上面的说明过于枯燥，我们来看几个具体的例子。需要特别注意的一点是：大部分标识字符可以同时使用。

```java
System.out.println(String.format("%1$,09d", -3123));
System.out.println(String.format("%1$9d", -31));
System.out.println(String.format("%1$-9d", -31));
System.out.println(String.format("%1$(9d", -31));
System.out.println(String.format("%1$#9x", 5689));
//结果为：
//-0003,123
// -31
//-31
// (31)
// 0x1639　
```

**2.对浮点数进行格式化：%[index$][标识][最少宽度][.精度]转换方式**

我们可以看到，浮点数的转换多了一个"精度"选项，可以控制小数点后面的位数。

标识：

- '-' 在最小宽度内左对齐，不可以与"用0填充"同时使用
- '+' 结果总是包括一个符号
- ' ' 正值前加空格，负值前加负号
- '0' 结果将用零来填充
- ',' 每3位数字之间用"，"分隔(只适用于fgG的转换)
- '(' 若参数是负数，则结果中不添加负号而是用圆括号把数字括起来(只适用于eEfgG的转换)

转换方式：

- 'e', 'E' -- 结果被格式化为用计算机科学记数法表示的十进制数
- 'f' -- 结果被格式化为十进制普通表示方式
- 'g', 'G' -- 根据具体情况，自动选择用普通表示方式还是科学计数法方式
- 'a', 'A' -- 结果被格式化为带有效位数和指数的十六进制浮点数

**3.对字符进行格式化：**

对字符进行格式化是非常简单的，c表示字符，标识中'-'表示左对齐，其他就没什么了。

##length() 方法，length 属性和 size() 方法的区别:

-  1、**length()** 方法是针对字符串来说的，要求一个字符串的长度就要用到它的length()方法；
-  2、**length 属性**是针对 Java 中的数组来说的，要求数组的长度可以用其 length 属性；
-  3、Java 中的 **size()** 方法是针对泛型集合说的, 如果想看这个泛型有多少个元素, 就调用此方法来查看!

这个例子来演示这两个方法和一个属性的用法：

```java
import java.util.ArrayList;
import java.util.List;

public class Main {

    public static void main(String[] args) {
        String array[] = { "First", "Second", "Third" };
        String a = "HelloWorld";
        List<String> list = new ArrayList<String>();
        list.add(a);
        System.out.println("数组array的长度为" + array.length);
        System.out.println("字符串a的长度为" + a.length());
        System.out.println("list中元素个数为" + list.size());

    }
}
```

输出的值为:

```
数组array的长度为3
字符串a的长度为10
list中元素个数为1
```

## Java StringBuffer 和 StringBuilder 类

![image-20221020232603826](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221020232603826.png)

![image-20221020232844307](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221020232844307.png)

## Java 正则表达式

java.util.regex 包主要包括以下三个类：

- Pattern 类：

  pattern 对象是一个正则表达式的编译表示。Pattern 类没有公共构造方法。要创建一个 Pattern 对象，你必须首先调用其公共静态编译方法，它返回一个 Pattern 对象。该方法接受一个正则表达式作为它的第一个参数。

- Matcher 类：

  Matcher 对象是对输入字符串进行解释和匹配操作的引擎。与Pattern 类一样，Matcher 也没有公共构造方法。你需要调用 Pattern 对象的 matcher 方法来获得一个 Matcher 对象。

- PatternSyntaxException：

  PatternSyntaxException 是一个非强制异常类，它表示一个正则表达式模式中的语法错误。

### 捕获组

捕获组是把多个字符当一个单独单元进行处理的方法，它通过对括号内的字符分组来创建。

例如，正则表达式 (dog) 创建了单一分组，组里包含"d"，"o"，和"g"。

捕获组是通过从左至右计算其开括号来编号。例如，在表达式（（A）（B（C））），有四个这样的组：

- ((A)(B(C)))
- (A)
- (B(C))
- (C)

可以通过调用 matcher 对象的 groupCount 方法来查看表达式有多少个分组。groupCount 方法返回一个 int 值，表示matcher对象当前有多个捕获组。

还有一个特殊的组（group(0)），它总是代表整个表达式。该组不包括在 groupCount 的返回值中。

### 正则表达式语法

在其他语言中，**\\** 表示：**我想要在正则表达式中插入一个普通的（字面上的）反斜杠，请不要给它任何特殊的意义。**

在 Java 中，**\\** 表示：**我要插入一个正则表达式的反斜线，所以其后的字符具有特殊的意义。**

所以，在其他的语言中（如 Perl），一个反斜杠 **\** 就足以具有转义的作用，而在 Java 中正则表达式中则需要有两个反斜杠才能被解析为其他语言中的转义作用。也可以简单的理解在 Java 的正则表达式中，两个 **\\** 代表其他语言中的一个 **\**，这也就是为什么表示一位数字的正则表达式是 **\\d**，而表示一个普通的反斜杠是 **\\**。

```java
System.out.print("\\");    // 输出为 \
System.out.print("\\\\");  // 输出为 \\
```

| 字符          | 说明                                                         |
| :------------ | :----------------------------------------------------------- |
| \             | 将下一字符标记为特殊字符、文本、反向引用或八进制转义符。例如， **n**匹配字符 **n**。**\n** 匹配换行符。序列 **\\\\** 匹配 **\\** ，**\\(** 匹配 **(**。 |
| ^             | 匹配输入字符串开始的位置。如果设置了 **RegExp** 对象的 **Multiline** 属性，^ 还会与"\n"或"\r"之后的位置匹配。 |
| $             | 匹配输入字符串结尾的位置。如果设置了 **RegExp** 对象的 **Multiline** 属性，$ 还会与"\n"或"\r"之前的位置匹配。 |
| *             | 零次或多次匹配前面的字符或子表达式。例如，zo* 匹配"z"和"zoo"。* 等效于 {0,}。 |
| +             | 一次或多次匹配前面的字符或子表达式。例如，"zo+"与"zo"和"zoo"匹配，但与"z"不匹配。+ 等效于 {1,}。 |
| ?             | 零次或一次匹配前面的字符或子表达式。例如，"do(es)?"匹配"do"或"does"中的"do"。? 等效于 {0,1}。 |
| {*n*}         | *n* 是非负整数。正好匹配 *n* 次。例如，"o{2}"与"Bob"中的"o"不匹配，但与"food"中的两个"o"匹配。 |
| {*n*,}        | *n* 是非负整数。至少匹配 *n* 次。例如，"o{2,}"不匹配"Bob"中的"o"，而匹配"foooood"中的所有 o。"o{1,}"等效于"o+"。"o{0,}"等效于"o*"。 |
| {*n*,*m*}     | *m* 和 *n* 是非负整数，其中 *n* <= *m*。匹配至少 *n* 次，至多 *m* 次。例如，"o{1,3}"匹配"fooooood"中的头三个 o。'o{0,1}' 等效于 'o?'。注意：您不能将空格插入逗号和数字之间。 |
| ?             | 当此字符紧随任何其他限定符（*、+、?、{*n*}、{*n*,}、{*n*,*m*}）之后时，匹配模式是"非贪心的"。"非贪心的"模式匹配搜索到的、尽可能短的字符串，而默认的"贪心的"模式匹配搜索到的、尽可能长的字符串。例如，在字符串"oooo"中，"o+?"只匹配单个"o"，而"o+"匹配所有"o"。 |
| .             | 匹配除"\r\n"之外的任何单个字符。若要匹配包括"\r\n"在内的任意字符，请使用诸如"[\s\S]"之类的模式。 |
| (*pattern*)   | 匹配 *pattern* 并捕获该匹配的子表达式。可以使用 **$0…$9** 属性从结果"匹配"集合中检索捕获的匹配。若要匹配括号字符 ( )，请使用"\("或者"\)"。 |
| (?:*pattern*) | 匹配 *pattern* 但不捕获该匹配的子表达式，即它是一个非捕获匹配，不存储供以后使用的匹配。这对于用"or"字符 (\|) 组合模式部件的情况很有用。例如，'industr(?:y\|ies) 是比 'industry\|industries' 更经济的表达式。 |
| (?=*pattern*) | 执行正向预测先行搜索的子表达式，该表达式匹配处于匹配 *pattern* 的字符串的起始点的字符串。它是一个非捕获匹配，即不能捕获供以后使用的匹配。例如，'Windows (?=95\|98\|NT\|2000)' 匹配"Windows 2000"中的"Windows"，但不匹配"Windows 3.1"中的"Windows"。预测先行不占用字符，即发生匹配后，下一匹配的搜索紧随上一匹配之后，而不是在组成预测先行的字符后。 |
| (?!*pattern*) | 执行反向预测先行搜索的子表达式，该表达式匹配不处于匹配 *pattern* 的字符串的起始点的搜索字符串。它是一个非捕获匹配，即不能捕获供以后使用的匹配。例如，'Windows (?!95\|98\|NT\|2000)' 匹配"Windows 3.1"中的 "Windows"，但不匹配"Windows 2000"中的"Windows"。预测先行不占用字符，即发生匹配后，下一匹配的搜索紧随上一匹配之后，而不是在组成预测先行的字符后。 |
| *x*\|*y*      | 匹配 *x* 或 *y*。例如，'z\|food' 匹配"z"或"food"。'(z\|f)ood' 匹配"zood"或"food"。 |
| [*xyz*]       | 字符集。匹配包含的任一字符。例如，"[abc]"匹配"plain"中的"a"。 |
| [^*xyz*]      | 反向字符集。匹配未包含的任何字符。例如，"[^abc]"匹配"plain"中"p"，"l"，"i"，"n"。 |
| [*a-z*]       | 字符范围。匹配指定范围内的任何字符。例如，"[a-z]"匹配"a"到"z"范围内的任何小写字母。 |
| [^*a-z*]      | 反向范围字符。匹配不在指定的范围内的任何字符。例如，"[^a-z]"匹配任何不在"a"到"z"范围内的任何字符。 |
| \b            | 匹配一个字边界，即字与空格间的位置。例如，"er\b"匹配"never"中的"er"，但不匹配"verb"中的"er"。 |
| \B            | 非字边界匹配。"er\B"匹配"verb"中的"er"，但不匹配"never"中的"er"。 |
| \c*x*         | 匹配 *x* 指示的控制字符。例如，\cM 匹配 Control-M 或回车符。*x* 的值必须在 A-Z 或 a-z 之间。如果不是这样，则假定 c 就是"c"字符本身。 |
| \d            | 数字字符匹配。等效于 [0-9]。                                 |
| \D            | 非数字字符匹配。等效于 [^0-9]。                              |
| \f            | 换页符匹配。等效于 \x0c 和 \cL。                             |
| \n            | 换行符匹配。等效于 \x0a 和 \cJ。                             |
| \r            | 匹配一个回车符。等效于 \x0d 和 \cM。                         |
| \s            | 匹配任何空白字符，包括空格、制表符、换页符等。与 [ \f\n\r\t\v] 等效。 |
| \S            | 匹配任何非空白字符。与 [^ \f\n\r\t\v] 等效。                 |
| \t            | 制表符匹配。与 \x09 和 \cI 等效。                            |
| \v            | 垂直制表符匹配。与 \x0b 和 \cK 等效。                        |
| \w            | 匹配任何字类字符，包括下划线。与"[A-Za-z0-9_]"等效。         |
| \W            | 与任何非单词字符匹配。与"[^A-Za-z0-9_]"等效。                |
| \x*n*         | 匹配 *n*，此处的 *n* 是一个十六进制转义码。十六进制转义码必须正好是两位数长。例如，"\x41"匹配"A"。"\x041"与"\x04"&"1"等效。允许在正则表达式中使用 ASCII 代码。 |
| \*num*        | 匹配 *num*，此处的 *num* 是一个正整数。到捕获匹配的反向引用。例如，"(.)\1"匹配两个连续的相同字符。 |
| \*n*          | 标识一个八进制转义码或反向引用。如果 \*n* 前面至少有 *n* 个捕获子表达式，那么 *n* 是反向引用。否则，如果 *n* 是八进制数 (0-7)，那么 *n* 是八进制转义码。 |
| \*nm*         | 标识一个八进制转义码或反向引用。如果 \*nm* 前面至少有 *nm* 个捕获子表达式，那么 *nm* 是反向引用。如果 \*nm* 前面至少有 *n* 个捕获，则 *n* 是反向引用，后面跟有字符 *m*。如果两种前面的情况都不存在，则 \*nm* 匹配八进制值 *nm*，其中 *n* 和 *m* 是八进制数字 (0-7)。 |
| \nml          | 当 *n* 是八进制数 (0-3)，*m* 和 *l* 是八进制数 (0-7) 时，匹配八进制转义码 *nml*。 |
| \u*n*         | 匹配 *n*，其中 *n* 是以四位十六进制数表示的 Unicode 字符。例如，\u00A9 匹配版权符号 (©)。 |

> 根据 Java Language Specification 的要求，Java 源代码的字符串中的反斜线被解释为 Unicode 转义或其他字符转义。因此必须在字符串字面值中使用两个反斜线，表示正则表达式受到保护，不被 Java 字节码编译器解释。例如，当解释为正则表达式时，字符串字面值 "\b" 与单个退格字符匹配，而 "\\b" 与单词边界匹配。字符串字面值 "\(hello\)" 是非法的，将导致编译时错误；要与字符串 (hello) 匹配，必须使用字符串字面值 "\\(hello\\)"。

## Java 方法

```java
public class TestPassByValue {
  public static void main(String[] args) {
    int num1 = 1;
    int num2 = 2;
 
    System.out.println("交换前 num1 的值为：" +
                        num1 + " ，num2 的值为：" + num2);
 
    // 调用swap方法
    swap(num1, num2);
    System.out.println("交换后 num1 的值为：" +
                       num1 + " ，num2 的值为：" + num2);
  }
  /** 交换两个变量的方法 */
  public static void swap(int n1, int n2) {
    System.out.println("\t进入 swap 方法");
    System.out.println("\t\t交换前 n1 的值为：" + n1
                         + "，n2 的值：" + n2);
    // 交换 n1 与 n2的值
    int temp = n1;
    n1 = n2;
    n2 = temp;
 
    System.out.println("\t\t交换后 n1 的值为 " + n1
                         + "，n2 的值：" + n2);
  }
}
```

以上实例编译运行结果如下：

```
交换前 num1 的值为：1 ，num2 的值为：2
    进入 swap 方法
        交换前 n1 的值为：1，n2 的值：2
        交换后 n1 的值为 2，n2 的值：1
交换后 num1 的值为：1 ，num2 的值为：2
```

传递两个参数调用swap方法。有趣的是，方法被调用后，实参的值并没有改变。

**对于 Java 的可变参数:**

```
typeName... parameterName
```

一个函数至多只能有一个可变参数，且可变参数为最后一个参数。对于可变参数，编译器会将其转型为一个数组，故在函数内部，可变参数名即可看作数组名。

且

```
void function(String... args);
void function(String [] args);
```

这两个方法的命名是相等的，不能作为方法的重载。

可变参数，即可向函数传递 0 个或多个参数，如：

```
void function("Wallen","John","Smith");
void function(new String [] {"Wallen","John","Smith"});
```

这两种调用方法效果是一样的。

对于可变参数的方法重载。

```
void function(String... args);
void function(String args1,String args2);
function("Wallen","John");
```

优先匹配固定参数的方法。

**方法中的参数绑定&变量类型的理解巩固:**

**参数绑定：**调用方把参数传递给实例方法时，调用时传递的值会按参数位置一一绑定。

基本类型参数的传递实例：

```java
public class Main {
    public static void main(String[] args) {
        Person p = new Person();
        int n = 15; // n的值为15    tip:基本类型变量
        p.setAge(n); // 传入n的值   tip:参数n传递的是值
        System.out.println(p.getAge()); // 15
        n = 20; // n的值改为20
        System.out.println(p.getAge()); // 15还是20?   tip:15
    }
}

class Person {
    private int age;  

    public int getAge() {   
        return this.age;
    }

    public void setAge(int age) {
        this.age = age;
    }
}
```

基本类型参数的传递，是调用方值的复制，双方各自的后续修改，互不影响。

基本类型变量：“持有某个数值”，变量名指向具体的数值。

引用类型参数的传递实例：

```java
public class Main {
    public static void main(String[] args) {
        Game g = new Game(); 
        String[] gamename = { "王者", "荣耀" };  // gamename变量指向的是这个数组的内存地址
        g.setName(gamename); // 传入gamename数组  tip:传入的是内存地址 ↑
        System.out.println(g.getName()); // 王者荣耀
        gamename[1] = "农药"; // gamename数组的第二个元素修改为"农药"
        System.out.println(g.getName()); // "王者荣耀"还是"王者农药"?  tip:王者农药
    }                                    
}

class Game {
    private String[] name;

    public String getName() {
        return this.name[0] + " " + this.name[1];
    }

    public void setName(String[] name) {
        this.name = name;
    }
}
```

引用类型参数的传递，调用方的变量，和接收方的参数变量，指向的是同一个数组地址(内存地址)。**双方任意一方对这个对象(数组)的修改，都会影响对方（因为指向同一个对象）**

引用类型变量：变量名指向某个对象的内存地址。

## Java 流(Stream)、文件(File)和IO

![image-20221021092245610](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221021092245610.png)

### FileInputStream

该流用于从文件读取数据，它的对象可以用关键字 new 来创建。

有多种构造方法可用来创建对象。

可以使用字符串类型的文件名来创建一个输入流对象来读取文件：

```java
InputStream f = new FileInputStream("C:/java/hello");
```

也可以使用一个文件对象来创建一个输入流对象来读取文件。我们首先得使用 File() 方法来创建一个文件对象：

```java
File f = new File("C:/java/hello"); 
InputStream in = new FileInputStream(f);
```

## FileOutputStream

该类用来创建一个文件并向文件中写数据。

如果该流在打开文件进行输出前，目标文件不存在，那么该流会创建该文件。

有两个构造方法可以用来创建 FileOutputStream 对象。

使用字符串类型的文件名来创建一个输出流对象：

```java
OutputStream f = new FileOutputStream("C:/java/hello");
```

也可以使用一个文件对象来创建一个输出流来写文件。我们首先得使用File()方法来创建一个文件对象：

```java
File f = new File("C:/java/hello"); 
OutputStream fOut = new FileOutputStream(f);
```

### 创建目录：

File类中有两个方法可以用来创建文件夹：

- **mkdir( )**方法创建一个文件夹，成功则返回true，失败则返回false。失败表明File对象指定的路径已经存在，或者由于整个路径还不存在，该文件夹不能被创建。
- **mkdirs()**方法创建一个文件夹和它的所有父文件夹。

### 读取目录

一个目录其实就是一个 File 对象，它包含其他文件和文件夹。

如果创建一个 File 对象并且它是一个目录，那么调用 isDirectory() 方法会返回 true。

可以通过调用该对象上的 list() 方法，来提取它包含的文件和文件夹的列表。

### 删除目录或文件

删除文件可以使用 **java.io.File.delete()** 方法。

以下代码会删除目录 **/tmp/java/**，需要注意的是当删除某一目录时，必须保证该目录下没有其他文件才能正确删除，否则将删除失败。

- BufferedReader 是支持同步的，而 Scanner 不支持。如果我们处理多线程程序，BufferedReader 应当使用。

- BufferedReader 相对于 Scanner 有足够大的缓冲区内存。

- Scanner 有很少的缓冲区(1KB 字符缓冲)相对于 BufferedReader(8KB字节缓冲)，但是这是绰绰有余的。

- BufferedReader 相对于 Scanner 来说要快一点，因为 Scanner 对输入数据进行类解析，而 BufferedReader 只是简单地读取字符序列。

有时需要列出目录下指定类型的文件，例如：. java、. txt 等扩展名的文件。可以使用File类的下述两个方法，列出指定类型的文件：

-  1、**public String[ ] list(FilenameFilter obj)** 该方法用字符串的形式返回目录下的指定类型的所有文件
-  2、**public File[ ] listFile(FilenameFilter obj)** 该方法用 File 对象形式返回目录下的指定类型的所有文件

上述两个方法的参数 FilenameFilter 是一个接口，该接口有一个方法：

```java
public boolean accept(File f,String name);
```

File 对象 f 调用 list 方法时，需向该方法传递一个实现 FilenameFilter 接口的对象，list 方法执行时，参数 obj 不断回调接口方法accept(File f,String name)，该方法中的参数 f 为调用 list 的当前目录，参数 name 被实例化为当前目录中的一个文件名，当接口方法返回 true 时，list 方法就将名字为 name 的文件存放到返回的数组中。

## Java Scanner 类

java.util.Scanner 是 Java5 的新特征，我们可以通过 Scanner 类来获取用户的输入。

下面是创建 Scanner 对象的基本语法：

Scanner s = new Scanner(System.in);

接下来我们演示一个最简单的数据输入，并通过 Scanner 类的 next() 与 nextLine() 方法获取输入的字符串，在读取前我们一般需要 使用 hasNext 与 hasNextLine 判断是否还有输入的数据：



### 使用 next 方法：

ScannerDemo.java 文件代码：

```java
import java.util.Scanner; 
 
public class ScannerDemo {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        // 从键盘接收数据
 
        // next方式接收字符串
        System.out.println("next方式接收：");
        // 判断是否还有输入
        if (scan.hasNext()) {
            String str1 = scan.next();
            System.out.println("输入的数据为：" + str1);
        }
        scan.close();
    }
}
```

执行以上程序输出结果为：

```java
$ javac ScannerDemo.java
$ java ScannerDemo
next方式接收：
runoob com
输入的数据为：runoob
```

可以看到 com 字符串并未输出，接下来我们看 nextLine。

### 使用 nextLine 方法：

ScannerDemo.java 文件代码：

```java
import java.util.Scanner;
 
public class ScannerDemo {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        // 从键盘接收数据
 
        // nextLine方式接收字符串
        System.out.println("nextLine方式接收：");
        // 判断是否还有输入
        if (scan.hasNextLine()) {
            String str2 = scan.nextLine();
            System.out.println("输入的数据为：" + str2);
        }
        scan.close();
    }
}
```

执行以上程序输出结果为：

```
$ javac ScannerDemo.java
$ java ScannerDemo
nextLine方式接收：
runoob com
输入的数据为：runoob com
```

可以看到 com 字符串输出。

### next() 与 nextLine() 区别

next():

- 1、一定要读取到有效字符后才可以结束输入。
- 2、对输入有效字符之前遇到的空白，next() 方法会自动将其去掉。
- 3、只有输入有效字符后才将其后面输入的空白作为分隔符或者结束符。
- next() 不能得到带有空格的字符串。

nextLine()：

- 1、以Enter为结束符,也就是说 nextLine()方法返回的是输入回车之前的所有字符。
- 2、可以获得空白。

## Java 异常处理

异常发生的原因有很多，通常包含以下几大类：

- 用户输入了非法数据。
- 要打开的文件不存在。
- 网络通信时连接中断，或者JVM内存溢出。

要理解Java异常处理是如何工作的，你需要掌握以下三种类型的异常：

- **检查性异常：**最具代表的检查性异常是**用户错误或问题引起的异常**，这是程序员无法预见的。例如要打开一个不存在文件时，一个异常就发生了，这些异常在编译时不能被简单地忽略。
- **运行时异常：** 运行时异常是可能被程序员避免的异常。与检查性异常相反，运行时异常**可以在编译时被忽略**。
- **错误：** 错误不是异常，而是脱离程序员控制的问题。错误在代码中通常被忽略。例如，当栈溢出时，一个错误就发生了，它们在编译也检查不到的。

![image-20221023001248839](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221023001248839.png)

### throws/throw 关键字：

如果一个方法没有捕获到一个检查性异常，那么该方法必须使用 throws 关键字来声明。throws 关键字放在方法签名的尾部。

也可以使用 throw 关键字抛出一个异常，无论它是新实例化的还是刚捕获到的。

下面方法的声明抛出一个 RemoteException 异常：

```java
import java.io.*;
public class className
{
  public void deposit(double amount) throws RemoteException
  {
    // Method implementation
    throw new RemoteException();
  }
  //Remainder of class definition
}
```

一个方法可以声明抛出多个异常，多个异常之间用逗号隔开。

例如，下面的方法声明抛出 RemoteException 和 InsufficientFundsException：

```java
import java.io.*;
public class className
{
   public void withdraw(double amount) throws RemoteException,
                              InsufficientFundsException
   {
       // Method implementation
   }
   //Remainder of class definition
}
```

### try-with-resources

JDK7 之后，Java 新增的 **try-with-resource** 语法糖来打开资源，并且可以在语句执行完毕后确保每个资源都被自动关闭 。

JDK7 之前所有被打开的系统资源，比如流、文件或者 Socket 连接等，都需要被开发者手动关闭，否则将会造成资源泄露。

```java
try (resource declaration) {
  // 使用的资源
} catch (ExceptionType e1) {
  // 异常块
}
```

以上的语法中 try 用于声明和实例化资源，catch 用于处理关闭资源时可能引发的所有异常。

**注意：**try-with-resources 语句关闭所有实现 AutoCloseable 接口的资源。

```java
import java.io.*;

public class RunoobTest {

    public static void main(String[] args) {
    String line;
        try(BufferedReader br = new BufferedReader(new FileReader("test.txt"))) {
            while ((line = br.readLine()) != null) {
                System.out.println("Line =>"+line);
            }
        } catch (IOException e) {
            System.out.println("IOException in try block =>" + e.getMessage());
        }
    }
}
```

以上实例输出结果为：

```
IOException in try block =>test.txt (No such file or directory)
```

以上实例中，我们实例一个 BufferedReader对象从 test.txt 文件中读取数据。

在 try-with-resources 语句中声明和实例化 BufferedReader 可确语句执行完毕后实例资源，不需要考虑 try 语句是正常执行还是抛出异常。

如果发生异常，可以使用 catch 来处理异常。

再看下不使用 **try-with-resources** 而改成 **finally** 来关闭资源，整体代码量多了很多，而且更复杂繁琐了：

```java
import java.io.*;

class RunoobTest {
    public static void main(String[] args) {
        BufferedReader br = null;
        String line;

        try {
            System.out.println("Entering try block");
            br = new BufferedReader(new FileReader("test.txt"));
            while ((line = br.readLine()) != null) {
            System.out.println("Line =>"+line);
            }
        } catch (IOException e) {
            System.out.println("IOException in try block =>" + e.getMessage());
        } finally {
            System.out.println("Entering finally block");
            try {
                if (br != null) {
                    br.close();
                }
            } catch (IOException e) {
                System.out.println("IOException in finally block =>"+e.getMessage());
            }
        }
    }
}
```

以上实例输出结果为：

```
Entering try block
IOException in try block =>test.txt (No such file or directory)
Entering finally block
```

### try-with-resources 处理多个资源

try-with-resources 语句中可以声明多个资源，方法是使用分号 **;** 分隔各个资源：

### 实例

```java
import java.io.*;
import java.util.*;
class RunoobTest {
    public static void main(String[] args) throws IOException{
        try (Scanner scanner = new Scanner(new File("testRead.txt"));
            PrintWriter writer = new PrintWriter(new File("testWrite.txt"))) {
            while (scanner.hasNext()) {
                writer.print(scanner.nextLine());
            }
        }
    }
}
```

以上实例使用 Scanner 对象从 testRead.txt 文件中读取一行并将其写入新的 testWrite.txt 文件中。

多个声明资源时，**try-with-resources** 语句以相反的顺序关闭这些资源。 在本例中，PrintWriter 对象先关闭，然后 Scanner 对象关闭。

### 声明自定义异常

在 Java 中你可以自定义异常。编写自己的异常类时需要记住下面的几点。

- 所有异常都必须是 Throwable 的子类。
- 如果希望写一个检查性异常类，则需要继承 Exception 类。
- 如果你想写一个运行时异常类，那么需要继承 RuntimeException 类。

可以像下面这样定义自己的异常类：

```java
class MyException extends Exception{ }
```

只继承Exception 类来创建的异常类是检查性异常类。

下面的 InsufficientFundsException 类是用户定义的异常类，它继承自 Exception。

一个异常类和其它任何类一样，包含有变量和方法。

### 实例

以下实例是一个银行账户的模拟，通过银行卡的号码完成识别，可以进行存钱和取钱的操作。

**InsufficientFundsException.java 文件代码：**

```java
// 文件名InsufficientFundsException.java
import java.io.*;
 
//自定义异常类，继承Exception类
public class InsufficientFundsException extends Exception
{
  //此处的amount用来储存当出现异常（取出钱多于余额时）所缺乏的钱
  private double amount;
  public InsufficientFundsException(double amount)
  {
    this.amount = amount;
  } 
  public double getAmount()
  {
    return amount;
  }
}
```

为了展示如何使用我们自定义的异常类，

在下面的 CheckingAccount 类中包含一个 withdraw() 方法抛出一个 InsufficientFundsException 异常。

**CheckingAccount.java 文件代码：**

```java
// 文件名称 CheckingAccount.java
import java.io.*;
 
//此类模拟银行账户
public class CheckingAccount
{
  //balance为余额，number为卡号
   private double balance;
   private int number;
   public CheckingAccount(int number)
   {
      this.number = number;
   }
  //方法：存钱
   public void deposit(double amount)
   {
      balance += amount;
   }
  //方法：取钱
   public void withdraw(double amount) throws
                              InsufficientFundsException
   {
      if(amount <= balance)
      {
         balance -= amount;
      }
      else
      {
         double needs = amount - balance;
         throw new InsufficientFundsException(needs);
      }
   }
  //方法：返回余额
   public double getBalance()
   {
      return balance;
   }
  //方法：返回卡号
   public int getNumber()
   {
      return number;
   }
}
```

下面的 BankDemo 程序示范了如何调用 CheckingAccount 类的 deposit() 和 withdraw() 方法。

**BankDemo.java 文件代码：**

```java
//文件名称 BankDemo.java
public class BankDemo
{
   public static void main(String [] args)
   {
      CheckingAccount c = new CheckingAccount(101);
      System.out.println("Depositing $500...");
      c.deposit(500.00);
      try
      {
         System.out.println("\nWithdrawing $100...");
         c.withdraw(100.00);
         System.out.println("\nWithdrawing $600...");
         c.withdraw(600.00);
      }catch(InsufficientFundsException e)
      {
         System.out.println("Sorry, but you are short $"
                                  + e.getAmount());
         e.printStackTrace();
      }
    }
}
```

编译上面三个文件，并运行程序 BankDemo，得到结果如下所示：

```
Depositing $500...

Withdrawing $100...

Withdrawing $600...
Sorry, but you are short $200.0
InsufficientFundsException
        at CheckingAccount.withdraw(CheckingAccount.java:25)
        at BankDemo.main(BankDemo.java:13)
```
