use std::ffi::CStr;

use rvvm_sys::fdt_node;
use static_assertions::{
    assert_eq_align,
    assert_eq_size,
};

use super::{
    FdtFindExt,
    FdtNodeAddExt,
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
    pub fn add<Prop: FdtNodeAddExt>(
        &mut self,
        name: impl AsRef<CStr>,
        prop: Prop,
    ) {
        // SAFETY: safe, since self is well-allocated and
        // well-aligned
        unsafe {
            prop.fdt_node_add(
                name.as_ref(),
                self as *mut Node as *mut fdt_node,
            )
        }
    }

    /// Search node in the tree. See `Node::find`.
    pub fn find_mut<By: FdtFindExt>(
        &mut self,
        by: By,
    ) -> Option<&'_ mut Node> {
        // SAFETY: safe, since search operation will not mutate
        // passed pointer
        let result = unsafe { by.find_child_ptr(self.mut_ptr_immut()) };

        if result.is_null() {
            None
        } else {
            // SAFETY: safe, since user can't get two mutable refs from
            // this function
            Some(unsafe { Node::from_ptr_mut(result) })
        }
    }

    /// Search through the node tree.
    ///
    /// Supported filters:
    /// - by name search, takes Name(cstr) struct
    /// - by name search, takes any `AsRef<str>` struct, but
    ///   allocates new `CString` and can panic if passed
    ///   string contains nul-byte terminator
    /// - search by name and region (searches the region
    ///   node type), takes Region(cstr, region).
    /// - search by name (searches the region node type),
    ///   takes AnyRegion(cstr)
    pub fn find<By: FdtFindExt>(&self, by: By) -> Option<&'_ Node> {
        // SAFETY: safe, since search operation will not mutate
        // passed pointer
        let result = unsafe { by.find_child_ptr(self.mut_ptr_immut()) };

        if result.is_null() {
            None
        } else {
            // SAFETY: safe, since result is not null and well-aligned
            Some(unsafe { Node::from_ptr(result as *const _) })
        }
    }
}

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
            // SAFETY: safe since self.node.name is not null & contains
            // nul-byte terminator
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

impl Node {
    unsafe fn mut_ptr_immut<'a>(&'a self) -> *mut fdt_node {
        union U<'a> {
            i: &'a Node,
            o: *mut fdt_node,
        }

        // VERY evil cast
        U::<'a> { i: self }.o
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
