//! Raw declarations for the pinned libghostty-vt C API.
//!
//! This crate deliberately provides no safe wrapper. Consumers must uphold the
//! ownership, lifetime, and output-buffer contracts documented by Ghostty's C
//! headers in `vendor/ghostty/include/ghostty/vt`.
//!
//! The declarations are a reviewed subset of that API. They must be reviewed
//! whenever the Ghostty submodule revision changes.

use std::ffi::{c_int, c_void};

pub type GhosttyResult = c_int;

pub const GHOSTTY_SUCCESS: GhosttyResult = 0;
pub const GHOSTTY_OUT_OF_MEMORY: GhosttyResult = -1;
pub const GHOSTTY_INVALID_VALUE: GhosttyResult = -2;

pub type GhosttyTerminalScrollViewportTag = c_int;

pub const GHOSTTY_SCROLL_VIEWPORT_TOP: GhosttyTerminalScrollViewportTag = 0;
pub const GHOSTTY_SCROLL_VIEWPORT_BOTTOM: GhosttyTerminalScrollViewportTag = 1;
pub const GHOSTTY_SCROLL_VIEWPORT_DELTA: GhosttyTerminalScrollViewportTag = 2;

pub type GhosttyRenderStateData = c_int;

pub const GHOSTTY_RENDER_STATE_DATA_COLS: GhosttyRenderStateData = 1;
pub const GHOSTTY_RENDER_STATE_DATA_ROWS: GhosttyRenderStateData = 2;
pub const GHOSTTY_RENDER_STATE_DATA_ROW_ITERATOR: GhosttyRenderStateData = 4;
pub const GHOSTTY_RENDER_STATE_DATA_CURSOR_VISUAL_STYLE: GhosttyRenderStateData = 10;
pub const GHOSTTY_RENDER_STATE_DATA_CURSOR_VISIBLE: GhosttyRenderStateData = 11;
pub const GHOSTTY_RENDER_STATE_DATA_CURSOR_BLINKING: GhosttyRenderStateData = 12;
pub const GHOSTTY_RENDER_STATE_DATA_CURSOR_VIEWPORT_HAS_VALUE: GhosttyRenderStateData = 14;
pub const GHOSTTY_RENDER_STATE_DATA_CURSOR_VIEWPORT_X: GhosttyRenderStateData = 15;
pub const GHOSTTY_RENDER_STATE_DATA_CURSOR_VIEWPORT_Y: GhosttyRenderStateData = 16;

pub type GhosttyRenderStateRowData = c_int;

pub const GHOSTTY_RENDER_STATE_ROW_DATA_CELLS: GhosttyRenderStateRowData = 3;

pub type GhosttyRenderStateRowCellsData = c_int;

pub const GHOSTTY_RENDER_STATE_ROW_CELLS_DATA_RAW: GhosttyRenderStateRowCellsData = 1;
pub const GHOSTTY_RENDER_STATE_ROW_CELLS_DATA_STYLE: GhosttyRenderStateRowCellsData = 2;
pub const GHOSTTY_RENDER_STATE_ROW_CELLS_DATA_GRAPHEMES_LEN: GhosttyRenderStateRowCellsData = 3;
pub const GHOSTTY_RENDER_STATE_ROW_CELLS_DATA_GRAPHEMES_BUF: GhosttyRenderStateRowCellsData = 4;

pub type GhosttyCellData = c_int;

pub const GHOSTTY_CELL_DATA_CONTENT_TAG: GhosttyCellData = 2;
pub const GHOSTTY_CELL_DATA_WIDE: GhosttyCellData = 3;
pub const GHOSTTY_CELL_DATA_COLOR_PALETTE: GhosttyCellData = 10;
pub const GHOSTTY_CELL_DATA_COLOR_RGB: GhosttyCellData = 11;

pub type GhosttyCellContentTag = c_int;

