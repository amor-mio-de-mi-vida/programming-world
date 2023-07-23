# **JAVA面向对象**

## Java 继承

![image-20221022104351953](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221022104351953.png)

### 继承的特性

- **子类拥有父类非 private 的属性、方法。**
- 子类可以拥有自己的属性和方法，即子类可以对父类进行扩展。
- 子类可以用自己的方式实现父类的方法。（重写）
- Java 的继承是单继承，但是可以多重继承，单继承就是一个子类只能继承一个父类，多重继承就是，例如 B 类继承 A 类，C 类继承 B 类，所以按照关系就是 B 类是 C 类的父类，A 类是 B 类的父类，这是 Java 继承区别于 C++ 继承的一个特性。
- 提高了类之间的耦合性（继承的缺点，耦合度高就会造成代码之间的联系越紧密，代码独立性越差）。

### 构造器

子类是不继承父类的构造器（构造方法或者构造函数）的，它只是调用（隐式或显式）。如果父类的构造器带有参数，则必须在子类的构造器中显式地通过 **super** 关键字调用父类的构造器并配以适当的参数列表。

如果父类构造器没有参数，则在子类的构造器中不需要使用 **super** 关键字调用父类构造器，系统会自动调用父类的无参构造器。

**注意：**对理解继承来说，最重要的事情是，知道哪些东西被继承了，或者说，子类从父类那里得到了什么。答案是：所有的东西，所有的父类的成员，包括变量和方法，都成为了子类的成员，除了构造方法。构造方法是父类所独有的，因为它们的名字就是类的名字，所以父类的构造方法在子类中不存在。除此之外，子类继承得到了父类所有的成员。

但是得到不等于可以随便使用。每个成员有不同的访问属性，子类继承得到了父类所有的成员，但是不同的访问属性使得子类在使用这些成员时有所不同：有些父类的成员直接成为子类的对外的界面，有些则被深深地隐藏起来，即使子类自己也不能直接访问。下表列出了不同访问属性的父类成员在子类中的访问属性：

| 父类成员访问属性 | 在父类中的含义                     | 在子类中的含义                                               |
| ---------------- | ---------------------------------- | ------------------------------------------------------------ |
| public           | 对所有人开放                       | 对所有人开放                                                 |
| protected        | 只有包内其它类、自己和子类可以访问 | 只有包内其它类、自己和子类可以访问                           |
| 缺省             | 只有包内其它类可以访问             | 如果子类与父类在同一个包内：只有包内其它类可以访问，否则：相当于private，不能访问 |
| private          | 只有自己可以访问                   | 不能访问                                                     |

public的成员直接成为子类的public的成员，protected的成员也直接成为子类的protected的成员。Java的protected的意思是包内和子类可访问，所以它比缺省的访问属性要宽一些。而对于父类的缺省的未定义访问属性的成员来说，他们是在父类所在的包内可见，如果子类不属于父类的包，那么在子类里面，这些缺省属性的成员和private的成员是一样的：不可见。父类的private的成员在子类里仍然是存在的，只是子类中不能直接访问。我们不可以在子类中重新定义继承得到的成员的访问属性。如果我们试图重新定义一个在父类中已经存在的成员变量，那么我们是在定义一个与父类的成员变量完全无关的变量，在子类中我们可以访问这个定义在子类中的变量，在父类的方法中访问父类的那个。尽管它们同名但是互不影响。

在构造一个子类的对象时，父类的构造方法也是会被调用的，而且父类的构造方法在子类的构造方法之前被调用。在程序运行过程中，子类对象的一部分空间存放的是父类对象。因为子类从父类得到继承，在子类对象初始化过程中可能会使用到父类的成员。所以父类的空间正是要先被初始化的，然后子类的空间才得到初始化。在这个过程中，如果父类的构造方法需要参数，如何传递参数就很重要了。

**总结：**

1、父类引用指向子类对象，而子类引用不能指向父类对象。

2、把子类对象直接赋给父类引用叫upcasting向上转型，向上转型不用强制转换吗，如：

```
Father f1 = new Son();
```

3、把指向子类对象的父类引用赋给子类引用叫向下转型(downcasting)，要强制转换，如：

f1 就是一个**指向子类对象的父类引用**。把f1赋给子类引用 s1 即 **Son s1 = (Son)f1;**

其中 f1 前面的(Son)必须加上，进行强制转换。

**在编写代码要注意：**

