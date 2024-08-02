use loco_rs::testing;
use serial_test::serial;
use wise_units_web::app::App;

#[tokio::test]
#[serial]
async fn can_post_convert_root() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let payload = serde_json::json!({
            "lhs": {
                "value": 1,
                "unit": "km2"
            },
            "rhs": "m2"
        });

        let expected = serde_json::json!({
            "value": 1_000_000.0,
            "unit": "m2"
        });
        let res = request.post("/api/convert").json(&payload).await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), serde_json::to_string(&expected).unwrap());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_not_get_request_root() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/convert").await;
        assert_eq!(res.status_code(), 405);
    })
    .await;
}
