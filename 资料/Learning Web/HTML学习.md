# HTML学习

[官方文档](https://html.spec.whatwg.org/multipage/)

这个主题包含以下模块，建已从第一个开始，按顺序进行学习。

- HTML介绍

  这一模块将为你铺路，帮你习惯一些重要的概念和语法，着眼于如何对文本应用HTML，创造超链接，以及使用HTML构造一个网页。

- 多媒体和嵌入

  这个模块带你探索如何使用HTML在你的网页里如何包含多媒体信息，例如用各种方法包含图片，以及嵌入视频，音频，甚至是嵌入其他完整的网页。

- HTML表格

  在网页上用易于理解和无障碍的方式来表示表格数据是一项挑战。这个模块包括了基本的表格标记及更多复杂的特性，例如实现标题和总结。

# HTML介绍

## 开始学习HTML

**块级元素和内联元素**

- 块级元素在页面中以块的形式展现。一个块级元素出现在它前面的内容之后的新行上。任何跟在块级元素后面的内容也会出现在新的行上。块级元素通常是页面上的结构元素。例如，一个块级元素可能代表标题、段落、列表、导航菜单或页脚。一个块级元素不会嵌套在一个内联元素里面，但它可能嵌套在另一个块级元素里面。
- 内联元素通常出现在块级元素中并环绕文档内容的一小部分，而不是一整个段落或者一组内容。内联元素不会导致文本换行。它通常与文本一起使用，例如，[``](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/a) 元素创建一个超链接，[``](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/em) 和 [``](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/strong) 等元素创建强调。

**空元素**

不是所有元素都拥有开始标签、内容和结束标签。一些元素只有一个标签，通常用来在此元素所在位置插入/嵌入一些东西。这些元素被称为[空元素](https://developer.mozilla.org/zh-CN/docs/Glossary/Void_element)。例如：元素 [``](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/img) 是用来在页面插入一张指定的图片。

```HTML
<img src="https://roy-tian.github.io/learning-area/extras/getting-started-web/beginner-html-site/images/firefox-icon.png" alt="Firefox 图标">
```

**元素的属性**

另一个例子是关于元素 [``](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/a) 的——元素 [``](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/a) 是*锚*，它使被标签包裹的内容成为一个超链接。锚元素可以添加多种属性，部分如下：

- `href`

  这个属性声明超链接的 web 地址。例如 `href="https://www.mozilla.org/"`。

- `title`

  `title` 属性为超链接声明额外的信息，比如你将链接至的那个页面。例如 `title="The Mozilla homepage"`。当鼠标悬停在超链接上面时，这部分信息将以工具提示的形式显示。

- `target`

  `target` 属性用于指定链接如何呈现出来。例如，`target="_blank"` 将在新标签页中显示链接。如果你希望在当前标签页显示链接，忽略这个属性即可。

**布尔属性**

有时你会看到没有值的属性，这也是完全可以接受的。这些属性被称为布尔属性。布尔属性只能有一个值，这个值一般与属性名称相同。例如，考虑 `disabled` 属性，你可以将其分配给表单输入元素。用它来禁用表单输入元素，这样用户就不能输入了。被禁用的元素通常有一个灰色的外观。示例如下：

```
<input type="text" disabled="disabled" />
```

Copy to Clipboard

方便起见，我们完全可以将其写成以下形式：

```
<!-- 使用 disabled 属性来防止终端用户输入文本到输入框中 -->
<input type="text" disabled />

<!-- 下面这个输入框不包含 disabled 属性，所以用户可以向其中输入 -->
<input type="text" />
```

Copy to Clipboard

作为参考，上面的例子还包括一个非禁用的表单输入元素。上面两段 HTML 代码产生的效果如下：

剖析HTML文档

单独的 HTML 元素本身并不十分有用。接下来，我们来看看单个元素是如何组合成整个 HTML 页面的：

```html
<!DOCTYPE html>
<html lang="zh-CN">
  <head>
    <meta charset="utf-8" />
    <title>我的测试站点</title>
  </head>
  <body>
    <p>这是我的页面</p>
  </body>
</html>
```

这里我们有：

1. ```html
   <!DOCTYPE html>
   ```

   : 声明文档类型。早期的 HTML（大约 1991-1992 年）文档类型声明类似于链接，规定了 HTML 页面必须遵从的良好规则，能自动检测错误和其他有用的东西。文档类型使用类似于这样：

   ```html
   <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
   ```

   Copy to ClipboardCopy to Clipboard

   文档类型是一个历史遗留问题，需要包含它才能使其他东西正常工作。现在，只需要知道

   ```html
   <!DOCTYPE html>
   ```

   是最短的有效文档声明！

2. `<html></html>`: [`html`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/html) 元素。这个元素包裹了页面中所有的内容，有时被称为根元素。

3. `<head></head>`: [`head`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/head) 元素。这个元素是一个容器，它包含了所有你想包含在 HTML 页面中但**不在 HTML 页面中显示**的内容。这些内容包括你想在搜索结果中出现的关键字和页面描述、CSS 样式、字符集声明等等。以后的章节中会学到更多相关的内容。

4. `<meta charset="utf-8">`: [`meta`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/meta) 元素。这个元素代表了不能由其他 HTML 元相关元素表示的元数据，比如 [`base`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/base)、[`link`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/link)、[`script`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/script)、[`style`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/style) 或 [`title`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/title)。[`charset`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/meta#attr-charset) 属性将你的文档的字符集设置为 UTF-8，其中包括绝大多数人类书面语言的大多数字符。有了这个设置，页面现在可以处理它可能包含的任何文本内容。没有理由不对它进行设置，它可以帮助避免以后的一些问题。

5. `<title></title>`: [`title`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/title) 元素。这设置了页面的标题，也就是出现在该页面加载的浏览器标签中的内容。当页面被加入书签时，页面标题也被用来描述该页面。

6. `<body></body>`: [`body`](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/body) 元素。包含了你访问页面时*所有*显示在页面上的内容，包含文本、图片、视频、游戏、可播放音频轨道等等。
