    use std::{path::PathBuf, sync::mpsc::Sender};
    use std::thread;
    //use std::sync::mpsc;
    use notify::{event::{AccessKind, AccessMode}, Event, EventKind, RecursiveMode, Watcher};
    //use crate::Signal; 
    use crate::signal::Signal;

    pub fn start_watch(file_path: &PathBuf, channel: Sender<Signal>) {
        let list_file = file_path.clone();
        let mut watch_file = file_path.clone();
        watch_file.pop(); //this will make the watcher watch every file in the directory

        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(event) => {
                    //println!("Event detected: {:?}", event);
                    if let Event {
                        kind: EventKind::Access(AccessKind::Close(AccessMode::Write)),
                        ..
                    } = event {
                        //println!("write detected on paths: [{:?}]", event.paths);
                        if event.paths.contains(&list_file) {
                            //println!("list.yaml edited");
                            channel.send(Signal::Reload).expect("could not send number");
                        }
                    }
                },
                Err(e) => {
                    println!("Error resolving event: {:?}", e);
                }
            }
        }).unwrap();
    
        let _watch_thread = thread::spawn(move || {
            watcher.watch(&watch_file, RecursiveMode::Recursive).unwrap();
            thread::park();
        });
    }