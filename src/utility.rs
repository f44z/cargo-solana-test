pub const TEST_TEMPLATE: &str = r#"#![cfg(feature = "test-bpf")]

use solana_poc_async::*;

#[tokio::test]
async fn poc() {
    assert_eq!(1, 1);
}
"#;