-  1.如果父类中不含 默认构造函数（就是 类名() ），那么子类中的super()语句就会执行失败，系统就会报错。一般 默认构造函数 编译时会自动添加，但如果类中已经有一个构造函数时，就不会添加。
-  2.执行父类构造函数的语句只能放在函数内语句的首句，不然会报错。
-  静态方法中不能使用 super 关键字和 this 关键字。

在继承关系中，在调用函数（方法）或者类中的成员变量时，JVM（JAVA虚拟机）会先检测当前的类（也就是子类）是否含有该函数或者成员变量，如果有，就执行子类中的，如果没有才会执行父类中的。

当子类出现与父类一样的函数时，这个被称为 **重写** 也叫 **覆盖**

Object类是所有类的直接父类或间接父类，也就是说是所有类的根父类，这个可以运用于参数的传递

**this 关键字用法：**限定当前对象的数据域变量。一般用于方法内的局部变量与对象的数据域变量同名的情况。如 this.num = num。this.num 表示当前对象的数据域变量 num，而 num 表示方法中的局部变量。

## Java 重写(Override)与重载(Overload)

### 重写(Override)

重写是子类对父类的允许访问的方法的实现过程进行重新编写, 返回值和形参都不能改变。**即外壳不变，核心重写！**

**重写方法不能抛出新的检查异常或者比被重写方法申明更加宽泛的异常。**例如： 父类的一个方法申明了一个检查异常 IOException，但是在重写这个方法的时候不能抛出 Exception 异常，因为 Exception 是 IOException 的父类，抛出 IOException 异常或者 IOException 的子类异常。

### 方法的重写规则

- **参数列表与被重写方法的参数列表必须完全相同。**
- 返回类型与被重写方法的返回类型可以不相同，但是必须是父类返回值的派生类（java5 及更早版本返回类型要一样，java7 及更高版本可以不同）。
- **访问权限不能比父类中被重写的方法的访问权限更低**。例如：如果父类的一个方法被声明为 public，那么在子类中重写该方法就不能声明为 protected。
- 父类的成员方法只能被它的子类重写。
- **声明为 final 的方法不能被重写。**
- **声明为 static 的方法不能被重写，但是能够被再次声明。**
- 子类和父类在同一个包中，那么子类可以重写父类所有方法，除了声明为 private 和 final 的方法。
- 子类和父类不在同一个包中，那么子类只能够重写父类的声明为 public 和 protected 的非 final 方法。
- 重写的方法能够抛出任何非强制异常，无论被重写的方法是否抛出异常。但是，重写的方法不能抛出新的强制性异常，或者比被重写方法声明的更广泛的强制性异常，反之则可以。
- 构造方法不能被重写。
- 如果不能继承一个类，则不能重写该类的方法。

## 重载(Overload)

重载(overloading) 是在一个类里面，方法名字相同，而参数不同。返回类型可以相同也可以不同。

每个重载的方法（或者构造函数）都必须有一个独一无二的参数类型列表。

最常用的地方就是构造器的重载。

**重载规则:**

- 被重载的方法必须改变参数列表(参数个数或类型不一样)；
- 被重载的方法可以改变返回类型；
- 被重载的方法可以改变访问修饰符；
- 被重载的方法可以声明新的或更广的检查异常；
- 方法能够在同一个类中或者在一个子类中被重载。
- 无法以返回值类型作为重载函数的区分标准。

## 总结

方法的重写(Overriding)和重载(Overloading)是java多态性的不同表现，重写是父类与子类之间多态性的一种表现，重载可以理解成多态的具体表现形式。

- (1)方法重载是一个类中定义了多个方法名相同,而他们的参数的数量不同或数量相同而类型和次序不同,则称为方法的重载(Overloading)。
- (2)方法重写是在子类存在方法与父类的方法的名字相同,而且参数的个数与类型一样,返回值也一样的方法,就称为重写(Overriding)。
- (3)方法重载是一个类的多态性表现,而方法重写是子类与父类的一种多态性表现。

![image-20221023182628985](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221023182628985.png)

## Java 多态

### 多态的优点

- 消除类型之间的耦合关系
-  可替换性
-  可扩充性
-  接口性
-  灵活性
-  简化性

### 多态存在的三个必要条件

- 继承
- 重写
- 父类引用指向子类对象：**Parent p = new Child();**

![image-20221023201722232](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221023201722232.png)

**多态的好处：可以使程序有良好的扩展，并可以对所有类的对象进行通用处理。**

