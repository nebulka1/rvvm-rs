use std::{
    ffi::CString,
    mem::ManuallyDrop,
    ops::{
        Deref,
        DerefMut,
    },
    ptr,
};

use rvvm_sys::{
    fdt_node,
    fdt_node_create,
    fdt_node_free,
};

use super::borrowed::*;

/// Owned version of the fdt `Node`
#[repr(transparent)]
pub struct NodeBuf {
    inner: &'static mut Node,
}

impl NodeBuf {
    /// Allocates single root fdt node. Same as Calling
    /// `NodeBuf::new(None)`, so, for detailed
    /// description refer to the `NodeBuf::new` method.
    pub fn root() -> Self {
        Self::new(None)
    }

    /// Allocates single fdt node. Root node should have the
    /// `None` name.
    ///
    /// # Panics
    ///
    /// Panics if specified name contains nul-byte character
    ///
    /// Panic example:
    ///
    /// ```should_panic
    /// use rvvm::fdt::NodeBuf;
    ///
    /// let _ = NodeBuf::new(Some("Fuckery\0bug!"));
    /// ```
    ///
    /// Good example:
    ///
    /// ```
    /// use rvvm::fdt::NodeBuf;
    ///
    /// let _ = NodeBuf::new(Some("Fuckery no bugs!"));
    /// ```
    pub fn new<'a>(name: impl Into<Option<&'a str>>) -> Self {
        let name = name.into().map(|s| {
            CString::new(s).expect("String contains nul-byte character")
        });

        let node_ptr = if let Some(name) = name {
            // SAFETY: name is a valid nul-terminated C-string
            // And now fdtlib owns its own copy of the name, so dropping
            // name is fine.
            unsafe { fdt_node_create(name.as_ptr()) }
        } else {
            // SAFETY: `fdt_node_create()` also accepts NULL as an
            // argument. This means that created node is meant to be
            // root.
            unsafe { fdt_node_create(ptr::null()) }
        };

        if node_ptr.is_null() {
            panic!("BUG: fdt_node_create() returned null pointer");
        }

        Self {
            // SAFETY: `node_ptr` is valid and well-aligned
            inner: unsafe { Node::from_ptr_mut::<'static>(node_ptr) },
        }
    }

    /// Get inner node reference. Memory allocated for the
    /// `Node` will not be freed even if reference will
    /// be dropped.
    ///
    /// # Safety
    ///
    /// Unsafe due to leaking of the owned reference
    pub unsafe fn leak(self) -> &'static mut Node {
        let this = ManuallyDrop::new(self);
        ptr::read(&this.inner)
    }

    /// Construct `NodeBuf` from the underlying reference
    /// type.
    ///
    /// Possibly acquired through the `NodeBuf::leak`
    pub fn unleak(node: &'static mut Node) -> Self {
        Self { inner: node }
    }
}

impl AsMut<Node> for NodeBuf {
    fn as_mut(&mut self) -> &mut Node {
        &mut *self.inner
    }
}

impl AsRef<Node> for NodeBuf {
    fn as_ref(&self) -> &Node {
        &*self.inner
    }
}

impl DerefMut for NodeBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

impl Deref for NodeBuf {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl Drop for NodeBuf {
    fn drop(&mut self) {
        // SAFETY: self.inner a reference, which guarantees that it
        // is:
        // - Non-null
        // - Well-aligned
        // - Contains valid pointer
        unsafe { fdt_node_free(self.inner as *mut Node as *mut fdt_node) };
    }
}
