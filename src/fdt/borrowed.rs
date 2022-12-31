use std::ffi::CStr;

use rvvm_sys::fdt_node;
use static_assertions::{
    assert_eq_align,
    assert_eq_size,
};

/// Struct that represents the underlying `fdt_node`
/// (Flattened device tree).
#[repr(transparent)]
pub struct Node {
    node: fdt_node,
}

assert_eq_size!(Node, fdt_node);
assert_eq_align!(Node, fdt_node);

impl Node {
    /// Creates `Node` from the `fdt_node` type.
    pub const fn new(node: fdt_node) -> Self {
        Self { node }
    }

    /// Get Node name. Returns None if node has no name.
    ///
    /// ```
    /// use rvvm::fdt::*;
    ///
    /// let node1 = NodeBuf::new("Nero");
    /// let node2 = NodeBuf::new(None);
    /// let node3 = NodeBuf::root();
    ///
    /// assert_eq!(node1.name().unwrap().to_str().unwrap(), "Nero");
    /// assert!(node2.name().is_none());
    /// assert!(node3.name().is_none());
    /// ```
    pub fn name<'a>(&'a self) -> Option<&'a CStr> {
        if self.node.name.is_null() {
            None
        } else {
            // SAFETY: safe since self.node.name is not null
            Some(unsafe { CStr::from_ptr::<'a>(self.node.name) })
        }
    }

    /// Check whether node is root or not.
    ///
    /// ```
    /// use rvvm::fdt::*;
    ///
    /// let node1 = NodeBuf::new("LekKit");
    /// let node2 = NodeBuf::root();
    /// let node3 = NodeBuf::new(None);
    ///
    /// assert!(!node1.is_root());
    /// assert!(node2.is_root());
    /// assert!(node3.is_root());
    /// ```
    pub fn is_root(&self) -> bool {
        self.node.name.is_null()
    }
}

impl Node {
    /// Creates `Node` reference from the underlying pointer
    /// to the `fdt_node`.
    ///
    /// # Safety
    ///
    /// This function is unsafe due to lack of pointer
    /// validity checks and due to possibility of producing
    /// an unbounded lifetimes, so actions considered UB
    /// are:
    ///
    /// - Specifying an invalid aligned pointer or null
    ///   pointer
    /// - Specifying invalid pointer to the `fdt_node`
    ///   struct
    ///
    /// Not considered as UB, but possible logical errors:
    /// - Producing an unbounded lifetime. Explicit lifetime
    ///   specifying is heavily reccommended, since lifetime
    ///   can be anything that satisfies the type-inference.
    pub unsafe fn from_ptr<'new>(ptr: *const fdt_node) -> &'new Node {
        &*(ptr as *const Node)
    }

    /// Creates mutable `Node` from the underlying pointer.
    ///
    /// # Safety
    ///
    /// Same as the `Node::from_ptr`
    pub unsafe fn from_ptr_mut<'new>(
        ptr: *mut fdt_node,
    ) -> &'new mut Node {
        &mut *(ptr as *mut Node)
    }
}

// Trait impls

impl From<fdt_node> for Node {
    fn from(value: fdt_node) -> Self {
        Self::new(value)
    }
}

impl From<Node> for fdt_node {
    fn from(value: Node) -> Self {
        value.node
    }
}