```java
public class Test {
    public static void main(String[] args) {
      show(new Cat());  // 以 Cat 对象调用 show 方法
      show(new Dog());  // 以 Dog 对象调用 show 方法
                
      Animal a = new Cat();  // 向上转型  
      a.eat();               // 调用的是 Cat 的 eat
      Cat c = (Cat)a;        // 向下转型  
      c.work();        // 调用的是 Cat 的 work
  }  
            
    public static void show(Animal a)  {
      a.eat();  
        // 类型判断
        if (a instanceof Cat)  {  // 猫做的事情 
            Cat c = (Cat)a;  
            c.work();  
        } else if (a instanceof Dog) { // 狗做的事情 
            Dog c = (Dog)a;  
            c.work();  
        }  
    }  
}
 
abstract class Animal {  
    abstract void eat();  
}  
  
class Cat extends Animal {  
    public void eat() {  
        System.out.println("吃鱼");  
    }  
    public void work() {  
        System.out.println("抓老鼠");  
    }  
}  
  
class Dog extends Animal {  
    public void eat() {  
        System.out.println("吃骨头");  
    }  
    public void work() {  
        System.out.println("看家");  
    }  
}
```

执行以上程序，输出结果为：

```
吃鱼
抓老鼠
吃骨头
看家
吃鱼
抓老鼠
```

### 重写

```java
/* 文件名 : Employee.java */
public class Employee {
   private String name;
   private String address;
   private int number;
   public Employee(String name, String address, int number) {
      System.out.println("Employee 构造函数");
      this.name = name;
      this.address = address;
      this.number = number;
   }
   public void mailCheck() {
      System.out.println("邮寄支票给： " + this.name
       + " " + this.address);
   }
   public String toString() {
      return name + " " + address + " " + number;
   }
   public String getName() {
      return name;
   }
   public String getAddress() {
      return address;
   }
   public void setAddress(String newAddress) {
      address = newAddress;
   }
   public int getNumber() {
     return number;
   }
}
/* 文件名 : Salary.java */
public class Salary extends Employee
{
   private double salary; // 全年工资
   public Salary(String name, String address, int number, double salary) {
       super(name, address, number);
       setSalary(salary);
   }
   public void mailCheck() {
       System.out.println("Salary 类的 mailCheck 方法 ");
       System.out.println("邮寄支票给：" + getName()
       + " ，工资为：" + salary);
   }
   public double getSalary() {
       return salary;
   }
   public void setSalary(double newSalary) {
       if(newSalary >= 0.0) {
          salary = newSalary;
       }
   }
   public double computePay() {
      System.out.println("计算工资，付给：" + getName());
      return salary/52;
   }
}
/* 文件名 : VirtualDemo.java */
public class VirtualDemo {
   public static void main(String [] args) {
      Salary s = new Salary("员工 A", "北京", 3, 3600.00);
      Employee e = new Salary("员工 B", "上海", 2, 2400.00);
      System.out.println("使用 Salary 的引用调用 mailCheck -- ");
      s.mailCheck();
      System.out.println("\n使用 Employee 的引用调用 mailCheck--");
      e.mailCheck();
    }
}
```

以上实例编译运行结果如下：

```
Employee 构造函数
Employee 构造函数
使用 Salary 的引用调用 mailCheck -- 
Salary 类的 mailCheck 方法 
邮寄支票给：员工 A ，工资为：3600.0

使用 Employee 的引用调用 mailCheck--
Salary 类的 mailCheck 方法 
邮寄支票给：员工 B ，工资为：2400.0
```

### 多态的实现方式

 方式一：重写：

 方式二：接口

 方式三：抽象类和抽象方法

**对于多态，可以总结以下几点：**

一、使用父类类型的引用指向子类的对象；

二、该引用只能调用父类中定义的方法和变量；

三、如果子类中重写了父类中的一个方法，那么在调用这个方法的时候，将会调用子类中的这个方法；（动态连接、动态调用）;

四、变量不能被重写（覆盖），”重写“的概念只针对方法，如果在子类中”重写“了父类中的变量，那么在编译时会报错。

### JAVA – 虚函数、抽象函数、抽象类、接口

**1. Java 虚函数**

虚函数的存在是为了多态。

C++ 中普通成员函数加上 virtual 关键字就成为虚函数。

Java 中其实没有虚函数的概念，它的普通函数就相当于 C++ 的虚函数，动态绑定是 Java 的默认行为。如果 Java 中不希望某个函数具有虚函数特性，可以加上 final 关键字变成非虚函数。

PS: 其实 C++ 和 Java 在虚函数的观点大同小异，异曲同工罢了。

**Java抽象函数(纯虚函数)**

