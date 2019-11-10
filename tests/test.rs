// See https://rocket.rs/v0.4/guide/testing/#local-dispatching
#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use bot_dofus_orchestrator::rocket;

    #[test]
    fn get_tasks() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/tasks").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_task() {
        // Well get and post tests are identical ...
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/tasks")
            .header(ContentType::JSON)
            .body(r#"{ "task": "LoginToAccount" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.get(format!("/tasks/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/tasks").dispatch();
    }

    #[test]
    fn post_task() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/tasks")
            .header(ContentType::JSON)
            .body(r#"{ "task": "LoginToAccount" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.get(format!("/tasks/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/tasks").dispatch();
    }

    #[test]
    fn update_task() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/tasks")
            .header(ContentType::JSON)
            .body(r#"{ "task": "LoginToAccount" }"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let response = client
            .put(format!("/tasks/{}", id[3]))
            .header(ContentType::JSON)
            .body(r#"{ "task": "LoginToCharacter" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let mut response = client.get(format!("/tasks/{}", id[3])).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains("LoginToCharacter"));
        client.delete("/tasks").dispatch();
    }

    #[test]
    fn delete_task() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/tasks")
            .header(ContentType::JSON)
            .body(r#"{ "task": "LoginToCharacter" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.delete(format!("/tasks/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/tasks").dispatch();
    }

    #[test]
    fn delete_all() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        client.delete("/tasks").dispatch();
        let response = client
            .post("/tasks")
            .header(ContentType::JSON)
            .body(r#"{ "task": "LoginToCharacter" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response = client.delete("/tasks").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
