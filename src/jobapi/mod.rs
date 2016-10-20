use valico::json_dsl;
use hyper::status::StatusCode;
use rustless::{Application, Api, Nesting, Versioning};
use rustless::json::ToJson;
use iron::Iron;
pub struct JobApi {
    server: String,
    container_only: bool,
}

impl JobApi {
    pub fn new(listen_on: String, force_sandbox_mode: bool) -> JobApi {
        JobApi {
            server: listen_on,
            container_only: force_sandbox_mode,
        }
    }

    pub fn start_server(&mut self) {
        // This is probably bad Rust but it works :D
        let server = self.server.to_owned();
        let api_meta = Api::build(|api| {
            api.prefix("api");

            // /api/run_task
            api.get("run_task", |endpoint| {

                endpoint.handle(|client, _params| {
                    println!("Request: /api/run_task");
                    client.text("Running Task".to_string())

                })
            });

            // /api/create_task
            api.get("create_task", |endpoint| {

                endpoint.handle(|client, _params| {
                    println!("Request: /api/create_task");
                    client.text("Creating Task".to_string())

                })
            });

        });

        // new Iron Server Application based on MIO
        let jobr_server = Application::new(api_meta);
        Iron::new(jobr_server).http(&server[..]).unwrap();
    }
}
