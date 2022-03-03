pub trait Progress {
    fn increment(step: i32);
    fn set_progress(progress: i32);
    fn start();
    fn stop();
}
