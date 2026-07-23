use std::sync::OnceLock;

// Dedicated thread pool with a larger stack to accommodate deeply nested rayon
// parallelism (collect_children → deserialize → collect_children → …).
pub fn deserializer_pool() -> &'static rayon::ThreadPool {
    static POOL: OnceLock<rayon::ThreadPool> = OnceLock::new();
    POOL.get_or_init(|| {
        rayon::ThreadPoolBuilder::new()
            .stack_size(32 * 1024 * 1024)
            .build()
            .expect("failed to build deserializer thread pool")
    })
}
