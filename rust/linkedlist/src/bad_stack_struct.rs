use std::fmt::Display;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new(items: Vec<i32>) -> Self {
        let mut end: List = List { head: Link::Empty };

        for i in items.iter().rev() {
            end.push(*i);
        }

        end
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            // author admits this `mem` truck is hacky
            next: std::mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(new_node.into())
    }

    pub fn pop(&mut self) -> Option<i32> {
        // author admits this `mem` truck is hacky
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let val = node.elem;
                self.head = node.next;
                Some(val)
            }
        }
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "END"),
            Self::More(node) => {
                let _ignored = write!(f, "{} -> ", node.elem);
                write!(f, "{}", node.next)
            }
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.head)
    }
}
//
// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {} // nothing to do
//             Link::More(ref mut boxed_node) => {
//                 drop(boxed_node);
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut list = List::new(vec![]);
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
