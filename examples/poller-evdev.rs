use input_device::*;
use poller::{EventContext, Events, Poller};
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dnumerate all input devices in the system.
    let devs = enumerate_shareable();
    // Create the Poller.
    let mut poller = Poller::new()?;
    // Add stdin to the watching list of the Poller.
    poller.add(0, Events::new().read(), None)?;
    // Add each dev to the watching list of the Poller.
    for dev in &devs {
        // Enable non-blocking access.
        dev.lock().unwrap().set_non_blocking(true);
        // Add to the Poller.
        poller.add(
            dev.lock().unwrap().as_raw_fd(),
            Events::new().read(),
            Some(Arc::clone(&dev) as EventContext),
        )?;
    }

    println!("Press any key to exit ...");

    'outer: loop {
        // Pull all events with 1 seconds timeout.
        let events = poller.pull_events(1000)?;
        for (_fd, _events, _ctx) in events.iter() {
            // Exit loop if press any key.
            if _fd == &0 {
                break 'outer;
            }
            // Use EventContext to processing the event.
            if let Some(x) = _ctx {
                if let Some(dev) = x.downcast_ref::<Mutex<Device>>() {
                    // println!("id={}", dev.lock().unwrap().id());
                    for ev in dev.lock().unwrap().events() {
                        println!("ev={:?}", ev);
                    }
                }
            }
        }
    }

    Ok(())
}
