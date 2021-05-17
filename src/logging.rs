pub fn init(log_level: &str) {
    let logging_env = env_logger::Env::default().default_filter_or(log_level);
    env_logger::Builder::from_env(logging_env).init();
}
