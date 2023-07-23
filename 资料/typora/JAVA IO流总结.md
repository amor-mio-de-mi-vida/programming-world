# JAVA I/O流总结

## I/O流总览

![image-20221029180748286](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221029180748286.png)

![image-20221029180837627](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221029180837627.png)

### **IO流分类：**

按照“流”的数据流向，可以将其化分为：**输入流**和**输出流**。

按照“流”中处理数据的单位，可以将其区分为：**字节流**和**字符流**。在java中，字节是占1个Byte，即8位；而字符是占2个Byte，即16位。而且，需要注意的是，java的字节是有符号类型，而字符是无符号类型！



字节流的抽象基类：

　　InputStream，OutputStream

字符流的抽象基类：

　　Reader，Writer

由这四个类派生出来的子类名称都是以其父类名作为子类名的后缀，如InputStream的子类FileInputStream，Reader的子类FileReader。

## 文本读写的例子：

### 1.FileInputStream、FileOutputStream（字节流）**

字节流的方式效率较低，不建议使用

```java
public class IOTest {
	public static void main(String[] args) throws IOException {
		File file = new File("D:/test.txt");

		write(file);
		System.out.println(read(file));
	}

	public static void write(File file) throws IOException {
		OutputStream os = new FileOutputStream(file, true);

		// 要写入的字符串
		String string = "松下问童子，言师采药去。只在此山中，云深不知处。";
		// 写入文件
		os.write(string.getBytes());
		// 关闭流
		os.close();
	}

	public static String read(File file) throws IOException {
		InputStream in = new FileInputStream(file);

		// 一次性取多少个字节
		byte[] bytes = new byte[1024];
		// 用来接收读取的字节数组
		StringBuilder sb = new StringBuilder();
		// 读取到的字节数组长度，为-1时表示没有数据
		int length = 0;
		// 循环取数据
		while ((length = in.read(bytes)) != -1) {
			// 将读取的内容转换成字符串
			sb.append(new String(bytes, 0, length));
		}
		// 关闭流
		in.close();

		return sb.toString();
	}
}
```

### **2、BufferedInputStream、BufferedOutputStream（缓冲字节流）**

缓冲字节流是为高效率而设计的，真正的读写操作还是靠`FileOutputStream`和`FileInputStream`，所以其构造方法入参是这两个类的对象也就不奇怪了。

```java
public class IOTest {

	public static void write(File file) throws IOException {
		// 缓冲字节流，提高了效率
		BufferedOutputStream bis = new BufferedOutputStream(new FileOutputStream(file, true));

		// 要写入的字符串
		String string = "松下问童子，言师采药去。只在此山中，云深不知处。";
		// 写入文件
		bis.write(string.getBytes());
		// 关闭流
		bis.close();
	}

	public static String read(File file) throws IOException {
		BufferedInputStream fis = new BufferedInputStream(new FileInputStream(file));

		// 一次性取多少个字节
		byte[] bytes = new byte[1024];
		// 用来接收读取的字节数组
		StringBuilder sb = new StringBuilder();
		// 读取到的字节数组长度，为-1时表示没有数据
		int length = 0;
		// 循环取数据
		while ((length = fis.read(bytes)) != -1) {
			// 将读取的内容转换成字符串
			sb.append(new String(bytes, 0, length));
		}
		// 关闭流
		fis.close();

		return sb.toString();
	}
}
```

**3、InputStreamReader、OutputStreamWriter（字符流）**

> **字符流适用于文本文件的读写**，`OutputStreamWriter`类其实也是借助`FileOutputStream`类实现的，故其构造方法是`FileOutputStream`的对象

```java
public class IOTest {
	
	public static void write(File file) throws IOException {
		// OutputStreamWriter可以显示指定字符集，否则使用默认字符集
		OutputStreamWriter osw = new OutputStreamWriter(new FileOutputStream(file, true), "UTF-8");

		// 要写入的字符串
		String string = "松下问童子，言师采药去。只在此山中，云深不知处。";
		osw.write(string);
		osw.close();
	}

	public static String read(File file) throws IOException {
		InputStreamReader isr = new InputStreamReader(new FileInputStream(file), "UTF-8");
		// 字符数组：一次读取多少个字符
		char[] chars = new char[1024];
		// 每次读取的字符数组先append到StringBuilder中
		StringBuilder sb = new StringBuilder();
		// 读取到的字符数组长度，为-1时表示没有数据
		int length;
		// 循环取数据
		while ((length = isr.read(chars)) != -1) {
			// 将读取的内容转换成字符串
			sb.append(chars, 0, length);
		}
		// 关闭流
		isr.close();

		return sb.toString()
	}
}
```

**4、字符流便捷类**

> Java提供了`FileWriter`和`FileReader`简化字符流的读写，`new FileWriter`等同于`new OutputStreamWriter(new FileOutputStream(file, true))`

