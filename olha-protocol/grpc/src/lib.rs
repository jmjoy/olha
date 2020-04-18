pub mod http {
    tonic::include_proto!("olha.v1.http");

    pub mod service {
        tonic::include_proto!("olha.v1.http.service");
    }

    pub mod client {
        tonic::include_proto!("olha.v1.http.client");
    }
}
