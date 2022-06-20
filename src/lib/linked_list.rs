/*
A transaction log (implemented with a linked list) in Rust.
*/

//Tip: use type definitions when multiple generics are at play.
type SingleLink = Option<Rc<RefCell<Node>>>;
//using a RefCell to dynamically mutate a nested structure
//is known as interior mutability pattern

/* RefCell allows you to enforce Rust's ownership rules at __runtime__ time
instead of at __compile__ time.
*/

//////// NODE DEFINITION ////////

#[derive(Clone)] //for cloning the structure with .clone()
struct Node {
    value: String,
    //Option<Node> will not compile since the compiler cannot determine how much memory it needs.
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        //create a new node with internal value _value
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

//////// LIST DEFINITION ////////

struct TransactionLog {
    //these variables are available to you
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            head: None,
            length: 0,
        }
    }

    //Appends a new node at the end of the list
    pub fn append(&mut self, value: String) {
        let new = Node::new(value);

        //.take() "takes" the wrapped value leaving None behind.
        match self.tail.take() {
            //need to clone the pointers.
            Some(node) => node.borrow_mut().next = Some(new.clone()),
            None =>
            /*empty list */
            {
                self.head = Some(new.clone())
            }
        }

        self.length += 1;
        tail = Some(new);
    }

    // Removes the head (first) node of the list
    pub fn pop() {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut.next.take() {
                self.head = Some(next);
            } else {
                //we have cleared the entire list
                //but we still have a pointer in the tail instance variable.
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(head) //unwraps and consumes the refcell ptr
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .value
        })
    }
}
