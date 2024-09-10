use poem::{
    http::Uri, session::Session, web::Redirect, Endpoint, IntoResponse, Middleware, Request,
    Response, Result,
};

pub struct LoginCheckMiddleware;

impl<E, R> Middleware<E> for LoginCheckMiddleware
where
    R: IntoResponse,
    E: Endpoint<Output = R>,
{
    type Output = LoginCheckMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LoginCheckMiddlewareImpl { inner: ep }
    }
}

pub struct LoginCheckMiddlewareImpl<E> {
    inner: E,
}

impl<E, R> Endpoint for LoginCheckMiddlewareImpl<E>
where
    R: IntoResponse,
    E: Endpoint<Output = R>,
{
    type Output = Response;

    async fn call(&self, request: Request) -> Result<Self::Output> {
        // Check if the user is logged in.
        let session = request.extensions().get::<Session>().unwrap();
        if session.get::<String>("user-id").is_none() {
            // Redirect to login page.
            let redirect_uri = Uri::builder()
                .path_and_query(format!("/admin/login?redirect={}", request.original_uri()))
                .build()
                .unwrap();

            return Ok(Redirect::see_other(redirect_uri).into_response());
        }

        self.inner
            .call(request)
            .await
            .map(IntoResponse::into_response)
    }
}
