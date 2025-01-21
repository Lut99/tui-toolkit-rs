//  SCROLL AREA.rs
//    by Lut99
//
//  Created:
//    21 Jan 2025, 20:26:24
//  Last edited:
//    21 Jan 2025, 20:56:42
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements a widget that can scroll its content.
//

use std::cmp::min;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{StatefulWidget, Widget};


/***** HELPER FUNCTIONS *****/
/// Does the math-y part of the scrolling.
///
/// # Arguments
/// - `scroll`: The amount of scrolling to apply.
/// - `outer`: The size of the _outer_ area (i.e., visible area).
/// - `inner`: The size of the _inner_ area (i.e., total area).
/// - `inner_buf`: The rendered inside area, part of which to copy to the `outer_buf`.
/// - `outer_buf`: The outside area to copy a smaller part of the `inner_buf` to.
fn scroll(scroll: (u16, u16), outer: Rect, inner: Rect, inner_buf: &Buffer, outer_buf: &mut Buffer) {
    // Next, decide which part of the inner window to copy
    let pos: (u16, u16) = (min(scroll.0, outer.width), min(scroll.1, outer.height));
    let cut: Rect = Rect::new(
        pos.0,
        pos.1,
        if inner.width >= outer.width { outer.width - pos.0 } else { inner.width },
        if inner.height >= outer.height { outer.height - pos.1 } else { inner.height },
    );

    // Then we copy that part into the output buffer (with the appropriate offsets)
    for y in 0..cut.height {
        let outer_y: u16 = outer.y + y;
        for x in 0..cut.width {
            let outer_x: u16 = outer.x + x;
            outer_buf.content[(outer_y * outer.width + outer_x) as usize] = inner_buf.content[(y * inner.width + x) as usize].clone();
        }
    }
}





/***** AUXILLARY *****/
/// The state that keeps track of the current scroll position of a [`ScrollArea`].
///
/// This version assumes that no widget state is kept (i.e., the state is [`()`]).
pub type ScrollState = StatefulScrollState<()>;



/// The state that is adapted such that the [`ScrollArea`] scrolls.
///
/// This version assumes that the nested widget is stateful.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StatefulScrollState<S> {
    /// The coordinates that offset the scroll area (as an x x y pair).
    pos:   (u16, u16),
    /// The nested state to pass to the ScrollArea.
    state: S,
}

// Constructors
impl<S: Default> Default for StatefulScrollState<S> {
    #[inline]
    fn default() -> Self { Self { pos: (0, 0), state: Default::default() } }
}
impl<S> StatefulScrollState<S> {
    /// Constructs a new StatefulScrollState.
    ///
    /// # Arguments
    /// - `state`: A nested `S`tate of a nested widget to pass to it when rendering.
    ///
    /// # Returns
    /// A new StatefulScrollState ready for keeping track of scroll states.
    #[inline]
    pub const fn new(state: S) -> Self { Self { pos: (0, 0), state } }
}

// Scrolling
impl<S> StatefulScrollState<S> {
    /// Scrolls the scroll area to the start (topleft-most).
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn reset(&mut self) -> &mut Self {
        self.pos = (0, 0);
        self
    }

    /// Scrolls the scroll area one line up.
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn scroll_up(&mut self) -> &mut Self { self.scroll_up_by(1) }
    /// Scrolls the scroll area up.
    ///
    /// It will automatically clip the scrolling.
    ///
    /// # Arguments
    /// - `n`: The number of lines to scroll up.
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn scroll_up_by(&mut self, n: u16) -> &mut Self {
        self.pos.1 = self.pos.1.saturating_sub(n);
        self
    }

    /// Scrolls the scroll area one character right.
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn scroll_right(&mut self) -> &mut Self { self.scroll_right_by(1) }
    /// Scrolls the scroll area right.
    ///
    /// It will automatically clip the scrolling.
    ///
    /// # Arguments
    /// - `n`: The number of character to scroll right.
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn scroll_right_by(&mut self, n: u16) -> &mut Self {
        self.pos.0 = self.pos.0.saturating_add(n);
        self
    }

    /// Scrolls the scroll area one line down.
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn scroll_down(&mut self) -> &mut Self { self.scroll_down_by(1) }
    /// Scrolls the scroll area down.
    ///
    /// It will automatically clip the scrolling.
    ///
    /// # Arguments
    /// - `n`: The number of lines to scroll down.
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn scroll_down_by(&mut self, n: u16) -> &mut Self {
        self.pos.1 = self.pos.1.saturating_add(n);
        self
    }

