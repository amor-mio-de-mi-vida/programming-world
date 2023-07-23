#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(data:T) -> Self {
        // 初始化时无下一链接
        Node { data: data, next: None }
    }
}

// 链表栈
#[derive(Debug)]
struct Stack<T> {
    size: usize,
    top: Link<T>,
}

impl<T: Clone> Stack<T> {
    fn new() -> Self {
        Stack { size: 0, top: None}
    }

    // take取出top中节点，留下空位，所以可以回填节点
    fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    // as_ref 将 top 节点转为引用对象
    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| {
            &node.data
        })
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }
}