pub const GHOSTTY_CELL_CONTENT_CODEPOINT: GhosttyCellContentTag = 0;
pub const GHOSTTY_CELL_CONTENT_CODEPOINT_GRAPHEME: GhosttyCellContentTag = 1;
pub const GHOSTTY_CELL_CONTENT_BG_COLOR_PALETTE: GhosttyCellContentTag = 2;
pub const GHOSTTY_CELL_CONTENT_BG_COLOR_RGB: GhosttyCellContentTag = 3;

pub type GhosttyCellWide = c_int;

pub const GHOSTTY_CELL_WIDE_NARROW: GhosttyCellWide = 0;
pub const GHOSTTY_CELL_WIDE_WIDE: GhosttyCellWide = 1;
pub const GHOSTTY_CELL_WIDE_SPACER_TAIL: GhosttyCellWide = 2;
pub const GHOSTTY_CELL_WIDE_SPACER_HEAD: GhosttyCellWide = 3;

pub type GhosttyStyleColorTag = c_int;

pub const GHOSTTY_STYLE_COLOR_NONE: GhosttyStyleColorTag = 0;
pub const GHOSTTY_STYLE_COLOR_PALETTE: GhosttyStyleColorTag = 1;
pub const GHOSTTY_STYLE_COLOR_RGB: GhosttyStyleColorTag = 2;

pub type GhosttyRenderStateCursorVisualStyle = c_int;

pub const GHOSTTY_RENDER_STATE_CURSOR_VISUAL_STYLE_BAR: GhosttyRenderStateCursorVisualStyle = 0;
pub const GHOSTTY_RENDER_STATE_CURSOR_VISUAL_STYLE_BLOCK: GhosttyRenderStateCursorVisualStyle = 1;
pub const GHOSTTY_RENDER_STATE_CURSOR_VISUAL_STYLE_UNDERLINE: GhosttyRenderStateCursorVisualStyle =
    2;
pub const GHOSTTY_RENDER_STATE_CURSOR_VISUAL_STYLE_BLOCK_HOLLOW:
    GhosttyRenderStateCursorVisualStyle = 3;

pub type GhosttySgrUnderline = c_int;

pub const GHOSTTY_SGR_UNDERLINE_NONE: GhosttySgrUnderline = 0;
pub const GHOSTTY_SGR_UNDERLINE_SINGLE: GhosttySgrUnderline = 1;
pub const GHOSTTY_SGR_UNDERLINE_DOUBLE: GhosttySgrUnderline = 2;
pub const GHOSTTY_SGR_UNDERLINE_CURLY: GhosttySgrUnderline = 3;
pub const GHOSTTY_SGR_UNDERLINE_DOTTED: GhosttySgrUnderline = 4;
pub const GHOSTTY_SGR_UNDERLINE_DASHED: GhosttySgrUnderline = 5;

