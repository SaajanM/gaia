use anyhow::Result;
use memmap2::MmapOptions;
use rand::Rng;
use tokio::{fs::OpenOptions, runtime::Builder};

mod argparse;
mod prestart;

const WORLD_SIZE: usize = 10 * 1024 * 1024 * 1024;
const CHUNK_SIZE: usize = 10 * 256 * 1024 * 1024;

fn main() -> Result<()> {
    let args = argparse::parse();

    let runtime = Builder::new_multi_thread()
        .worker_threads(args.thread_count)
        .thread_name("titan-worker")
        .build()
        .unwrap();

    runtime.block_on(pre_start())?;
    Ok(())
}

// async fn start_server<'a>() -> Result<()> {
//     let args = argparse::parse();

//     let world_file = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .open(&args.world_path)
//         .await?;

//     world_file.set_len(WORLD_SIZE as u64).await?;

//     let thread_size = f64::ceil((CHUNK_SIZE as f64) / (args.thread_count as f64)) as usize;

//     let mut mmap = unsafe { MmapOptions::new().stack().populate().map_mut(&world_file)? };
//     let mmaps = mmap.chunks_mut(CHUNK_SIZE);

//     for chunk in mmaps {
//         let thread_segments = chunk.chunks_mut(thread_size);

//         tokio_scoped::scope(|scope| {
//             for (thread_num, data) in thread_segments.enumerate() {
//                 scope.spawn(async move {
//                     let mut rng = rand::thread_rng();
//                     println!("Hello Gaia! From Titan {}", thread_num);
//                     for i in 0..data.len() {
//                         data[i] = rng.gen_range(0..255);
//                     }
//                 });
//             }
//         });
//     }

//     Ok(())
// }