```java
public class IOTest {
	
	public static void write(File file) throws IOException {
		FileWriter fw = new FileWriter(file, true);

		// 要写入的字符串
		String string = "松下问童子，言师采药去。只在此山中，云深不知处。";
		fw.write(string);
		fw.close();
	}

	public static String read(File file) throws IOException {
		FileReader fr = new FileReader(file);
		// 一次性取多少个字节
		char[] chars = new char[1024];
		// 用来接收读取的字节数组
		StringBuilder sb = new StringBuilder();
		// 读取到的字节数组长度，为-1时表示没有数据
		int length;
		// 循环取数据
		while ((length = fr.read(chars)) != -1) {
			// 将读取的内容转换成字符串
			sb.append(chars, 0, length);
		}
		// 关闭流
		fr.close();

		return sb.toString();
	}
}
```

**5、BufferedReader、BufferedWriter（字符缓冲流）**

```java
public class IOTest {
	
	public static void write(File file) throws IOException {
		// BufferedWriter fw = new BufferedWriter(new OutputStreamWriter(new
		// FileOutputStream(file, true), "UTF-8"));
		// FileWriter可以大幅度简化代码
		BufferedWriter bw = new BufferedWriter(new FileWriter(file, true));

		// 要写入的字符串
		String string = "松下问童子，言师采药去。只在此山中，云深不知处。";
		bw.write(string);
		bw.close();
	}

	public static String read(File file) throws IOException {
		BufferedReader br = new BufferedReader(new FileReader(file));
		// 用来接收读取的字节数组
		StringBuilder sb = new StringBuilder();

		// 按行读数据
		String line;
		// 循环取数据
		while ((line = br.readLine()) != null) {
			// 将读取的内容转换成字符串
			sb.append(line);
		}
		// 关闭流
		br.close();

		return sb.toString();
	}
}
```

## 2.1 File类

File类是用来操作文件的类，但它不能操作文件中的数据。

```java
public class File extends Object implements Serializable, Comparable<File>
```


​		File类实现了Serializable、 Comparable<File>，说明它是支持序列化和排序的。

**File类的构造方法**

| 方法                              | 说明                                                         |
| --------------------------------- | ------------------------------------------------------------ |
| File(File parent, String child)   | 根据 parent 抽象路径名和 child 路径名字符串创建一个新 File 实例。 |
| File(String pathname)             | 通过将给定路径名字符串转换为抽象路径名来创建一个新 File 实例。 |
| File(String parent, String child) | 根据 parent 路径名字符串和 child 路径名字符串创建一个新 File 实例。 |
| File(URI uri)                     | 通过将给定的 file: URI 转换为一个抽象路径名来创建一个新的 File 实例。 |

**File类的常用方法**

| 方法              | 说明                                                         |
| ----------------- | ------------------------------------------------------------ |
| createNewFile()   | 当且仅当不存在具有此抽象路径名指定名称的文件时，不可分地创建一个新的空文件。 |
| delete()          | 删除此抽象路径名表示的文件或目录。                           |
| exists()          | 测试此抽象路径名表示的文件或目录是否存在。                   |
| getAbsoluteFile() | 返回此抽象路径名的绝对路径名形式。                           |
| getAbsolutePath() | 返回此抽象路径名的绝对路径名字符串。                         |
| length()          | 返回由此抽象路径名表示的文件的长度。                         |
| mkdir()           | 创建此抽象路径名指定的目录。                                 |

**File类使用实例**

```java
public class FileTest {
	public static void main(String[] args) throws IOException {
		File file = new File("C:/Mu/fileTest.txt");

		// 判断文件是否存在
		if (!file.exists()) {
			// 不存在则创建
			file.createNewFile();
		}
		System.out.println("文件的绝对路径：" + file.getAbsolutePath());
		System.out.println("文件的大小：" + file.length());

		// 刪除文件
		file.delete();
	}
}
```

## 2.2 字节流

`InputStream`与`OutputStream`是两个抽象类，是字节流的基类，所有具体的字节流实现类都是分别继承了这两个类。

以`InputStream`为例，它继承了`Object`，实现了`Closeable`

```java
public abstract class InputStream
extends Object
implements Closeable
```

`InputStream`类有很多的实现子类，下面列举了一些比较常用的：

![image-20221030163108534](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221030163108534.png)

详细说明一下上图中的类：

- **InputStream**：InputStream是所有字节输入流的抽象基类，前面说过抽象类不能被实例化，实际上是作为模板而存在的，为所有实现类定义了处理输入流的方法。
- **FileInputSream**：文件输入流，一个非常重要的字节输入流，用于对文件进行读取操作。
- **PipedInputStream**：管道字节输入流，能实现多线程间的管道通信。
- **ByteArrayInputStream**：字节数组输入流，从字节数组(byte[])中进行以字节为单位的读取，也就是将资源文件都以字节的形式存入到该类中的字节数组中去。
- **FilterInputStream**：装饰者类，具体的装饰者继承该类，这些类都是处理类，作用是对节点类进行封装，实现一些特殊功能。
- **DataInputStream**：数据输入流，它是用来装饰其它输入流，作用是“允许应用程序以与机器无关方式从底层输入流中读取基本 Java 数据类型”。
- **BufferedInputStream**：缓冲流，对节点流进行装饰，内部会有一个缓存区，用来存放字节，每次都是将缓存区存满然后发送，而不是一个字节或两个字节这样发送，效率更高。
- **ObjectInputStream**：对象输入流，用来提供对基本数据或对象的持久存储。通俗点说，也就是能直接传输对象，通常应用在反序列化中。它也是一种处理流，构造器的入参是一个InputStream的实例对象。

