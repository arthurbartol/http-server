enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

struct Request {
    route: String,
    request: Option<String>,
    method: Method,
}
