mod arguments;
mod device;
mod discovery;

use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::{RecvTimeoutError, sync_channel};
use std::time::Duration;

use arguments::Options;

const DISCOVERY_TIMEOUT: Duration = Duration::from_secs(1);

struct PreparedLaunch {
    program: String,
    arguments: Vec<String>,
    device: device::RenderDevice,
    explicit: bool,
}

fn prepare(options: Options) -> Result<PreparedLaunch, String> {
    let device = discovery::discover()?;
    let adapted = options.adapt(device.path.to_str().ok_or("device_path_unrepresentable")?)?;
    for selection in &adapted.explicit_devices {
        device::validate_explicit(Path::new(&selection.path), &device)?;
    }
    device::validate_explicit(&device.path, &device)?;
    Ok(PreparedLaunch {
        program: options.program,
        arguments: adapted.arguments,
        device,
        explicit: !adapted.explicit_devices.is_empty(),
    })
}

pub(crate) fn try_run(args: &[String]) -> Result<bool, Box<dyn std::error::Error>> {
    if args.first().map(String::as_str) != Some("client-launch") {
        return Ok(false);
    }
    let options = Options::parse(&args[1..])?;
    let check_only = options.check_only;
    let (sender, receiver) = sync_channel(1);
    // Blocking X11 and kernel discovery are bounded by this short-lived process.
    // The worker drops all descriptors before sending; only passive launch data crosses.
    let worker = std::thread::Builder::new()
        .name("client-device-discovery".into())
        .spawn(move || {
            let result = prepare(options);
            let _ = sender.send(result);
        })?;
    let result = match receiver.recv_timeout(DISCOVERY_TIMEOUT) {
        Ok(result) => result,
        Err(RecvTimeoutError::Timeout) => {
            eprintln!("sophia_client_launch schema=1 status=failed reason=discovery_timeout");
            // Joining a worker blocked on the server would defeat the launch deadline.
            // Exit also closes its resources in check-only mode.
            std::process::exit(1);
        }
        Err(RecvTimeoutError::Disconnected) => return Err("device_discovery_failed".into()),
    };
    worker.join().map_err(|_| "device_discovery_failed")?;
    let prepared = result?;
    let status = if prepared.explicit {
        "explicit"
    } else {
        "adapted"
    };
    let diagnostic = format!(
        "sophia_client_launch schema=1 adapter=chromium status={status} render_node={} device={}:{}",
        prepared.device.path.display(),
        prepared.device.major,
        prepared.device.minor,
    );
    if check_only {
        println!("{diagnostic}");
        return Ok(true);
    }
    eprintln!("{diagnostic}");
    let error = Command::new(prepared.program)
        .args(prepared.arguments)
        .exec();
    Err(format!("client_exec_failed: {}", error.kind()).into())
}