    /// Scrolls the scroll area one character left.
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn scroll_left(&mut self) -> &mut Self { self.scroll_left_by(1) }
    /// Scrolls the scroll area left.
    ///
    /// It will automatically clip the scrolling.
    ///
    /// # Arguments
    /// - `n`: The number of character to scroll left.
    ///
    /// # Returns
    /// A mutable reference to Self for chaining.
    #[inline]
    pub const fn scroll_left_by(&mut self, n: u16) -> &mut Self {
        self.pos.0 = self.pos.0.saturating_sub(n);
        self
    }
}

// State
impl<S> StatefulScrollState<S> {
    /// Provides read-only access to the inner scroll state.
    ///
    /// # Returns
    /// A reference to `S`.
    #[inline]
    pub const fn state(&self) -> &S { &self.state }

    /// Provides mutable access to the inner scroll state.
    ///
    /// # Returns
    /// A mutable reference to `S`.
    #[inline]
    pub const fn state_mut(&mut self) -> &mut S { &mut self.state }

    /// Returns the inner scroll state.
    ///
    /// # Returns
    /// The inner `S`.
    #[inline]
    pub fn into_state(self) -> S { self.state }
}





/***** LIBRARY *****/
/// The ScrollArea will render a stateful widget to a larger area, and then cut that area
/// to a smaller one.
///
/// This smaller area can then be scrolled using the [`StatefulScrollState`].
///
/// See the [`ScrollArea`] for non-stateful widgets.
#[derive(Debug, Clone)]
pub struct ScrollArea<W> {
    /// The nested widget that is rendered within the scrolled area.
    widget: W,
    /// The scrolled area, e.g., the size of the thing we're rendering (as a width x height pair).
    inner:  (u16, u16),
}
impl<W> ScrollArea<W> {
    /// Constructs a new ScrollArea.
    ///
    /// # Arguments
    /// - `widget`: Some widget that will render to the _inner_ area. The scroll area takes care to
    ///   project a window over that area that is equal to the ScrollArea's _outer_ area.
    /// - `inner`: The size of the scroll area's inner area (i.e., the size of the area the inner
    ///   widget renders to). Given as `(width x height)`.
    ///
    /// # Returns
    /// A new ScrollArea that can be rendered.
    #[inline]
    pub const fn new(widget: W, inner: (u16, u16)) -> Self { Self { widget, inner } }
}
impl<W: Widget> StatefulWidget for ScrollArea<W> {
    type State = ScrollState;

    #[inline]
    fn render(self, outer: Rect, outer_buf: &mut Buffer, state: &mut Self::State) {
        // Render the given widget to a buffer the size of the inner area first.
        let inner: Rect = Rect::new(0, 0, self.inner.0, self.inner.1);
        let mut inner_buf = Buffer::empty(inner);
        self.widget.render(inner, &mut inner_buf);

        // Run the math
        scroll(state.pos, outer, inner, &inner_buf, outer_buf);
    }
}



/// The StatefulScrollArea will render a stateful widget to a larger area, and then cut that area
/// to a smaller one.
///
/// This smaller area can then be scrolled using the [`StatefulScrollState`].
///
/// See the [`ScrollArea`] for non-stateful widgets.
#[derive(Debug, Clone)]
pub struct StatefulScrollArea<W> {
    /// The nested widget that is rendered within the scrolled area.
    widget: W,
    /// The scrolled area, e.g., the size of the thing we're rendering (as a width x height pair).
    inner:  (u16, u16),
}
impl<W> StatefulScrollArea<W> {
    /// Constructs a new StatefulScrollArea.
    ///
    /// # Arguments
    /// - `widget`: Some widget that will render to the _inner_ area. The scroll area takes care to
    ///   project a window over that area that is equal to the StatefulScrollArea's _outer_ area.
    /// - `inner`: The size of the scroll area's inner area (i.e., the size of the area the inner
    ///   widget renders to). Given as `(width x height)`.
    ///
    /// # Returns
    /// A new StatefulScrollArea that can be rendered.
    #[inline]
    pub const fn new(widget: W, inner: (u16, u16)) -> Self { Self { widget, inner } }
}
impl<W: StatefulWidget> StatefulWidget for StatefulScrollArea<W> {
    type State = StatefulScrollState<W::State>;

    #[inline]
    fn render(self, outer: Rect, outer_buf: &mut Buffer, state: &mut Self::State) {
        // Render the given widget to a buffer the size of the inner area first.
        let inner: Rect = Rect::new(0, 0, self.inner.0, self.inner.1);
        let mut inner_buf = Buffer::empty(inner);
        self.widget.render(inner, &mut inner_buf, &mut state.state);

        // Run the math
        scroll(state.pos, outer, inner, &inner_buf, outer_buf);
    }
}