抽象函数或者说是纯虚函数的存在是为了定义接口。

C++ 中纯虚函数形式为：

```
virtual void print() = 0;
```

Java 中纯虚函数形式为：

```
abstract void print();
```

**PS:** 在抽象函数方面 C++ 和 Java 还是换汤不换药。

**3. Java 抽象类**

抽象类的存在是因为父类中既包括子类共性函数的具体定义，也包括需要子类各自实现的函数接口。抽象类中可以有数据成员和非抽象方法。

C++ 中抽象类只需要包括纯虚函数，既是一个抽象类。如果仅仅包括虚函数，不能定义为抽象类，因为类中其实没有抽象的概念。

Java 抽象类是用 abstract 修饰声明的类。

**PS:** 抽象类其实是一个半虚半实的东西，可以全部为虚，这时候变成接口。

**4. Java 接口**

接口的存在是为了形成一种规约。接口中不能有普通成员变量，也不能具有非纯虚函数。

C++ 中接口其实就是全虚基类。

Java 中接口是用 interface 修饰的类。

**PS:** 接口就是虚到极点的抽象类。

**5. 小结**

```
C++ 虚函数    ==  Java 普通函数

C++ 纯虚函数  ==  Java 抽象函数

C++ 抽象类    ==  Java 抽象类

C++ 虚基类    ==  Java 接口
```

**静态方法被子类重写，要看父类引用还是子类引用指向子类对象，如果是父类引用指向子类对象，那么调静态方法时，会调用父类的静态方法，如果是子类引用指向子类对象，那么调静态方法时，会调用子类的静态方法。**

```java
public class Test {
    public static void main(String[] args) {
        System.out.println("Polymorghpic Test");
        Animal a = new Dog();
        Animal b = new Dog("smith", 5);
        Dog c = new Dog();
        // 子类的非静态方法与变量覆盖父类
        a.getInfo();
        // 子类通过super调用父类的非静态成员方法和变量
        a.getSuperInfo();
        // 子类的静态方法被父类隐藏
        a.hello();
        b.getInfo();
        b.hello();
        // 调用子类的静态方法
        c.hello();
    }
}

class Animal {
    String name;
    int age;
    public Animal() {
        name = "alex";
        age = 1;
    }

    public Animal(String nm, int ag) {
        name = nm;
        age = ag;
    }

    public void getInfo() {
        System.out.print(name + "'s age is " + age + '\n');
    }

    public void getSuperInfo() {}
    public static void hello() {
        System.out.println("Greeting from Animal");
    }
}

class Dog extends Animal {
    String name;
    int age;
    public Dog() {
        name = "prter";
        age = 3;
    }

    public Dog(String nm, int ag) {
        super(nm, ag);
    }

    public void getInfo() {
        System.out.print(name + "'s age is " + age + '\n');
    }

    // 通过super显式调用父类的非静态成员方法
    public void getSuperInfo() {
        super.getInfo();
    }

    public static void hello() {
        System.out.println("Greeting from Dog");
    }
}
```

输出结果：

```
Polymorghpic Test
prter's age is 3
alex's age is 1
Greeting from Animal
null's age is 0
Greeting from Animal
Greeting from Dog
```

### 抽象类总结规定

- 抽象类不能被实例化(初学者很容易犯的错)，如果被实例化，就会报错，编译无法通过。只有抽象类的非抽象子类可以创建对象。
- 抽象类中不一定包含抽象方法，但是有抽象方法的类必定是抽象类。
- 抽象类中的抽象方法只是声明，不包含方法体，就是不给出方法的具体实现也就是方法的具体功能。
- 构造方法，类方法（用 static 修饰的方法）不能声明为抽象方法。
- 抽象类的子类必须给出抽象类中的抽象方法的具体实现，除非该子类也是抽象类。

### 抽象类和接口的区别

#### 1、语法层面上的区别

-  1）抽象类可以提供成员方法的实现细节，而接口中只能存在**public abstract** 方法；
-  2）抽象类中的成员变量可以是各种类型的，而接口中的成员变量只能是**public static final**类型的；
-  3）**接口中不能含有静态代码块以及静态方法，而抽象类可以有静态代码块和静态方法；**？
-  4）一个类只能继承一个抽象类，而一个类却可以实现多个接口。

#### 2、设计层面上的区别

