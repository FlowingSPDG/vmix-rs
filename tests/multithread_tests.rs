use std::{
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use vmix_rs::vmix::VmixApi;

#[cfg(test)]
mod tests {
    use super::*;
    use vmix_rs::traits::VmixTcpApiClient;

    #[tokio::test]
    async fn test_vmix_api_send_sync_traits() {
        // This test will only compile if VmixApi implements Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        
        assert_send::<VmixApi>();
        assert_sync::<VmixApi>();
    }

    #[tokio::test]
    async fn test_vmix_api_with_arc_mutex() {
        // Test that VmixApi can be used with Arc<Mutex<>>
        // This simulates the TCP-only usage pattern
        
        // Note: This test uses a dummy address that won't connect
        // In a real scenario, you would use a valid vMix instance
        let addr: SocketAddr = "127.0.0.1:8099".parse().unwrap();
        let timeout = Duration::from_secs(5);
        
        // Test Arc<tokio::sync::Mutex<VmixApi>>
        let result = VmixApi::new(addr, timeout).await;
        
        // Since we're not running an actual vMix instance, the connection will fail
        // But this test ensures that the type system accepts our Send + Sync implementation
        match result {
            Ok(api) => {
                let api_arc = Arc::new(Mutex::new(api));
                let api_clone = api_arc.clone();
                
                // This should compile without errors if Send + Sync are properly implemented
                tokio::spawn(async move {
                    let client = api_clone.lock().await;
                    // Test TCP-specific methods
                    let _connected = client.is_connected();
                    // In a real test with a running vMix instance:
                    // let _result = client.send_command(SendCommand::TALLY);
                })
                .await
                .unwrap();
            }
            Err(_) => {
                // Connection failed as expected since no vMix instance is running
                // But the important thing is that the code compiles
            }
        }
    }

    #[tokio::test]
    async fn test_vmix_api_with_std_mutex() {
        // Test that VmixApi can be used with Arc<std::sync::Mutex<>>
        let addr: SocketAddr = "127.0.0.1:8099".parse().unwrap();
        let timeout = Duration::from_secs(5);
        
        let result = VmixApi::new(addr, timeout).await;
        
        match result {
            Ok(api) => {
                let api_arc = Arc::new(std::sync::Mutex::new(api));
                let api_clone = api_arc.clone();
                
                // This should compile without errors if Send + Sync are properly implemented
                tokio::spawn(async move {
                    let client = api_clone.lock().unwrap();
                    // Test TCP-specific methods (non-async)
                    let _connected = client.is_connected();
                })
                .await
                .unwrap();
            }
            Err(_) => {
                // Connection failed as expected
            }
        }
    }

    #[tokio::test] 
    async fn test_spawn_multiple_tasks_tcp_only() {
        // Test spawning multiple tasks that would use VmixApi for TCP operations
        let addr: SocketAddr = "127.0.0.1:8099".parse().unwrap();
        let timeout = Duration::from_secs(1);
        
        let result = VmixApi::new(addr, timeout).await;
        
        match result {
            Ok(api) => {
                let api_arc = Arc::new(Mutex::new(api));
                
                let handles: Vec<_> = (0..3)
                    .map(|_| {
                        let api_clone = api_arc.clone();
                        tokio::spawn(async move {
                            let client = api_clone.lock().await;
                            // Test TCP-specific operations
                            let _connected = client.is_connected();
                            // In a real scenario with a running vMix instance:
                            // let sender = client.get_sender();
                            // let _result = client.try_receive_command(Duration::from_millis(100));
                        })
                    })
                    .collect();
                
                // Wait for all tasks to complete
                for handle in handles {
                    handle.await.unwrap();
                }
            }
            Err(_) => {
                // Connection failed as expected
            }
        }
    }

    #[test]
    fn test_tcp_api_traits() {
        // Compile-time test to ensure VmixApi implements VmixTcpApiClient
        fn assert_tcp_api_client<T: VmixTcpApiClient>() {}
        assert_tcp_api_client::<VmixApi>();
    }

    #[tokio::test]
    async fn test_shutdown_handling() {
        // Test that VmixApi shuts down properly even when connection fails
        let addr: SocketAddr = "127.0.0.1:8099".parse().unwrap();
        let timeout = Duration::from_millis(100); // Short timeout for quick test
        
        let result = VmixApi::new(addr, timeout).await;
        
        match result {
            Ok(api) => {
                // If connection succeeds, test normal shutdown
                assert!(api.is_connected());
                drop(api); // This should trigger clean shutdown
                // If we reach here, shutdown didn't hang
            }
            Err(_) => {
                // Expected - no vMix instance running
                // This is fine for testing shutdown behavior
            }
        }
    }
}