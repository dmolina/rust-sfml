use graphics::FloatRect;
use graphics::csfml_graphics_sys as ffi;
use std::borrow::{Borrow, ToOwned};
use std::ops::{Deref, DerefMut};
use system::Vector2f;
use std::fmt::{self, Debug};

extern "C" {
    /// 2D camera that defines what region is shown on screen
    ///
    /// This is a very powerful concept: you can scroll,
    /// rotate or zoom the entire scene without altering
    /// the way that your drawable objects are drawn.
    pub type View;
}

impl Debug for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "View at {:p}", self)
    }
}

impl View {
    /// Get the current orientation of a view
    ///
    /// Return the rotation angle of the view, in degrees
    pub fn rotation(&self) -> f32 {
        unsafe { ffi::sfView_getRotation(self.raw()) }
    }
    /// Get the center of a view
    ///
    /// Return the center of the view
    pub fn center(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfView_getCenter(self.raw())) }
    }

    /// Get the size of a view
    ///
    /// Return the size of the view
    pub fn size(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfView_getSize(self.raw())) }
    }

    /// Get the target viewport rectangle of a view
    ///
    /// Return the viewport rectangle, expressed as a factor of the target size
    pub fn viewport(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfView_getViewport(self.raw())) }
    }
    /// Creates a view with position and size
    ///
    /// # Arguments
    /// * center - The center of the view
    /// * size - The size of the view
    pub fn new(center: Vector2f, size: Vector2f) -> ViewBox {
        let mut view = ViewBox::default();
        view.set_center(center);
        view.set_size(size);
        view
    }

    /// Construct a view from a rectangle
    ///
    /// # Arguments
    /// * rectangle - The rectangle defining the zone to display
    pub fn from_rect(rectangle: &FloatRect) -> ViewBox {
        let view = unsafe { ffi::sfView_createFromRect(rectangle.raw()) };
        assert!(!view.is_null(), "Failed to create ViewBox from Rect");
        ViewBox { view: view }
    }

    /// Set the orientation of a view
    ///
    /// The default rotation of a view is 0 degree.
    ///
    /// # Arguments
    /// * angle - New angle, in degrees
    pub fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfView_setRotation(self.raw_mut(), angle) }
    }

    /// Rotate a view relatively to its current orientation
    ///
    /// # Arguments
    /// * angle - Angle to rotate, in degrees
    pub fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfView_rotate(self.raw_mut(), angle) }
    }

    /// Resize a view rectangle relatively to its current size
    ///
    /// Resizing the view simulates a zoom, as the zone displayed
    /// on screen grows or shrinks.
    ///
    /// # factor is a multiplier:
    /// * 1 keeps the size unchanged
    /// * bigger than 1 makes the view bigger (objects appear smaller)
    /// * smaller than 1 makes the view smaller (objects appear bigger)
    ///
    /// # Arguments
    /// * factor - Zoom factor to apply
    pub fn zoom(&mut self, factor: f32) {
        unsafe { ffi::sfView_zoom(self.raw_mut(), factor) }
    }

    /// Set the center of a view
    ///
    /// # Arguments
    /// * center - New center
    pub fn set_center<C: Into<Vector2f>>(&mut self, center: C) {
        unsafe { ffi::sfView_setCenter(self.raw_mut(), center.into().raw()) }
    }

    /// Set the size of a view
    ///
    /// # Arguments
    /// * size - New size of the view
    pub fn set_size<S: Into<Vector2f>>(&mut self, size: S) {
        unsafe { ffi::sfView_setSize(self.raw_mut(), size.into().raw()) }
    }

    /// Move a view relatively to its current position
    ///
    /// # Arguments
    /// * offset - Offset
    pub fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfView_move(self.raw_mut(), offset.into().raw()) }
    }

    /// Set the target viewport of a view
    ///
    /// The viewport is the rectangle into which the contents of the
    /// view are displayed, expressed as a factor (between 0 and 1)
    /// of the size of the render target to which the view is applied.
    /// For example, a view which takes the left side of the target would
    /// be defined by a rect of (0, 0, 0.5, 1).
    /// By default, a view has a viewport which covers the entire target.
    ///
    /// # Arguments
    /// * viewport - New viewport rectangle
    pub fn set_viewport(&mut self, viewport: &FloatRect) {
        unsafe { ffi::sfView_setViewport(self.raw_mut(), viewport.raw()) }
    }

    /// Reset a view to the given rectangle
    ///
    /// Note that this function resets the rotation angle to 0.
    ///
    /// # Arguments
    /// * rectangle - Rectangle defining the zone to display
    pub fn reset(&mut self, rectangle: &FloatRect) {
        unsafe { ffi::sfView_reset(self.raw_mut(), rectangle.raw()) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfView {
        let ptr: *const Self = self;
        ptr as _
    }
    fn raw_mut(&mut self) -> *mut ffi::sfView {
        let ptr: *mut Self = self;
        ptr as _
    }
}

impl ToOwned for View {
    type Owned = ViewBox;
    fn to_owned(&self) -> Self::Owned {
        let view = unsafe { ffi::sfView_copy(self.raw()) };
        if view.is_null() {
            panic!("Not enough memory to clone ViewBox")
        } else {
            ViewBox { view: view }
        }
    }
}

/// An owning handle to a `View` allocated by CSFML.
#[derive(Debug)]
pub struct ViewBox {
    view: *mut ffi::sfView,
}

impl Deref for ViewBox {
    type Target = View;

    fn deref(&self) -> &View {
        unsafe { &*(self.view as *const View) }
    }
}

impl DerefMut for ViewBox {
    fn deref_mut(&mut self) -> &mut View {
        unsafe { &mut *(self.view as *mut View) }
    }
}

impl Default for ViewBox {
    fn default() -> Self {
        let view = unsafe { ffi::sfView_create() };
        assert!(!view.is_null(), "Failed to create ViewBox");
        ViewBox { view: view }
    }
}

impl Borrow<View> for ViewBox {
    fn borrow(&self) -> &View {
        &*self
    }
}

impl Clone for ViewBox {
    /// Return a new ViewBox or panic! if there is not enough memory
    fn clone(&self) -> ViewBox {
        (**self).to_owned()
    }
}

impl Drop for ViewBox {
    fn drop(&mut self) {
        unsafe { ffi::sfView_destroy(self.view) }
    }
}
