type Wrapper<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Item<T> {
    elem: Wrapper<T>,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Node<T> {
    item: T,
    next: Wrapper<T>
}

#[derive(Debug)]
pub struct Bag<T> {
    first: Wrapper<T>,
    n: i32
}

impl<T> Bag<T> {
    pub fn get_empty_bag() -> Self {
        Bag{ first: None, n: 0 }
    }

    pub fn is_empty_bag(&self) -> bool{
        self.first.is_none()
    }

    pub fn size(&self) -> i32 {
        self.n
    }

    pub fn get_from_bag(&self) -> Option<&T> {
        self.first.as_ref().map(|node| {
            &node.item
        })
    }

    pub fn add(&mut self, item: T) {
        let new_node = Box::new(Node {
            item,
            next: self.first.take()
        });
        self.first = Some(new_node);
        self.n += 1;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.first.as_deref() }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.item
        })
    }
}

