#[derive(Debug, Clone, Copy)]
pub struct Public;

impl actix_web::FromRequest for Public {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        _: &actix_web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        std::future::ready(Ok(Public))
    }
}

