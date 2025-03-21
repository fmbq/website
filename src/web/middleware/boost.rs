use poem::{http::StatusCode, Endpoint, IntoResponse, Middleware, Request, Response, Result};

pub struct BoostMiddleware;

impl<E, R> Middleware<E> for BoostMiddleware
where
    R: IntoResponse,
    E: Endpoint<Output = R>,
{
    type Output = BoostMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        BoostMiddlewareImpl { inner: ep }
    }
}

pub struct BoostMiddlewareImpl<E> {
    inner: E,
}

impl<E, R> Endpoint for BoostMiddlewareImpl<E>
where
    R: IntoResponse,
    E: Endpoint<Output = R>,
{
    type Output = Response;

    async fn call(&self, request: Request) -> Result<Self::Output> {
        let boosted = request.header("hx-boosted").is_some();

        let mut response = self
            .inner
            .call(request)
            .await
            .map(IntoResponse::into_response)?;

        if boosted && response.status() == 404 {
            response.set_status(StatusCode::OK);
        }

        Ok(response)
    }
}
