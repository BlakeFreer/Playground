// https://rust-unofficial.github.io/too-many-lists/fifth.html

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::new(Node { elem, next: None });

        let old_tail = std::mem::replace(&mut self.tail, Some(new_tail));

        match old_tail {
            Some(mut old_tail) => {
                old_tail.next = Some(new_tail);
            }
            None => self.head = Some(new_tail),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;
            old_head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
    }
}
