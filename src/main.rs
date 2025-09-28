#![allow(dead_code)]
#![allow(unused_variables)]
mod ds {
    pub mod list {
        pub mod doublelist;
        pub mod linklist;
    }
    mod stack {
        pub mod linkstack;
        pub mod sqstack;
    }
    mod queue {
        pub mod link_queue;
        pub mod sd_queue;
    }
    pub mod trees {
        pub mod tree;
    }
    pub mod string {
        pub mod link_string;
        pub mod sq_string;
    }
}
pub mod sort {
    pub mod bubble_sort;
    pub mod insert_sort;
    pub mod quick_sort;
}
mod course {
    pub mod error;
    pub mod ptr;
    pub mod simple_parser;
}
mod ds_ptr {
    pub mod vector;
    pub mod auto_ptr;
}
mod leetcode{
    mod l83;
}
fn main() {}
