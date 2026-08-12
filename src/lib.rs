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
pub const GHOSTTY_OUT_OF_SPACE: GhosttyResult = -3;

pub type GhosttyMode = u16;

pub const GHOSTTY_MODE_DECCKM: GhosttyMode = 1;
pub const GHOSTTY_MODE_ALT_SCROLL: GhosttyMode = 1007;

pub type GhosttyTerminalData = c_int;

pub const GHOSTTY_TERMINAL_DATA_ACTIVE_SCREEN: GhosttyTerminalData = 6;
pub const GHOSTTY_TERMINAL_DATA_SCROLLBAR: GhosttyTerminalData = 9;
pub const GHOSTTY_TERMINAL_DATA_MOUSE_TRACKING: GhosttyTerminalData = 11;

pub type GhosttyTerminalScreen = c_int;

pub const GHOSTTY_TERMINAL_SCREEN_PRIMARY: GhosttyTerminalScreen = 0;
pub const GHOSTTY_TERMINAL_SCREEN_ALTERNATE: GhosttyTerminalScreen = 1;

pub type GhosttyMods = u16;

pub const GHOSTTY_MODS_SHIFT: GhosttyMods = 1 << 0;
pub const GHOSTTY_MODS_CTRL: GhosttyMods = 1 << 1;
pub const GHOSTTY_MODS_ALT: GhosttyMods = 1 << 2;
pub const GHOSTTY_MODS_SUPER: GhosttyMods = 1 << 3;

pub type GhosttyMouseAction = c_int;

pub const GHOSTTY_MOUSE_ACTION_PRESS: GhosttyMouseAction = 0;

pub type GhosttyMouseButton = c_int;

pub const GHOSTTY_MOUSE_BUTTON_FOUR: GhosttyMouseButton = 4;
pub const GHOSTTY_MOUSE_BUTTON_FIVE: GhosttyMouseButton = 5;

pub type GhosttyMouseEncoderOption = c_int;

pub const GHOSTTY_MOUSE_ENCODER_OPT_SIZE: GhosttyMouseEncoderOption = 2;

pub type GhosttyTerminalScrollViewportTag = c_int;

pub const GHOSTTY_SCROLL_VIEWPORT_TOP: GhosttyTerminalScrollViewportTag = 0;
pub const GHOSTTY_SCROLL_VIEWPORT_BOTTOM: GhosttyTerminalScrollViewportTag = 1;
pub const GHOSTTY_SCROLL_VIEWPORT_DELTA: GhosttyTerminalScrollViewportTag = 2;

pub type GhosttyTerminalOption = c_int;

pub const GHOSTTY_TERMINAL_OPT_USERDATA: GhosttyTerminalOption = 0;
pub const GHOSTTY_TERMINAL_OPT_WRITE_PTY: GhosttyTerminalOption = 1;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GhosttyTerminalScrollbar {
    pub total: u64,
    pub offset: u64,
    pub len: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GhosttyMousePosition {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GhosttyMouseEncoderSize {
    pub size: usize,
    pub screen_width: u32,
    pub screen_height: u32,
    pub cell_width: u32,
    pub cell_height: u32,
    pub padding_top: u32,
    pub padding_bottom: u32,
    pub padding_right: u32,
    pub padding_left: u32,
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
pub struct GhosttyMouseEventImpl {
    _private: [u8; 0],
}

pub type GhosttyMouseEvent = *mut GhosttyMouseEventImpl;

#[repr(C)]
pub struct GhosttyMouseEncoderImpl {
    _private: [u8; 0],
}

pub type GhosttyMouseEncoder = *mut GhosttyMouseEncoderImpl;

pub type GhosttyTerminalWritePtyFn = unsafe extern "C" fn(
    terminal: GhosttyTerminal,
    userdata: *mut c_void,
    data: *const u8,
    length: usize,
);

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

    pub fn ghostty_terminal_set(
        terminal: GhosttyTerminal,
        option: GhosttyTerminalOption,
        value: *const c_void,
    ) -> GhosttyResult;

    pub fn ghostty_terminal_scroll_viewport(
        terminal: GhosttyTerminal,
        behavior: GhosttyTerminalScrollViewport,
    );

    pub fn ghostty_terminal_mode_get(
        terminal: GhosttyTerminal,
        mode: GhosttyMode,
        out_value: *mut bool,
    ) -> GhosttyResult;

    pub fn ghostty_terminal_get(
        terminal: GhosttyTerminal,
        data: GhosttyTerminalData,
        out: *mut c_void,
    ) -> GhosttyResult;

    pub fn ghostty_mouse_event_new(
        allocator: *const GhosttyAllocator,
        event: *mut GhosttyMouseEvent,
    ) -> GhosttyResult;

    pub fn ghostty_mouse_event_free(event: GhosttyMouseEvent);

    pub fn ghostty_mouse_event_set_action(event: GhosttyMouseEvent, action: GhosttyMouseAction);

    pub fn ghostty_mouse_event_set_button(event: GhosttyMouseEvent, button: GhosttyMouseButton);

    pub fn ghostty_mouse_event_set_mods(event: GhosttyMouseEvent, mods: GhosttyMods);

    pub fn ghostty_mouse_event_set_position(
        event: GhosttyMouseEvent,
        position: GhosttyMousePosition,
    );

    pub fn ghostty_mouse_encoder_new(
        allocator: *const GhosttyAllocator,
        encoder: *mut GhosttyMouseEncoder,
    ) -> GhosttyResult;

    pub fn ghostty_mouse_encoder_free(encoder: GhosttyMouseEncoder);

    pub fn ghostty_mouse_encoder_setopt(
        encoder: GhosttyMouseEncoder,
        option: GhosttyMouseEncoderOption,
        value: *const c_void,
    );

    pub fn ghostty_mouse_encoder_setopt_from_terminal(
        encoder: GhosttyMouseEncoder,
        terminal: GhosttyTerminal,
    );

    pub fn ghostty_mouse_encoder_encode(
        encoder: GhosttyMouseEncoder,
        event: GhosttyMouseEvent,
        out_buf: *mut std::ffi::c_char,
        out_buf_size: usize,
        out_len: *mut usize,
    ) -> GhosttyResult;

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
