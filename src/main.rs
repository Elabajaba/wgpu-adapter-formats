const HELP: &str = "\
App
USAGE:
  app [OPTIONS]
FLAGS:
  -h, --help            Prints help information
OPTIONS:
  -i, --igpu true|false       Choose whether to use the integrated GPU or not
";

#[derive(Debug)]
struct AppArgs {
    use_igpu: Option<bool>,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    pollster::block_on(run(args));
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        use_igpu: pargs.opt_value_from_str(["-i", "--igpu"])?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

async fn run(args: AppArgs) {
    let power_preference = match args.use_igpu {
        Some(b) => match b {
            true => wgpu::PowerPreference::LowPower,
            false => wgpu::PowerPreference::HighPerformance,
        },
        None => wgpu::PowerPreference::HighPerformance,
    };

    println!("Power preference: {:?}", power_preference);

    let adapter = wgpu::Instance::new(wgpu::Backends::DX12)
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference,
            ..Default::default()
        })
        .await
        .unwrap();

    println!("{:?}", adapter.get_info());
    println!("{:?}", adapter.features());

    let (_device, _queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                // features: wgpu::Features::empty(),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
}
