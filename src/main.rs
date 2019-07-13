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

    /*
       Insert: Take in a number and inserts a node.
               If number is larger than current node it inserts to right.
               If number is smaller than current node it inserts to left.
               If number already exists it doesn't do anything. This tree doesnt add dupes.
     */

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

    /*
        delete_left: Deletes the node to the left and all children of that node
    */
    
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

    /*
        delete_right: Deletes the node to the right and all children of that node
    */
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

    /*
        print_tree: Prints a tree somewhat in order of levels
    */

    fn print_tree(&self) {
        let mut vectorised_tree:Vec<Vec<i32>> = Vec::new();

        let size = self.get_depth()+1;
        for _ in 0..size {
            let inner: Vec<i32> = Vec::new();
            vectorised_tree.push(inner);
        }

        fn recurse_build (n_d:&Node,mut tree:&mut Vec<Vec<i32>>,depth:i32)  {
            tree[depth as usize].push(n_d.data);
            //println!("Tree data pushing:{}",n_d.data);

            if ! n_d.right.is_null() {
                unsafe {
                    recurse_build(&*n_d.right , tree,depth - 1);
                }
            }

            if ! n_d.left.is_null(){
                unsafe {
                    recurse_build(&*n_d.left , tree,depth -1);
                }
            }
        }
        recurse_build(self,&mut vectorised_tree,size-1);

        for i in (0..size).rev() {
            let inner_value = vectorised_tree.get(i as usize);
            println!("");
            match inner_value {
                Some(n) => {
                    for k in n.iter().rev() {
                        print!(" {} ",k);
                    }
                }
                 None =>{}
            }
        }
          println!("\n");
    }

    /*
       get_depth: Finds the depth of the tree.
                  Must visit each node. 
    */

    fn get_depth(&self)->i32 {
       
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
        return get_depth_inner(&self,0);

        
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
