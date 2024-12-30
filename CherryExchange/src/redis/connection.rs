use crate::CeXError::CexError::{CeXError, CeXErrorType, CeXStep};
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::time::Duration;

const CACHE_POOL_MAX_OPEN: u32 = 16;
const CACHE_POOL_MIN_IDLE: u32 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub fn connect(env: &serde_yaml::Value) -> Result<r2d2::Pool<RedisConnectionManager>, CeXError> {
    let redis_addr = env["redis"]["addr"].as_str().unwrap_or("redis://127.0.0.1/");
    let manager = RedisConnectionManager::new(redis_addr)
        .expect("Unable to create Redis Connection Manager");
    match r2d2::Pool::builder()
        .max_size(CACHE_POOL_MAX_OPEN)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .min_idle(Some(CACHE_POOL_MIN_IDLE))
        .build(manager)
    {
        Ok(pool) => Ok(pool),
        Err(e) => Err(CeXError::new(
            CeXStep::CeXRedis,
            CeXErrorType::ConnectError,
            e.to_string(),
        )),
    }
}

pub fn get_con(pool: &r2d2::Pool<RedisConnectionManager>) -> Result<r2d2::PooledConnection<RedisConnectionManager>, CeXError> {
    match pool.get_timeout(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)){
        Ok(con) => Ok(con),
        Err(e) => Err(CeXError::new(
            CeXStep::CeXRedis,
            CeXErrorType::ConnectError,
            e.to_string(),
        )),
    }
}
