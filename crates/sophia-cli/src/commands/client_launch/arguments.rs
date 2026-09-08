#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ArgvStyle {
    Direct,
    Wrapper,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Options {
    pub program: String,
    pub arguments: Vec<String>,
    pub check_only: bool,
    pub style: ArgvStyle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DeviceSwitch {
    HardwareVideoDevicePath,
    RenderNodeOverride,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExplicitDevice {
    pub kind: DeviceSwitch,
    pub path: String,
}

/// Explicit paths still require physical identity validation by the launcher.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct AdaptedArguments {
    pub arguments: Vec<String>,
    pub explicit_devices: Vec<ExplicitDevice>,
}

impl Options {
    pub(super) fn parse(args: &[String]) -> Result<Self, String> {
        let separator = args
            .iter()
            .position(|arg| arg == "--")
            .ok_or("client-launch requires -- before PROGRAM")?;
        let mut adapter = false;
        let mut style = None;
        let mut check_only = false;
        for arg in &args[..separator] {
            match arg.as_str() {
                "--adapter=chromium" if !adapter => adapter = true,
                "--argv-style=direct" | "--argv-style=wrapper" if style.is_none() => {
                    style = Some(if arg == "--argv-style=direct" {
                        ArgvStyle::Direct
                    } else {
                        ArgvStyle::Wrapper
                    });
                }
                "--check-only" if !check_only => check_only = true,
                "--adapter=chromium"
                | "--argv-style=direct"
                | "--argv-style=wrapper"
                | "--check-only" => {
                    return Err(format!("duplicate client-launch option: {arg}"));
                }
                _ => return Err("unknown client-launch option".into()),
            }
        }
        if !adapter {
            return Err("client-launch requires --adapter=chromium".into());
        }
        let style = style.ok_or("client-launch requires --argv-style=direct|wrapper")?;
        let program = args
            .get(separator + 1)
            .filter(|arg| !arg.is_empty())
            .ok_or("client-launch requires PROGRAM after --")?
            .clone();
        let arguments = args[separator + 2..].to_vec();
        if program.contains('\0') || arguments.iter().any(|arg| arg.contains('\0')) {
            return Err("client-launch arguments cannot contain NUL".into());
        }
        let options = Self {
            program,
            arguments,
            check_only,
            style,
        };
        options.browser_start()?;
        Ok(options)
    }

    pub(super) fn adapt(&self, selected_device: &str) -> Result<AdaptedArguments, String> {
        if !std::path::Path::new(selected_device).is_absolute() || selected_device.contains('\0') {
            return Err("selected render device must be an absolute path without NUL".into());
        }
        let start = self.browser_start()?;
        let end = self.arguments[start..]
            .iter()
            .position(|arg| arg == "--")
            .map_or(self.arguments.len(), |offset| start + offset);
        let mut explicit_devices: Vec<ExplicitDevice> = Vec::new();
        for argument in &self.arguments[start..end] {
            for (name, kind) in [
                (
                    "hardware-video-device-path",
                    DeviceSwitch::HardwareVideoDevicePath,
                ),
                ("render-node-override", DeviceSwitch::RenderNodeOverride),
            ] {
                let canonical = format!("--{name}");
                let single_dash = format!("-{name}");
                if argument == &canonical
                    || argument == &single_dash
                    || argument
                        .strip_prefix(&single_dash)
                        .is_some_and(|rest| rest.starts_with('='))
                {
                    return Err(format!("device selection requires --{name}=PATH"));
                }
                let Some(path) = argument
                    .strip_prefix(&canonical)
                    .and_then(|rest| rest.strip_prefix('='))
                else {
                    continue;
                };
                if path.is_empty() {
                    return Err(format!("device selection requires --{name}=PATH"));
                }
                if explicit_devices.iter().any(|device| device.kind == kind) {
                    return Err(format!("duplicate browser device selection: --{name}"));
                }
                explicit_devices.push(ExplicitDevice {
                    kind,
                    path: path.into(),
                });
            }
        }
        let mut arguments = self.arguments.clone();
        if explicit_devices.is_empty() {
            arguments.insert(end, format!("--render-node-override={selected_device}"));
        }
        Ok(AdaptedArguments {
            arguments,
            explicit_devices,
        })
    }

    fn browser_start(&self) -> Result<usize, String> {
        match self.style {
            ArgvStyle::Direct => Ok(0),
            ArgvStyle::Wrapper => self
                .arguments
                .iter()
                .position(|arg| arg == "--")
                .map(|separator| separator + 1)
                .ok_or_else(|| "wrapper argv requires -- before browser arguments".into()),
        }
    }
}
