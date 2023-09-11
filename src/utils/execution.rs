use anyhow::Result;
use crossbeam_utils::thread;
use  std::time;

pub trait  Runnable:Sync {
    fn run(&self)->Result<()>;
}
pub fn run_in_parallel(runnables:Vec<&dyn Runnable>){
    thread::scope(|s|{
        for  runnale in runnables {
            s.spawn(move |_|{
                runnale.run().unwrap();
            });
        }
    }).unwrap();
}
pub fn sleep_millis(millis:u64){
    let wait_time=time::Duration::from_millis(millis);
    std::thread::sleep(wait_time);
}