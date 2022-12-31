use std::{
    ffi::CStr,
    mem,
    slice,
};

use rvvm_sys::{
    fdt_node,
    fdt_node_add_child,
    fdt_node_get_phandle,
    fdt_prop_list,
    fdt_serialize,
    fdt_size,
};
use static_assertions::{
    assert_eq_align,
    assert_eq_size,
};

use super::{
    error::SerializeError,
    FdtFindExt,
    FdtNodeAddPropExt,
    NodeBuf,
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
    pub fn child(&mut self, child: NodeBuf) -> &mut Self {
        // SAFETY: child has valid Node inside and self is valid
        // Node too
        unsafe {
            let ptr = child.leak();
            fdt_node_add_child(self.mut_ptr(), ptr.mut_ptr());
        }

        self
    }

    pub fn has_prop(&self, name: impl AsRef<CStr>) -> bool {
        unsafe fn has_prop_inner(
            name: &CStr,
            list: *const fdt_prop_list,
        ) -> bool {
            if list.is_null() {
                false
            } else {
                let list = &*list;
                if !list.prop.name.is_null()
                    && CStr::from_ptr(list.prop.name) == name
                {
                    true
                } else {
                    has_prop_inner(name, list.next)
                }
            }
        }

        let props = self.node.props;
        unsafe { has_prop_inner(name.as_ref(), props) }
    }

    pub fn prop<Prop: FdtNodeAddPropExt>(
        &mut self,
        name: impl AsRef<CStr>,
        prop: Prop,
    ) -> &mut Self {
        // SAFETY: safe, since self is well-allocated and
        // well-aligned
        unsafe {
            prop.fdt_node_add(
                name.as_ref(),
                self as *mut Node as *mut fdt_node,
            )
        }

        self
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

    /// Serializes nodes to the dynamically allocated
    /// buffer.
    ///
    /// Same as calling to `Node::size` and then
    /// `Node::try_serialize_to_uninit` with the
    /// pre-allocated buffer with sufficient size.
    ///
    /// Refer to the `Node::try_serialize_to_uninit` for
    /// more information.
    pub fn serialize(&self, boot_cpuid: u32) -> Vec<u8> {
        let size = self.size();
        let mut buffer = Vec::with_capacity(size);
        let size = self
            .try_serialize_to_uninit(
                &mut buffer.spare_capacity_mut()[..size],
                boot_cpuid,
            )
            .expect("Failed to serialize, its a bug");

        // SAFETY: size bytes is initialized by the
        // `try_serialize_to_uninit`
        unsafe { buffer.set_len(size) }

        buffer
    }

    /// Try serialize to buffer. Same as
    /// `Node::try_serialize_to_uninit` but consumes the
    /// initialized slice of bytes, refer to the
    /// `Node::try_serialize_to_uninit` for more detailed
    /// description.
    pub fn try_serialize_to<'a>(
        &self,
        to: &'a mut [u8],
        boot_cpuid: u32,
    ) -> Result<usize, SerializeError> {
        self.try_serialize_to_uninit(
            // SAFETY: this is safe, since `mem::MaybeUninit<u8>` has same
            // in-memory representation as the `u8`
            unsafe {
                slice::from_raw_parts_mut::<'a, mem::MaybeUninit<u8>>(
                    to.as_ptr() as *mut _,
                    to.len(),
                )
            },
            boot_cpuid,
        )
    }

    /// Try serialize to buffer.
    ///
    /// Returns `Ok` with size of serialized content,
    /// otherwise `Err` with the `SerializeError` enum.
    pub fn try_serialize_to_uninit(
        &self,
        to: &mut [mem::MaybeUninit<u8>],
        boot_cpuid: u32,
    ) -> Result<usize, SerializeError> {
        // SAFETY: `fdt_serialize` is not mutating the pointer and
        // to is a valid location
        let len = unsafe {
            fdt_serialize(
                self.mut_ptr_immut(),
                to.as_ptr() as *mut () as *mut _,
                to.len(),
                boot_cpuid,
            )
        };

        if len == 0 {
            Err(SerializeError::InsufficientSpace)
        } else {
            Ok(len)
        }
    }

    /// Calculate size of the serialized fdt
    pub fn size(&self) -> usize {
        // SAFETY: this is safe, since `fdt_size` is not mutating
        // the pointer
        unsafe { fdt_size(self.mut_ptr_immut()) }
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

    pub fn phandle(&mut self) -> u32 {
        // SAFETY: safe, since self.mut_ptr() returns valid mutable
        // pointer
        unsafe { fdt_node_get_phandle(self.mut_ptr()) }
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

    pub fn mut_ptr(&mut self) -> *mut fdt_node {
        self as *mut Node as *mut fdt_node
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
