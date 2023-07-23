# 对JAVA异常的理解

- 编译时异常

1. **受检**异常：CheckedException
2. **受控**异常

- 运行时异常

1. **未受检**异常：UnCheckedException
2. **非受控**异常

1、补充：

```java
public class ExceptionTest03 {
    public static void main(String[] args) {
        System.out.println(100 / 0);
        // 这里的HelloWorld没有输出，没有执行。
        System.out.println("Hello World!");
    }
}
```

 程序执行到System.out.println(100 / 0);
		此处发生了 ArithmeticException 异常，底层 new 了一个ArithmeticException异常对象，然后抛出了。
		由于是 main方法 调用了100 / 0，所以这个异常ArithmeticException抛给了main方法。
		main方法没有处理，将这个异常自动抛给了 JVM。JVM最终终止程序的执行。

此时System.out.println("Hello World!");并不会执行。

注意：ArithmeticException 继承 RuntimeException，属于 运行时异常。在编写程序阶段不需要对这种异常进行预先的处理。

eg：

```java
public class ExceptionTest04 {
    public static void main(String[] args) {
        // main方法中调用doSome()方法
        // 因为doSome()方法声明位置上有：throws ClassNotFoundException
        // 我们在调用doSome()方法的时候必须对这种异常进行预先的处理。
        // 如果不处理，编译器就报错。
        //编译器报错信息： Unhandled exception: java.lang.ClassNotFoundException
        doSome();
    }

    /**
     * doSome方法在方法声明的位置上使用了：throws ClassNotFoundException
     * 这个代码表示doSome()方法在执行过程中，有可能会出现ClassNotFoundException异常。
     * 叫做类没找到异常。这个异常直接父类是：Exception，所以ClassNotFoundException属于编译时异常。
     * @throws ClassNotFoundException
     */
    public static void doSome() throws ClassNotFoundException{
        System.out.println("doSome!!!!");
    }
}
```

解决方法一、throws上报给方法调用者（推卸责任：调用者知道）

```java
public class ExceptionTest04 {
    public static void main(String[] args) throws ClassNotFoundException {
        doSome();
    }
    public static void doSome() throws ClassNotFoundException{
        System.out.println("doSome!!!!");
    }
}
```

解决方法二、try…catch捕捉，处理（调用者是不知道）

```java
public class ExceptionTest04 {
    public static void main(String[] args) {
        try {
            doSome();
        } catch (ClassNotFoundException e) {
            e.printStackTrace();
        }
    }
    
    public static void doSome() throws ClassNotFoundException{
        System.out.println("doSome!!!!");
    }
}
```

## 异常的处理方式

-  **throws**

​	在方法声明的位置上使用 throws 关键字抛出，谁调用我这个方法，我就抛给谁。抛给调用者来处理。

​	这种处理异常的态度：上报。

- **try…catch**

  这个异常不会上报，自己把这个事儿处理了。

  异常抛到此处为止，不再上抛了。

**注意：**

 只要异常没有捕捉，采用上报的方式，此方法的 后续代码不会执行。

try语句块中的某一行出现异常，该行后面的代码不会执行。

try…catch捕捉异常之后，后续代码可以执行

eg：

```java
try {
    m1();
    // m1方法出异常，下面代码不执行。
    System.out.println("hello world!");//不执行
} catch (FileNotFoundException e){ 
	//异常处理
    System.out.println("出异常了！！");
    System.out.println(e); 
}
System.out.println("hello world"); //会执行
```

注意：

- 异常发生之后，如果我选择了上抛，抛给了我的调用者，调用者需要对这个异常继续处理，那么调用者处理这个异常同样有两种处理方式。
- 一般不建议在main方法上使用throws，因为这个异常如果真正的发生了，一定会抛给JVM。JVM只有终止。
- 一般main方法中的异常建议使用try…catch进行捕捉。

```java
try {
    
} catch (ClassNotFoundException e) {
    e.printStackTrace();
}
```

这个分支中可以使用e引用，**`e引用`** 保存的内存地址是那个**new**出来 **`异常对象的内存地址`**。

## 深入try…catch

1. catch后面的小括号中的类型可以是 **`具体的异常类型`**，也可以是该异常类型的 **`父类型`**。
2. catch可以写**多个**。建议catch的时候，**精确的一个一个处理**。这样有利于程序的调试。
3. catch写多个的时候，**从上到下**，必须遵守 **`从小到大`**。

eg

```java
try {
	FileInputStream fis = new FileInputStream("D:\\Download\\Javabean-addperson案例解析.docx");
} catch(FileNotFoundException e) {
	System.out.println("文件不存在！");
}

等同于

try {
	FileInputStream fis = new FileInputStream("D:\\Download\\Javabean-addperson案例解析.docx");
} catch(Exception e) {// 多态：Exception e = new FileNotFoundException();
	System.out.println("文件不存在！");
}
```

