# bb8-redis-cluster

A Async redis cluster connection pool (bb8).

## Example

```rust
#[tokio::main]
async fn main() {
    let manager = RedisConnectionManager::new(vec!["redis://localhost:1234"]).unwrap();
    let pool = bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap();

    for _ in 0..20 {
        let pool = pool.clone();
        tokio::spawn(async move {
            let mut conn = pool.get().await.unwrap();
            let reply: String = cmd("PING").query_async(&mut *conn).await.unwrap();
            assert_eq!("PONG", reply);
        });
    }
}
```
