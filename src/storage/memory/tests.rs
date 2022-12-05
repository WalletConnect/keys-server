use std::assert_matches::assert_matches;
use crate::storage::memory::MemoryStorage;
use crate::storage::Storage;

#[tokio::test]
async fn set_get() {
    let st = MemoryStorage::default();

    let v = st.set("key", "value").await;
    assert_matches!(v, Ok(None));

    let l = st.count().await;
    assert_matches!(l, Ok(1));

    let v = st.get("key").await;
    assert_matches!(v, Ok(Some(_)));
    assert_eq!(v.unwrap().unwrap(), "value")
}

#[tokio::test]
async fn set_get_nonexistent() {
    let st = MemoryStorage::default();

    let v = st.set("key1", "value").await;
    assert_matches!(v, Ok(None));

    let l = st.count().await;
    assert_matches!(l, Ok(1));

    let v = st.get("key2").await;
    assert_matches!(v, Ok(None));
}

#[tokio::test]
async fn set_remove_get() {
    let st = MemoryStorage::default();

    let v = st.set("key1", "value1").await;
    assert_matches!(v, Ok(None));
    let v = st.set("key2", "value2").await;
    assert_matches!(v, Ok(None));

    let l = st.count().await;
    assert_matches!(l, Ok(2));

    let v = st.remove("key2").await;
    assert_matches!(v, Ok(Some(_)));
    assert_eq!(v.unwrap().unwrap(), "value2");

    let l = st.count().await;
    assert_matches!(l, Ok(1));

    let v = st.get("key1").await;
    assert_matches!(v, Ok(Some(_)));
    assert_eq!(v.unwrap().unwrap(), "value1");

    let v = st.get("key2").await;
    assert_matches!(v, Ok(None));
}

#[tokio::test]
async fn clear() {
    let st = MemoryStorage::default();

    let _v = st.set("key1", "value1").await;
    let _v = st.set("key2", "value2").await;

    let l = st.count().await;
    assert_matches!(l, Ok(2));

    let v = st.clear().await;
    assert_matches!(v, Ok(_));

    let l = st.count().await;
    assert_matches!(l, Ok(0));
}
