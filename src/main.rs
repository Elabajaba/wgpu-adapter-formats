fn main() {
    pollster::block_on(run());
}

async fn run() {
    let adapter = wgpu::Instance::new(wgpu::Backends::DX12)
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
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
