use engine::Node;

use core::fmt::Display;
use std::fmt;

pub struct UctGfx<'a> {
    root: &'a Node,
}

impl<'a> UctGfx<'a> {

    pub fn new(root: &Node) -> UctGfx{
        UctGfx { 
            root: root,
        }
    }
}

impl<'a> Display for UctGfx<'a> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str("LABEL ");
        for child in self.root.children() {
            s.push_str(&format!("{} ", child.m().to_gtp()));
            s.push_str(&format!("{} ", child.playouts()));
        }

        s.fmt(f)
    }
}