```java
try {
    FileInputStream fis = new FileInputStream("D:\\Download\\Javabean-addperson案例解析.docx");
    fis.read();
} catch(IOException e){
    System.out.println("读文件报错了！");
} catch(FileNotFoundException e) {
    System.out.println("文件不存在！");
}
```

JDK8的新特性：**catch()** 异常间可以**自小到大**用 **`|`** 分割

```java
try {
    //创建输入流
    FileInputStream fis = new FileInputStream("D:\\Download\\Javabean-addperson案例解析.docx");
    // 进行数学运算
    System.out.println(100 / 0); // 这个异常是运行时异常，编写程序时可以处理，也可以不处理。
} catch(FileNotFoundException | ArithmeticException | NullPointerException e) {
    System.out.println("文件不存在？数学异常？空指针异常？都有可能！");
}
```

## 异常两个重要方法

| 方法名                 | 作用                           |
| ---------------------- | ------------------------------ |
| String getMessage()    | 返回异常的详细消息字符串       |
| void printStackTrace() | 追踪堆栈异常信息(采用异步线程) |

## finally字句

在finally子句中的代码是最后执行的，并且是 **`一定会执行`** 的，即使try语句块中的代码出现了异常。

**finally**子句必须和**try**一起出现，不能单独编写。

通常在finally语句块中完成 **`资源的释放/关闭`**。

eg:

```java
public class ExceptionTest10 {
    public static void main(String[] args) {
        FileInputStream fis = null; // 声明位置放到try外面。这样在finally中才能用。
        try {
            fis = new FileInputStream("D:\\Download\\Javabean-addperson案例解析.docx");
            String s = null;
            // 这里一定会出现空指针异常！
            s.toString();
            System.out.println("hello world!");

            // 流使用完需要关闭，因为流是占用资源的。
            // 即使以上程序出现异常，流也必须要关闭！
            // 放在这里有可能流关不了。
            //fis.close();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch(IOException e){
            e.printStackTrace();
        } catch(NullPointerException e) {
            e.printStackTrace();
        } finally {
            System.out.println("hello 浩克！");
            // 流的关闭放在这里比较保险。
            // finally中的代码是一定会执行的。
            // 即使try中出现了异常！
            if (fis != null) { // 避免空指针异常！
                try {
                    // close()方法有异常，采用捕捉的方式。
                    fis.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
    }
}
```

**try和finally联用，没有catch**

```java
public class ExceptionTest11 {
    public static void main(String[] args) {
    	try {
            System.out.println("try...");
            return;
        } finally {
            System.out.println("finally...");
        }

        // 这里不能写语句，因为这个代码是无法执行到的。
        //System.out.println("Hello World!");
    }
}
```

**以下代码的执行顺序：**

1. 先执行try…
2. 再执行finally…
3. 最后执行 return （return语句只要执行方法必然结束。）

**注意：**

- try不能单独使用。
- try finally可以联合使用。
- 放在finally语句块中的代码是一定会执行的

**finally字句失效**

```java
public class ExceptionTest12 {
    public static void main(String[] args) {
        try {
            System.out.println("try...");
            // 退出JVM
            System.exit(0); // 退出JVM之后，finally语句中的代码就不执行了！
        } finally {
            System.out.println("finally...");
        }
    }
}
```

**final finally finalize有什么区别？**

final 关键字

- final修饰的类无法继承
- final修饰的方法无法覆盖
- final修饰的变量不能重新赋值。

finally 关键字

- finally 和try一起联合使用。

- finally语句块中的代码是必须执行的。

finalize 标识符

- 是一个Object类中的方法名。

- 这个方法是由垃圾回收器GC负责调用的

## 方法覆盖时遗留的问题

重写之后的方法不能比重写之前的方法抛出更多（更宽泛）的异常，可以更少。[[方法覆盖](https://blog.csdn.net/qq_44715943/article/details/115680718)](https://blog.csdn.net/qq_44715943/article/details/115680718)

```java
class Animal {
    public void doSome(){

    }

    public void doOther() throws Exception{

    }
}

class Cat extends Animal {

    // 编译正常。
    public void doSome() throws RuntimeException{

    }

    // 编译报错。
    /*public void doSome() throws Exception{

    }*/

    // 编译正常。
    /*public void doOther() {

    }*/

    // 编译正常。
    /*public void doOther() throws Exception{

    }*/

    // 编译正常。
    public void doOther() throws NullPointerException{

    }
}
```

**注意：**一般不会这样考虑，方法覆盖复制一份，然后重写就好了。

