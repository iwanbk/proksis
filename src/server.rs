use pingora::server::configuration::Opt;
use pingora::server::Server;
use pingora::services::Service;

use crate::proksis::proksis_service;

pub fn run(opt: Option<Opt>) {
    let mut server = Server::new(opt).unwrap();
    server.bootstrap();

    let mut proksis_service = proksis_service();
    proksis_service.add_tcp("127.0.0.1:6379");

    let services: Vec<Box<dyn Service>> = vec![
        Box::new(proksis_service),
    ];

    server.add_services(services);
    server.run_forever();
}