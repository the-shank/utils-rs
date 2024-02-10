//! Tool to identify all broken symlinks under a particular directory.

use clap::Parser;
use color_eyre::eyre::Result;
use std::env;
use std::fs::{self, read_dir};
use std::path::PathBuf;
use std::time::Instant;

use utils_rs::common::parsers;

/// A simple utility to find broken symlinks
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[arg(value_parser=parsers::parse_dir)]
    root: Option<PathBuf>,

    #[arg(default_value_t = false, short, long)]
    verbose: bool,
}

/// Config for the application
struct Opts {
    verbose: bool,
}

impl Opts {
    pub(crate) fn new(args: &Args) -> Self {
        Self {
            verbose: args.verbose,
        }
    }
}

fn main() -> Result<()> {
    // setup color_eyre panic and error report handlers
    color_eyre::install()?;

    // parse args
    let args = Args::parse();
    // TODO: keep only a reference to root (later)
    let root_dir = args.root.clone().unwrap_or(env::current_dir()?);

    // construct options from the args
    let opts = Opts::new(&args);

    // fire off!
    let start = Instant::now();
    process_dir(&root_dir, &opts)?;
    // parallel(&root_dir, &opts)?;
    println!("Completed in: {:.2?}", start.elapsed());

    Ok(())
}

fn process_entry(entry: &fs::DirEntry, opts: &Opts) -> Result<()> {
    if opts.verbose {
        println!("Processing entry: {:?}", entry.file_name());
    }

    let file_type = entry.file_type()?;

    if file_type.is_dir() {
        // process directory
        process_dir(&entry.path(), opts)?;
    } else if file_type.is_symlink() {
        // process symlink
        let target = entry.path();
        match target.try_exists() {
            Ok(exists) => {
                if !exists {
                    println!("[Broken]: {}", target.display());
                }
            }
            Err(e) => {
                eprintln!("[error ({})]: {}", e.kind(), target.display(),);
            }
        }
    }

    Ok(())
}

fn process_dir(dir: &PathBuf, opts: &Opts) -> Result<()> {
    if opts.verbose {
        println!("Processing directory: {}", dir.display());
    }

    match read_dir(dir) {
        Ok(dir) => {
            // process the entries
            for entry in dir {
                let entry = entry?;
                let _file_type = entry.file_type()?;
                let _ = process_entry(&entry, opts);
            }
        }
        Err(e) => {
            // print the failures to stderr
            eprintln!("[error ({})]: {}", e.kind(), dir.display(),);
        }
    }

    Ok(())
}

// // PARALLEL PROCESSING
// //
// // use a VecDeque<Option<entry>> as the work queue
// //
// // single producer that adds to this work queue
// // multiple consumers that pull items out of this queue and work on them
// // a value of `None` in the queue signals to the worker thread that there wont be any more entries
// // and that the worker thread should exit now
// //
// // observation: this version is 5 times slower than the single threaded version
// // the likely reason is that the contention on the mutex is causing the slowdown as the worker
// // threads are constantsly locking the mutex to see if there is a new item. Another issue is that
// // we are making the calls to entry.file_type() twice now, which is definitely wasteful.
// // TODO: under what conditions will this fail?
// // TODO: LATER: handle failure gracefully rather than just crashing the program
// fn parallel(root: &PathBuf, opts: &Opts) -> Result<()> {
//     let nworkers = 5;
//     let work_queue: Mutex<VecDeque<Option<DirEntry>>> = Mutex::new(VecDeque::new());
//
//     thread::scope(|s| {
//         // start the workers
//         for i in 0..nworkers {
//             s.spawn(|| {
//                 // [-] 1st version: just keep waiting on the queue
//                 // [ ] 2nd version: go to sleep if the queue is empty (use condvar)
//                 let tid = thread::current().id();
//                 println!("started thread: [{:?}]", tid);
//                 loop {
//                     let entry = work_queue.lock().unwrap().pop_front();
//                     match entry {
//                         Some(entry) => {
//                             // process the entry
//                             match entry {
//                                 Some(entry) => match entry.file_type() {
//                                     Ok(file_type) => {
//                                         if file_type.is_symlink() {
//                                             let target = entry.path();
//                                             match target.try_exists() {
//                                                 Ok(exists) => {
//                                                     if !exists {
//                                                         println!("[Broken]: {}", target.display());
//                                                     }
//                                                 }
//                                                 Err(e) => {
//                                                     eprintln!(
//                                                         "[error ({})]: {}",
//                                                         e.kind(),
//                                                         target.display(),
//                                                     );
//                                                 }
//                                             }
//                                         }
//                                     }
//                                     Err(e) => {
//                                         eprintln!("[error ({})]: {:?}", e.kind(), entry.path());
//                                     }
//                                 },
//                                 None => {
//                                     // exit self
//                                     break;
//                                 }
//                             }
//                         }
//                         None => {
//                             // queue was empty
//                             continue;
//                         }
//                     }
//                 }
//                 println!("[W:{:?}] shutting down", tid);
//             });
//         }
//
//         // walk and add entries to the work queue
//         let mut dir_queue = vec![root.clone()];
//         while let Some(dir) = dir_queue.pop() {
//             match read_dir(&dir) {
//                 Ok(dir) => {
//                     // process the entries
//                     for entry in dir {
//                         match entry {
//                             Ok(entry) => match entry.file_type() {
//                                 Ok(file_type) => {
//                                     if file_type.is_dir() {
//                                         let p = entry.path();
//                                         dir_queue.push(p);
//                                     } else {
//                                         // println!(
//                                         //     "[P] adding entry to worker_queue: {:?}",
//                                         //     entry.path()
//                                         // );
//                                         work_queue.lock().unwrap().push_back(Some(entry));
//                                     }
//                                 }
//                                 Err(e) => {
//                                     eprintln!("[error ({})]: {:?}", e.kind(), entry.path());
//                                 }
//                             },
//                             Err(e) => {
//                                 eprintln!("[error]: {:?}", e.kind());
//                             }
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     // print the failures to stderr
//                     eprintln!("[error ({})]: {}", e.kind(), dir.display(),);
//                 }
//             }
//         }
//
//         // signal to the workers to finish
//         for _ in 0..nworkers {
//             work_queue.lock().unwrap().push_back(None);
//         }
//     });
//
//     Ok(())
// }