**1**）抽象类是对一种事物的抽象，即对类抽象，而接口是对行为的抽象。抽象类是对整个类整体进行抽象，包括属性、行为，但是接口却是对类局部（行为）进行抽象。举个简单的例子，飞机和鸟是不同类的事物，但是它们都有一个共性，就是都会飞。那么在设计的时候，可以将飞机设计为一个类 Airplane，将鸟设计为一个类 Bird，但是不能将 **飞行** 这个特性也设计为类，因此它只是一个行为特性，并不是对一类事物的抽象描述。此时可以将 飞行 设计为一个接口Fly，包含方法fly( )，然后Airplane和Bird分别根据自己的需要实现Fly这个接口。然后至于有不同种类的飞机，比如战斗机、民用飞机等直接继承Airplane即可，对于鸟也是类似的，不同种类的鸟直接继承Bird类即可。从这里可以看出，继承是一个 "**是不是**"的关系，而 接口 实现则是 "**有没有**"的关系。如果一个类继承了某个抽象类，则子类必定是抽象类的种类，而接口实现则是有没有、具备不具备的关系，比如鸟是否能飞（或者是否具备飞行这个特点），能飞行则可以实现这个接口，不能飞行就不实现这个接口。

**2）**设计层面不同，抽象类作为很多子类的父类，它是一种模板式设计。而接口是一种行为规范，它是一种辐射式设计。什么是模板式设计？最简单例子，大家都用过 ppt 里面的模板，如果用模板 A 设计了 ppt B 和 ppt C，ppt B 和 ppt C 公共的部分就是模板 A 了，如果它们的公共部分需要改动，则只需要改动模板 A 就可以了，不需要重新对 ppt B 和 ppt C 进行改动。而辐射式设计，比如某个电梯都装了某种报警器，一旦要更新报警器，就必须全部更新。也就是说对于抽象类，如果需要添加新的方法，**可以直接在抽象类中添加具体的实现，子类可以不进行变更**；而对于接口则不行，**如果接口进行了变更，则所有实现这个接口的类都必须进行相应的改动。**

下面看一个网上流传最广泛的例子：门和警报的例子：门都有 **open()** 和 **close()** 两个动作，此时我们可以定义通过抽象类和接口来定义这个抽象概念：

```java
abstract class Door {
    public abstract void open();
    public abstract void close();
}
```

或者：

```java
interface Door {
    public abstract void open();
    public abstract void close();
}
```

但是现在如果我们需要门具有报警 的功能，那么该如何实现？下面提供两种思路：

1）将这三个功能都放在抽象类里面，但是这样一来所有继承于这个抽象类的子类都具备了报警功能，但是有的门并不一定具备报警功能；

2）将这三个功能都放在接口里面，需要用到报警功能的类就需要实现这个接口中的 open( ) 和 close( )，也许这个类根本就不具备 open( ) 和 close( ) 这两个功能，比如火灾报警器。

从这里可以看出， Door 的 open() 、close() 和 alarm() 根本就属于两个不同范畴内的行为，open() 和 close() 属于门本身固有的行为特性，而 alarm() 属于延伸的附加行为。因此最好的解决办法是单独将报警设计为一个接口，包含 alarm() 行为，Door 设计为单独的一个抽象类，包含 open 和 close 两种行为。再设计一个报警门继承 Door 类和实现 Alarm 接口。

```java
interface Alram {
    void alarm();
}
 
abstract class Door {
    void open();
    void close();
}
 
class AlarmDoor extends Door implements Alarm {
    void oepn() {
      //....
    }
    void close() {
      //....
    }
    void alarm() {
      //....
    }
}
```

## Java 封装

### 封装的优点

- 良好的封装能够减少耦合。
- 类内部的结构可以自由修改。
- 可以对成员变量进行更精确的控制。
- 隐藏信息，实现细节。

## Java 接口

接口（英文：Interface），在JAVA编程语言中是一个抽象类型，是抽象方法的集合，接口通常以interface来声明。一个类通过继承接口的方式，从而来继承接口的抽象方法。

接口并不是类，编写接口的方式和类很相似，但是它们属于不同的概念。类描述对象的属性和方法。接口则包含类要实现的方法。

除非实现接口的类是抽象类，否则该类要定义接口中的所有方法。

接口无法被实例化，但是可以被实现。一个实现接口的类，必须实现接口内所描述的所有方法，否则就必须声明为抽象类。另外，**在 Java 中，接口类型可用来声明一个变量，他们可以成为一个空指针，或是被绑定在一个以此接口实现的对象。**

### 接口与类相似点：

- 一个接口可以有多个方法。
- 接口文件保存在 .java 结尾的文件中，文件名使用接口名。
- 接口的字节码文件保存在 .class 结尾的文件中。
- 接口相应的字节码文件必须在与包名称相匹配的目录结构中。