#[repr(C)]
pub struct GhosttyTerminalOptions {
    pub cols: u16,
    pub rows: u16,
    pub max_scrollback: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union GhosttyTerminalScrollViewportValue {
    pub delta: isize,
    pub _padding: [u64; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GhosttyTerminalScrollViewport {
    pub tag: GhosttyTerminalScrollViewportTag,
    pub value: GhosttyTerminalScrollViewportValue,
}

#[repr(C)]
pub struct GhosttyAllocator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GhosttyTerminalImpl {
    _private: [u8; 0],
}

pub type GhosttyTerminal = *mut GhosttyTerminalImpl;

#[repr(C)]
pub struct GhosttyRenderStateImpl {
    _private: [u8; 0],
}

pub type GhosttyRenderState = *mut GhosttyRenderStateImpl;

#[repr(C)]
pub struct GhosttyRenderStateRowIteratorImpl {
    _private: [u8; 0],
}

pub type GhosttyRenderStateRowIterator = *mut GhosttyRenderStateRowIteratorImpl;

#[repr(C)]
pub struct GhosttyRenderStateRowCellsImpl {
    _private: [u8; 0],
}

pub type GhosttyRenderStateRowCells = *mut GhosttyRenderStateRowCellsImpl;

pub type GhosttyCell = u64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GhosttyColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub type GhosttyColorPaletteIndex = u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub union GhosttyStyleColorValue {
    pub palette: GhosttyColorPaletteIndex,
    pub rgb: GhosttyColorRgb,
    pub _padding: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GhosttyStyleColor {
    pub tag: GhosttyStyleColorTag,
    pub value: GhosttyStyleColorValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GhosttyStyle {
    pub size: usize,
    pub fg_color: GhosttyStyleColor,
    pub bg_color: GhosttyStyleColor,
    pub underline_color: GhosttyStyleColor,
    pub bold: bool,
    pub italic: bool,
    pub faint: bool,
    pub blink: bool,
    pub inverse: bool,
    pub invisible: bool,
    pub strikethrough: bool,
    pub overline: bool,
    pub underline: GhosttySgrUnderline,
}

unsafe extern "C" {
    pub fn ghostty_terminal_new(
        allocator: *const GhosttyAllocator,
        terminal: *mut GhosttyTerminal,
        options: GhosttyTerminalOptions,
    ) -> GhosttyResult;

    pub fn ghostty_terminal_free(terminal: GhosttyTerminal);

    pub fn ghostty_terminal_resize(
        terminal: GhosttyTerminal,
        columns: u16,
        rows: u16,
        cell_width_pixels: u32,
        cell_height_pixels: u32,
    ) -> GhosttyResult;

    pub fn ghostty_terminal_vt_write(terminal: GhosttyTerminal, data: *const u8, length: usize);

    pub fn ghostty_terminal_scroll_viewport(
        terminal: GhosttyTerminal,
        behavior: GhosttyTerminalScrollViewport,
    );

    pub fn ghostty_render_state_new(
        allocator: *const GhosttyAllocator,
        state: *mut GhosttyRenderState,
    ) -> GhosttyResult;

    pub fn ghostty_render_state_free(state: GhosttyRenderState);

    pub fn ghostty_render_state_update(
        state: GhosttyRenderState,
        terminal: GhosttyTerminal,
    ) -> GhosttyResult;

    pub fn ghostty_render_state_get(
        state: GhosttyRenderState,
        data: GhosttyRenderStateData,
        out: *mut c_void,
    ) -> GhosttyResult;

    pub fn ghostty_render_state_row_iterator_new(
        allocator: *const GhosttyAllocator,
        iterator: *mut GhosttyRenderStateRowIterator,
    ) -> GhosttyResult;

    pub fn ghostty_render_state_row_iterator_free(iterator: GhosttyRenderStateRowIterator);

    pub fn ghostty_render_state_row_iterator_next(iterator: GhosttyRenderStateRowIterator) -> bool;

    pub fn ghostty_render_state_row_get(
        iterator: GhosttyRenderStateRowIterator,
        data: GhosttyRenderStateRowData,
        out: *mut c_void,
    ) -> GhosttyResult;

    pub fn ghostty_render_state_row_cells_new(
        allocator: *const GhosttyAllocator,
        cells: *mut GhosttyRenderStateRowCells,
    ) -> GhosttyResult;

    pub fn ghostty_render_state_row_cells_free(cells: GhosttyRenderStateRowCells);

    pub fn ghostty_render_state_row_cells_next(cells: GhosttyRenderStateRowCells) -> bool;

    pub fn ghostty_render_state_row_cells_get(
        cells: GhosttyRenderStateRowCells,
        data: GhosttyRenderStateRowCellsData,
        out: *mut c_void,
    ) -> GhosttyResult;

    pub fn ghostty_cell_get(
        cell: GhosttyCell,
        data: GhosttyCellData,
        out: *mut c_void,
    ) -> GhosttyResult;
}
