pub async fn launch() {
    env_logger::init();

    crate::window::create_and_run();
}
