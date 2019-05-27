use std::fmt::{ Display};

pub struct Tree<T: Display> {
    left:Option<Box<Tree<T>>>,
    right:Option<Box<Tree<T>>>,
    root:T,
}

impl <T: Display> Tree<T> {
    fn new(root:T) -> Tree<T> {
        Tree{
            root:root,
            left:None,
            right:None
        }


    }

    fn insert_left(&mut self,leaf:Tree<T>){
        self.left = Some(Box::new(leaf));
        
    }

    fn insert_right(&mut self,leaf:Tree<T>) {
        self.right = Some(Box::new(leaf));
        
    }
   pub fn print_leaves_left_to_right(&self){

    match &self.left {
        None =>{
        },
        Some(n) => {
            n.print_leaves_left_to_right();
        }
    }

    match &self.right {
        None => {
        },
        Some(n) => {
            n.print_leaves_left_to_right();
        }
    }
    println!("leaf: {}",self.root);
}

}


fn main() {
    let mut tt = Tree::new(7);
    tt.insert_left(Tree::new(6));
    tt.insert_right(Tree::new(8));


    tt.print_leaves_left_to_right();
                       
}
