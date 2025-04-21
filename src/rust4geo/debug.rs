#[macro_export]
macro_rules! time_it {
    ($expr:expr) => {
        {
            let start = std::time::Instant::now();
            let result = $expr;
            let duration = start.elapsed();
            println!("{} executed in {:?}", stringify!($expr), duration);
            result
        }
    };
}