### 接口与类的区别：

- 接口不能用于实例化对象。
- 接口没有构造方法。
- 接口中所有的方法必须是抽象方法，**Java 8 之后 接口中可以使用 default 关键字修饰的非抽象方法。**
- 接口不能包含成员变量，除了 static 和 final 变量。
- 接口不是被类继承了，而是要被类实现。
- 接口支持多继承。

### 接口特性

- 接口中每一个方法也是隐式抽象的,接口中的方法会被隐式的指定为 **public abstract**（只能是 public abstract，其他修饰符都会报错）。
- 接口中可以含有变量，但是接口中的变量会被隐式的指定为 **public static final** 变量（并且只能是 public，用 private 修饰会报编译错误）。
- 接口中的方法是不能在接口中实现的，只能由实现接口的类来实现接口中的方法。

### 抽象类和接口的区别

- 抽象类中的方法可以有方法体，就是能实现方法的具体功能，但是接口中的方法不行。
- 抽象类中的成员变量可以是各种类型的，而接口中的成员变量只能是 **public static final** 类型的。
- **接口中不能含有静态代码块以及静态方法(用 static 修饰的方法)，而抽象类是可以有静态代码块和静态方法。**
- 一个类只能继承一个抽象类，而一个类却可以实现多个接口。

> **注**：JDK 1.8 以后，接口里可以有静态方法和方法体了。
>
> **注**：JDK 1.8 以后，接口允许包含具体实现的方法，该方法称为"默认方法"，默认方法使用 default 关键字修饰。更多内容可参考 [Java 8 默认方法](https://www.runoob.com/java/java8-default-methods.html)。
>
> **注**：JDK 1.9 以后，允许将方法定义为 private，使得某些复用的代码不会把方法暴露出去。更多内容可参考 [Java 9 私有接口方法](https://www.runoob.com/java/java9-private-interface-methods.html)。

重写接口中声明的方法时，需要注意以下规则：

- 类在实现接口的方法时，不能抛出强制性异常，只能在接口中，或者继承接口的抽象类中抛出该强制性异常。
- 类在重写方法时要保持一致的方法名，并且应该保持相同或者相兼容的返回值类型。
- 如果实现接口的类是抽象类，那么就没必要实现该接口的方法。

在实现接口的时候，也要注意一些规则：

- 一个类可以同时实现多个接口。
- 一个类只能继承一个类，但是能实现多个接口。
- 一个接口能继承另一个接口，这和类之间的继承比较相似。

### 接口的多继承

在Java中，类的多继承是不合法，但接口允许多继承。

在接口的多继承中extends关键字只需要使用一次，在其后跟着继承接口。 如下所示：

```java
public interface Hockey extends Sports, Event
```

以上的程序片段是合法定义的子接口，与类不同的是，接口允许多继承，而 Sports及 Event 可以定义或是继承相同的方法

### 标记接口

最常用的继承接口是没有包含任何方法的接口。

标记接口是没有任何方法和属性的接口.它仅仅表明它的类属于一个特定的类型,供其他代码来测试允许做一些事情。

标记接口作用：简单形象的说就是给某个对象打个标（盖个戳），使对象拥有某个或某些特权。

例如：java.awt.event 包中的 MouseListener 接口继承的 java.util.EventListener 接口定义如下：

```java
package java.util; 
public interface EventListener
{}
```

没有任何方法的接口被称为标记接口。标记接口主要用于以下两种目的：

- 建立一个公共的父接口：

  正如EventListener接口，这是由几十个其他接口扩展的Java API，**你可以使用一个标记接口来建立一组接口的父接口。**例如：当一个接口继承了EventListener接口，Java虚拟机(JVM)就知道该接口将要被用于一个事件的代理方案。

- 向一个类添加数据类型：

  这种情况是标记接口最初的目的，实现标记接口的类不需要定义任何接口方法(因为标记接口根本就没有方法)，**但是该类通过多态性变成一个接口类型。**

**注：在 JDK1.8，允许我们给接口添加两种非抽象的方法实现：**

1、默认方法，添加 **default** 修饰即可；

2、静态方法，使用 static 修饰；示例如下：

```java
interface Test{
    //这个是默认方法
    default String get(String aa){
        System.out.println("我是jdk1.8默认实现方法...");
        return "";
    }   
    //这个是静态方法    
    static void staticmethod(){
        System.out.println("我是静态方法");
    }
}
```

调用得话，静态方法只能通过接口名调用，不可以通过实现类的类名或者实现类的对象调用，default 方法只能通过接口实现类的对象来调用。

接口的默认方法、静态方法实现的一点补充。

```java
//定义一个接口

public interface Inter {

    void show(); //抽象方法   

    default void method() { //默认方法
        System.out.println("默认方法被实现了");    }

    static void test(){ //静态方法
        System.out.println("静态方法被实现了");    }
}

//定义接口的一个实现类

public class Interlmpl implements Inter {
    @Override    
    public void show() {
        System.out.println("show方法");    }
}

//定义测试类

public class InterDemo {
  public static void main(String[] args) {
    Inter i = new Interlmpl();        
    i.show();        //抽象方法强制被重写
    i.method();      //默认方法不强制被重写，但可以被重写，重写时去掉default关键字        
    Inter.test();   //静态方法只能通过接口名调用,不能通过实现类名或者对象名调用
  }
}
```

## Java 枚举(enum)

### values(), ordinal() 和 valueOf() 方法

enum 定义的枚举类默认继承了 java.lang.Enum 类，并实现了 java.lang.Serializable 和 java.lang.Comparable 两个接口。

values(), ordinal() 和 valueOf() 方法位于 java.lang.Enum 类中：

- values() 返回枚举类中所有的值。
- ordinal()方法可以找到每个枚举常量的索引，就像数组索引一样。
- valueOf()方法返回指定字符串值的枚举常量。

### 枚举类成员

枚举跟普通类一样可以用自己的变量、方法和构造函数，构造函数只能使用 private 访问修饰符，所以外部无法调用。

枚举既可以包含具体方法，也可以包含抽象方法。 如果枚举类具有抽象方法，则枚举类的每个实例都必须实现它。

### 实例

```java
enum Color
{
    RED, GREEN, BLUE;
 
    // 构造函数
    private Color()
    {
        System.out.println("Constructor called for : " + this.toString());
    }
 
    public void colorInfo()
    {
        System.out.println("Universal Color");
    }
}
 
public class Test
{    
    // 输出
    public static void main(String[] args)
    {
        Color c1 = Color.RED;
        System.out.println(c1);
        c1.colorInfo();
    }
}
```

执行以上代码输出结果为：

```java
Constructor called for : RED
Constructor called for : GREEN
Constructor called for : BLUE
RED
Universal Color
```

枚举类中的抽象方法实现，需要枚举类中的每个对象都对其进行实现。

```java
enum color{
    red{
        public void display() {
            System.out.println("red------"+"红色");
        }
    },
    blue{
        public void display() {
            System.out.println("blue------"+"蓝色");
        }
    },
    white{
        public void display() {
            System.out.println("white------"+"白色");
        }
    },
    blank
            {
                public void display() {
                    System.out.println("blank------"+"黑色");
                }
            },
    pink{
        public void display() {
            System.out.println("pink------"+"粉红色");
        }
    };
    private  color() {

        System.out.println("独属于color的构造方法！");
    }
    public void checkInfo() {
        System.out.println("该枚举类的类型为："+color.class);
    }
    public abstract void display();
}
public class Test {
    public static void main(String[] args) {

        color test=color.blue;
        test.checkInfo();
        test.display();
        System.out.println("遍历枚举的值");
        for (color temp : color.values()) {
            temp.display();
        }

    }
}
```

输出结果：

```java
独属于color的构造方法！
独属于color的构造方法！
独属于color的构造方法！
独属于color的构造方法！
独属于color的构造方法！
该枚举类的类型为：class color
blue------蓝色
遍历枚举的值
red------红色
blue------蓝色
white------白色
blank------黑色
pink------粉红色
```

## Java 包(package)

### 包的作用

- 1、把功能相似或相关的类或接口组织在同一个包中，方便类的查找和使用。
- 2、如同文件夹一样，包也采用了树形目录的存储方式。同一个包中的类名字是不同的，不同的包中的类的名字是可以相同的，当同时调用两个不同包中相同类名的类时，应该加上包名加以区别。因此，包可以避免名字冲突。
- 3、包也限定了访问权限，拥有包访问权限的类才能访问某个包中的类。

Java 使用包（package）这种机制是为了防止命名冲突，访问控制，提供搜索和定位类（class）、接口、枚举（enumerations）和注释（annotation）等。

### 创建包

创建包的时候，你需要为这个包取一个合适的名字。之后，如果其他的一个源文件包含了这个包提供的类、接口、枚举或者注释类型的时候，都必须将这个包的声明放在这个源文件的开头。

包声明应该在源文件的第一行，**每个源文件只能有一个包声明，这个文件中的每个类型都应用于它。**

如果一个源文件中没有使用包声明，那么其中的类，函数，枚举，注释等将被放在一个无名的包（unnamed package）中。

### import 关键字

**例子**

下面的 payroll 包已经包含了 Employee 类，接下来向 payroll 包中添加一个 Boss 类。Boss 类引用 Employee 类的时候可以不用使用 payroll 前缀，Boss 类的实例如下。

### Boss.java 文件代码：

```java
package payroll;
 
public class Boss
{
   public void payEmployee(Employee e)
   {
      e.mailCheck();
   }
}
```

如果 Boss 类不在 payroll 包中又会怎样？Boss 类必须使用下面几种方法之一来引用其他包中的类。

使用类全名描述，例如：

```
payroll.Employee
```

用 **import** 关键字引入，使用通配符 *****：

```
import payroll.*;
```

使用 **import** 关键字引入 Employee 类：

```
import payroll.Employee;
```

**注意：**

类文件中可以包含任意数量的 import 声明。import 声明必须在包声明之后，类声明之前。

**Java 中带包（创建及引用）的类的编译**

只有一个文件时编译：

```
javac A.java
```

一个包的文件都在时编译：

```
javac -d . *.java
```

**运行：**编译之后会自己生成文件夹，不要进入这个文件夹，直接运行 **java -cp /home/test test.Run**，其中源文件在 test 文件夹中，包名为 test，启动文件为 **Run.java**。

### package 的目录结构

类放在包中会有两种主要的结果：

- 包名成为类名的一部分，正如我们前面讨论的一样。
- 包名必须与相应的字节码所在的目录结构相吻合。

下面是管理你自己 java 中文件的一种简单方式：

将类、接口等类型的源码放在一个文本中，这个文件的名字就是这个类型的名字，并以.java作为扩展名。例如：

// 文件名 :  Car.java  package vehicle;  public class Car {   // 类实现   }

接下来，把源文件放在一个目录中，这个目录要对应类所在包的名字。

....\vehicle\Car.java

现在，正确的类名和路径将会是如下样子：

- 类名 -> vehicle.Car
- 路径名 -> vehicle\Car.java (在 windows 系统中)

通常，一个公司使用它互联网域名的颠倒形式来作为它的包名.例如：互联网域名是 runoob.com，所有的包名都以 com.runoob 开头。包名中的每一个部分对应一个子目录。

例如：有一个 **com.runoob.test** 的包，这个包包含一个叫做 Runoob.java 的源文件，那么相应的，应该有如下面的一连串子目录：

....\com\runoob\test\Runoob.java

编译的时候，编译器为包中定义的每个类、接口等类型各创建一个不同的输出文件，输出文件的名字就是这个类型的名字，并加上 .class 作为扩展后缀。 例如：

// 文件名: Runoob.java  package com.runoob.test; public class Runoob {       } class Google {       }

现在，我们用-d选项来编译这个文件，如下：

```
$javac -d . Runoob.java
```

这样会像下面这样放置编译了的文件：

```
.\com\runoob\test\Runoob.class
.\com\runoob\test\Google.class
```

你可以像下面这样来导入所有 **\com\runoob\test\** 中定义的类、接口等：

```
import com.runoob.test.*;
```

编译之后的 .class 文件应该和 .java 源文件一样，它们放置的目录应该跟包的名字对应起来。但是，并不要求 .class 文件的路径跟相应的 .java 的路径一样。你可以分开来安排源码和类的目录。

```
<path-one>\sources\com\runoob\test\Runoob.java
<path-two>\classes\com\runoob\test\Google.class
```

这样，你可以将你的类目录分享给其他的编程人员，而不用透露自己的源码。用这种方法管理源码和类文件可以让编译器和java 虚拟机（JVM）可以找到你程序中使用的所有类型。

类目录的绝对路径叫做 **class path**。设置在系统变量 **CLASSPATH** 中。编译器和 java 虚拟机通过将 package 名字加到 class path 后来构造 .class 文件的路径。

<path- two>\classes 是 class path，package 名字是 com.runoob.test,而编译器和 JVM 会在 <path-two>\classes\com\runoob\test 中找 .class 文件。

一个 class path 可能会包含好几个路径，多路径应该用分隔符分开。默认情况下，编译器和 JVM 查找当前目录。JAR 文件按包含 Java 平台相关的类，所以他们的目录默认放在了 class path 中。