`OutputStream`类继承关系图：

![image-20221030163243098](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221030163243098.png)

`OutputStream`类继承关系与`InputStream`类似，需要注意的是`PrintStream`.

## 2.3 字符流

与字节流类似，字符流也有两个抽象基类，分别是`Reader`和`Writer`。其他的字符流实现类都是继承了这两个类。

以`Reader`为例，它的主要实现子类如下图：

![image-20221030163324470](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20221030163324470.png)

各个类的详细说明：

- InputStreamReader：从字节流到字符流的桥梁（InputStreamReader构造器入参是FileInputStream的实例对象），它读取字节并使用指定的字符集将其解码为字符。它使用的字符集可以通过名称指定，也可以显式给定，或者可以接受平台的默认字符集。
- BufferedReader：从字符输入流中读取文本，设置一个缓冲区来提高效率。BufferedReader是对InputStreamReader的封装，前者构造器的入参就是后者的一个实例对象。
- FileReader：用于读取字符文件的便利类，new FileReader(File file)等同于new InputStreamReader(new FileInputStream(file, true),"UTF-8")，但FileReader不能指定字符编码和默认字节缓冲区大小。
- PipedReader ：管道字符输入流。实现多线程间的管道通信。
- CharArrayReader：从Char数组中读取数据的介质流。
- StringReader ：从String中读取数据的介质流。
- Writer与Reader结构类似，方向相反，不再赘述。唯一有区别的是，Writer的子类PrintWriter。

## 3.1 字节流方法

字节输入流InputStream主要方法：

- read() ：从此输入流中读取一个数据字节。
- read(byte[] b) ：从此输入流中将最多 b.length 个字节的数据读入一个 byte 数组中。
- read(byte[] b, int off, int len) ：从此输入流中将最多 len 个字节的数据读入一个 byte 数组中。
- close()：关闭此输入流并释放与该流关联的所有系统资源。

字节输出流OutputStream主要方法：

- write(byte[] b) ：将 b.length 个字节从指定 byte 数组写入此文件输出流中。
- write(byte[] b, int off, int len) ：将指定 byte 数组中从偏移量 off 开始的 len 个字节写入此文件输出流。
- write(int b) ：将指定字节写入此文件输出流。
- close() ：关闭此输入流并释放与该流关联的所有系统资源。

3.2 字符流方法
字符输入流Reader主要方法：

- read()：读取单个字符。

- read(char[] cbuf) ：将字符读入数组。

- read(char[] cbuf, int off, int len) ： 将字符读入数组的某一部分。

- read(CharBuffer target) ：试图将字符读入指定的字符缓冲区。

- flush() ：刷新该流的缓冲。

- close() ：关闭此流，但要先刷新它。

  

字符输出流Writer主要方法：

- write(char[] cbuf) ：写入字符数组。
- write(char[] cbuf, int off, int len) ：写入字符数组的某一部分。
- write(int c) ：写入单个字符。
- write(String str) ：写入字符串。
- write(String str, int off, int len) ：写入字符串的某一部分。
- flush() ：刷新该流的缓冲。
- close() ：关闭此流，但要先刷新它。

另外，字符缓冲流还有两个独特的方法：

- `BufferedWriter`类`newLine()` ：**写入一个行分隔符。这个方法会自动适配所在系统的行分隔符。**

- `BufferedReader`类`readLine()` ：读取一个文本行。

  

## 位、字节、字符

字节(Byte)是计量单位，表示数据量多少，是计算机信息技术用于计量存储容量的一种计量单位，通常情况下一字节等于八位。

字符(Character)计算机中使用的字母、数字、字和符号，比如’A’、‘B’、’$’、’&'等。

一般在英文状态下一个字母或字符占用一个字节，一个汉字用两个字节表示。

**字节与字符：**

- ASCII 码中，一个英文字母（不分大小写）为一个字节，一个中文汉字为两个字节。
- UTF-8 编码中，一个英文字为一个字节，一个中文为三个字节。
- Unicode 编码中，一个英文为一个字节，一个中文为两个字节。
- 符号：英文标点为一个字节，中文标点为两个字节。例如：英文句号 . 占1个字节的大小，中文句号 。占2个字节的大小。
- UTF-16 编码中，一个英文字母字符或一个汉字字符存储都需要 2 个字节（Unicode 扩展区的一些汉字存储需要 4 个字节）。
- UTF-32 编码中，世界上任何字符的存储都需要 4 个字节。
