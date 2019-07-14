use std::mem;
use std::ptr;
use rand::{self, Rng};
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
                if !(*self.left).left.is_null() {
                    (*self.left).delete_left();
                }
                if !(*self.left).right.is_null() {
                    (*self.left).delete_right();
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
      get_left_value: If value of node left of current return Option value containing data
                      else return None value.

    */
    fn get_left_value(&self)-> Option<i32> {
       unsafe {
            if ! self.left.is_null() {       
                Some((*self.left).data)
            } else {
                None
            }
       }
       
    }
    /*
      get_left_value: If value of node left of current return Option value containing data
                      else return None value.

     */
    fn get_right_value(&self)-> Option<i32> {
       unsafe {
            if ! self.right.is_null() {       
                Some((*self.right).data)
            } else {
                None
            }
       }
       
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

    /* is_balanced: test whether tree is balanced. Algorithm taken from.
                    https://www.geeksforgeeks.org/how-to-determine-if-a-binary-tree-is-balanced/

    */

    fn is_balanced(&self)-> bool {
        let mut left_height = 0;
        let mut right_height = 0;

        if self.left.is_null() && self.right.is_null() {
            return true;
        }

        
        if !self.left.is_null() {
            unsafe {
                left_height = (*self.left).get_depth();
            }
        }

        if !self.right.is_null(){
            unsafe {
                right_height = (*self.right).get_depth();
            }
        }

        if (left_height - right_height).abs() <=1 {
            let mut bool1 = false;
            let mut bool2 = false;
            if self.right.is_null() {
                bool1 = true
            } else {
                unsafe {
                    bool1 = (*self.right).is_balanced();
                }
            }
            if self.left.is_null() {
                bool2 = true;
            } else {
                unsafe {
                    bool2 = (*self.left).is_balanced();
                }
            }

            if bool1 && bool2 {
                return true;
            } else {
                return false;
            }
            
        }
        //not balanced
        return false;

    }
}

fn main() {
    println!("unsafe rust!");
    let mut root = Node::new(100);


    root.insert(99);
    root.insert(101);
     println!("root  balanced:{}",root.is_balanced());
    // tree should be balanced;
    root.insert(102);
    root.insert(103);
    root.insert(104);
    println!("root  balanced:{}",root.is_balanced());
    

    
    let mut rng = rand::thread_rng();
    for _ in 1..200 {
        let die = rng.gen_range(1, 200);
        root.insert(die);
    }
    

    unsafe {
        println!("right branch = {}", (*root.right).data);
        println!("left branch = {}", (*root.left).data);
    }
    
    println!("depth:{}",root.get_depth());
    root.print_tree();
    let leftvalue = match root.get_left_value() {
        Some(n)=> {n},
        None => 0

    };
    println!("Deleting everything left of {}",leftvalue);
    root.delete_left();

    root.print_tree();
}
