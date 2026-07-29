pub async fn slow_operation() -> u32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_slow_operation_real_time() {
        // Normally this would take 10 seconds!
    }

    #[tokio::test(start_paused = true)]
    async fn test_slow_operation_paused() {
        let res = slow_operation().await;
        assert_eq!(res, 42);
        // This finishes instantly!
    }
}
