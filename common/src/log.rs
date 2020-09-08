pub fn init_log() {
    use chrono::Local;
    use std::io::Write;
    use env_logger::Target;

    std::env::set_var("RUST_LOG", "actix_web=info");

    let env = env_logger::Env::default();
    
    env_logger::Builder::from_env(env).target(Target::Stdout)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        }).init();
}
