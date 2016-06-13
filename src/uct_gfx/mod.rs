use config::Config;
use engine::Node;

use core::fmt::Display;
use std::fmt;
use std::sync::Arc;

pub struct UctGfx<'a> {
    config: Arc<Config>,
    root: &'a Node,
}

impl<'a> UctGfx<'a> {

    pub fn new(config: Arc<Config>, root: &Node) -> UctGfx{
        UctGfx { 
            config: config,
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
