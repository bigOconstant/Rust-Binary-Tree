use std::ptr;
pub struct Node {
    left:*mut Node,
    right:*mut Node,
    data:i32,
    parent: *mut Node,
}

impl Node {
    fn new(d:i32) -> Node{
        Node {
            left:ptr::null_mut(),
            right:ptr::null_mut(),
            data:d,
            parent:ptr::null_mut()
        }
    }

         fn insert(&mut self, n:i32) {
             unsafe {
             if n < self.data{
                 if self.left.is_null(){
                     let mut newnode = Node::new(n);
                     self.left = Box::into_raw(Box::new(Node::new(n)));
                     (*self.left).parent = self;
                 }else{
                     (*self.left).insert(n);
                 }
                
             }
            else {
                if self.right.is_null(){
                    let mut newnode = Node::new(n);
                    self.right = Box::into_raw(Box::new(Node::new(n)));
                    (*self.right).parent = self;
                } else{
                    (*self.right).insert(n);
                }
            }

             }
         }

    fn print(&mut self){
        println!("value:{}",self.data)
    }
    fn print_left_child(&mut self){
        unsafe {
            if !self.left.is_null(){
                (*self.left).print();
            }

        }
    }
    fn print_right_child(&mut self){
        unsafe {
            if !self.right.is_null(){
                (*self.right).print();
            }
        }
    }   
}

fn main() {
    println!("unsafe rust!");
    let mut root = Node::new(5);
    root.insert(4);
    root.insert(6);
    root.insert(2);
    unsafe {
      println!("right branch = {}",(*root.right).data);
      println!("left branch = {}",(*root.left).data);
      println!("left->left = {}",(*(*root.left).left).data);
      println!("parent value = {}",(*(*root.left).parent).data);
        
    }
    root.print_left_child();
    root.print_right_child();
}

