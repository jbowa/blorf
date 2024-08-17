pub async fn adapter_info() {
    let adapter = {
        let instance = wgpu::Instance::default();
        instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap()
    };
    log::info!("Selected adapter: {:?}", adapter.get_info());
}
