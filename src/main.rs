use std::mem;
use std::ptr;
pub struct Node {
    left: *mut Node,
    right: *mut Node,
    data: i32,
    parent: *mut Node,
}

impl Node {
    fn new(d: i32) -> Node {
        Node {
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            data: d,
            parent: ptr::null_mut(),
        }
    }

    fn insert(&mut self, n: i32) {
        unsafe {
            if n < self.data {
                if self.left.is_null() {
                    self.left = Box::into_raw(Box::new(Node::new(n)));
                    (*self.left).parent = self;
                } else {
                    (*self.left).insert(n);
                }
            } else {
                if self.right.is_null() {
                    self.right = Box::into_raw(Box::new(Node::new(n)));
                    (*self.right).parent = self;
                } else {
                    (*self.right).insert(n);
                }
            }
        }
    }
    fn delete_left(&mut self) {
        if !self.left.is_null() {
            unsafe {
                if !(*self.right).left.is_null() {
                    (*self.right).delete_left();
                }
                if !(*self.right).right.is_null() {
                    (*self.right).delete_right();
                }
                let mut left_ptr = ptr::null_mut();

                mem::swap(&mut left_ptr, &mut self.left);
                let _ = Box::from_raw(left_ptr);
            }
        }
    }

    fn delete_right(&mut self) {
        if !self.right.is_null() {
            unsafe {
                if !(*self.right).right.is_null() {
                    (*self.right).delete_right();
                }
                if !(*self.right).left.is_null() {
                    (*self.right).delete_left();
                }

                let mut right_ptr = ptr::null_mut();

                mem::swap(&mut right_ptr, &mut self.right);
                let _ = Box::from_raw(right_ptr);
            }
        }
    }

    fn print_tree(&self) {
        println!("{}", self.data);
        unsafe {
            if !self.right.is_null() {
                println!("\\");
                (*self.right).print_tree();
            }
            if !self.left.is_null() {
                println!("/");
                (*self.left).print_tree();
            }
        }

    }

    fn get_depth(&self)->i32 {
        let depth = 0;
        fn get_depth_inner(n_d:&Node,depth:i32) ->i32{
            let mut leftdepth = 0;
            let mut rightdepth = 0;

            if n_d.right.is_null() { // right node is null
                if n_d.left.is_null(){ //left node is also null, return
                    return depth;
                }else { // left node isn't null. Recurse to the left
                    unsafe{
                        leftdepth = get_depth_inner(&*n_d.left,depth+1);
                    }
                }
            } else if  n_d.left.is_null() { //right node is not null left node is
               unsafe {
                   rightdepth = get_depth_inner(&*n_d.right,depth+1)
               }
                              
            } else {
                // Niether left or right is null. update both
                unsafe {
                    leftdepth = get_depth_inner(&*n_d.left,depth+1);
                    rightdepth = get_depth_inner(&*n_d.right,depth+1);
                }
            }


            
            let mut max = depth;
            if leftdepth > max {
                max = leftdepth;
            }
            if rightdepth > max {
                max = rightdepth;
            }
            return max;
            
        }
        return get_depth_inner(&self,depth);

        
    }

    
}

fn main() {
    println!("unsafe rust!");
    let mut root = Node::new(15);
    root.insert(14);
    root.insert(16);
    root.insert(12);
    root.insert(11);
    root.insert(10);
    root.insert(17);

    unsafe {
        println!("right branch = {}", (*root.right).data);
        println!("left branch = {}", (*root.left).data);
    }
    
    println!("depth:{}",root.get_depth());
    root.print_tree();
    root.delete_left();
    println!("Deleting everything left of 15");
    root.print_tree();
}
