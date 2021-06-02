use pmrender::show_lines;

fn main() {
    show_lines(
        vec![
            (0.0, 0.0),
            (0.9, 0.0),
            (0.0, 0.5),
            (0.9, 0.5),
            (0.0, 0.9),
            (0.9, 0.9),
        ],
        [
            [1.0, 0.0, 0.0, 0.0],  // 1. column
            [0.0, 1.0, 0.0, 0.0],  // 2. column
            [0.0, 0.0, 1.0, 0.0],  // 3. column
            [-0.9, 0.0, 0.9, 1.0], // 4. column (left bottom is the origin)
        ],
        900,
        900,
    );
}
