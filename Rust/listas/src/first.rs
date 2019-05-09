use std::mem;


// Solo vamos a permitir los enteros de 32 bits
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    nect: Link,
}

/*
 fn foo(self, arg2: Type2) -> ReturnType {
    // body
}

donde:

self - valor
&mut self - referencia mutable
&self - referencia compartida

 */

impl List {
    pub fn new() -> Self { // Con Self nos ahorramos el volver a escribir tipos
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// Código para pruebas, favor de no mover :3

mod test {
    #[test]
    fn basics() {
        //TODO
    }
}
