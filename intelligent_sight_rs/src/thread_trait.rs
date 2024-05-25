pub trait Processor
where
    Self: Sized,
{
    type Output;
    fn get_output_buffer(&self) -> intelligent_sight_lib::Reader<Self::Output>;
    fn start_processor(self) -> std::thread::JoinHandle<()>;
}
