use wgpu;

pub async fn adapter_info() -> Result<(), String> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await;

    match adapter {
        Some(adapter) => {
            tracing::info!("Selected adapter: {:?}", adapter.get_info());
            Ok(())
        }
        None => {
            tracing::error!("Failed to find a suitable GPU adapter");
            Err("No suitable GPU adapter found".to_string())
        }
    }
}
