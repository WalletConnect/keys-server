use crate::storage::{mongo::MongoStorage, Storage};

#[cfg(feature = "test-mongo")]
#[tokio::test]
async fn set_get() {
    let st = init_test_storage().await;

    st.set("key", "value").await.unwrap();
    let res = st.get("key").await.unwrap();

    assert_matches!(res, Some(_));
    assert_eq!(res.unwrap(), "value");
}

#[cfg(feature = "test-mongo")]
#[tokio::test]
async fn set_get_nonexistent() {
    let st = init_test_storage().await;

    st.set("key1", "value").await.unwrap();

    let v = st.get("key10").await;
    assert_matches!(v, Ok(None));
}

#[cfg(feature = "test-mongo")]
#[tokio::test]
async fn set_remove_get() {
    let st = init_test_storage().await;

    let v = st.set("key1", "value1").await;
    assert_matches!(v, Ok(()));
    let v = st.set("key2", "value2").await;
    assert_matches!(v, Ok(()));

    st.remove("key2").await.unwrap();

    let l = st.count().await;
    assert_matches!(l, Ok(1));

    let v = st.get("key1").await;
    assert_matches!(v, Ok(Some(_)));
    assert_eq!(v.unwrap().unwrap(), "value1");

    let v = st.get("key2").await;
    assert_matches!(v, Ok(None));
}

#[cfg(feature = "test-mongo")]
#[tokio::test]
async fn clear() {
    let st = init_test_storage().await;

    let _v = st.set("key1", "value1").await;
    let _v = st.set("key2", "value2").await;

    let l = st.count().await;
    assert_matches!(l, Ok(2));

    let v = st.clear().await;
    assert_matches!(v, Ok(_));

    let l = st.count().await;
    assert_matches!(l, Ok(0));
}

#[cfg(feature = "test-mongo")]
#[inline]
async fn init_test_storage() -> MongoStorage {
    MongoStorage::new("mongodb://localhost:27017", "keyserver")
        .await
        .expect("failed to connect to mongodb instance")
}
