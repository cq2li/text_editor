struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
}

impl CursorController {
    fn new() -> Self {
        Self {cursor_x: 0, cursor_y: 0}
    }
} 
