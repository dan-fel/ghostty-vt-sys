use std::ptr;

#[test]
fn creates_and_frees_a_terminal() {
    let mut terminal: ghostty_vt_sys::GhosttyTerminal = ptr::null_mut();

    // SAFETY: The default allocator is selected with null, `terminal` is a valid out-pointer,
    // and both terminal dimensions are non-zero.
    let result = unsafe {
        ghostty_vt_sys::ghostty_terminal_new(
            ptr::null(),
            &mut terminal,
            ghostty_vt_sys::GhosttyTerminalOptions {
                cols: 1,
                rows: 1,
                max_scrollback: 0,
            },
        )
    };

    assert_eq!(result, ghostty_vt_sys::GHOSTTY_SUCCESS);
    assert!(!terminal.is_null());

    // SAFETY: Successful creation returned this owned terminal handle, which has not been freed.
    unsafe {
        ghostty_vt_sys::ghostty_terminal_free(terminal);
    }
}
