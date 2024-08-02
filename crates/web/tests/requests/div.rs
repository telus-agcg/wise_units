
use loco_rs::testing;
use serial_test::serial;
use wise_units_web::app::App;

#[tokio::test]
#[serial]
async fn can_post_div_root() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let payload = serde_json::json!({
            "lhs": {
                "value": 10,
                "unit": "km3"
            },
            "rhs": {
                "value": 5,
                "unit": "km2"
            }
        });

        let expected = serde_json::json!({
            "value": 2.0,
            "unit": "km"
        });
        let res = request.post("/api/div").json(&payload).await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), serde_json::to_string(&expected).unwrap());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_not_get_request_root() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/div").await;
        assert_eq!(res.status_code(), 405);
    })
    .await;
}

