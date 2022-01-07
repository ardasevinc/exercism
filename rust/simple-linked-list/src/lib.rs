use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    el: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }
    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut curr = self.head.as_ref();

        let mut count: usize = 0;
        while let Some(boxed_node) = curr {
            count += 1;
            curr = boxed_node.next.as_ref();
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            el: element,
            next: self.head.take()
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.el
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.el)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let self_vec: Vec<_> = self.into();
        self_vec.into_iter().rev().collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut prev = None;

        for i in iter {
            let new_node = Some( Box::new(Node {
                el: i,
                next: prev,
            }) );
            prev = new_node;
        }

        SimpleLinkedList {
            head: prev,
        }
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();

        while let Some(t) = self.pop() {
            v.push(t);
        }
        v.into_iter().rev().collect()
    }
}
