pub fn seq_gen() -> Box<(dyn Fn() -> usize)> {
    Box::new(
        move || {
            let mut count: usize = 0;

            move || {
                count += 1;
                count
            }
        }()
    )
}
