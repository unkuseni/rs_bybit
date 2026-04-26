use crate::prelude::*;
use log::trace;
use std::time::Duration;

/// The main client struct that wraps the reqwest client.
///
/// It stores the API key, secret key, and host to make requests to the Bybit API.
#[derive(Clone)]
pub struct Client {
    /// The API key for the Bybit account.
    pub api_key: Option<String>,

    /// The secret key for the Bybit account.
    pub secret_key: Option<String>,

    /// The host to make requests to.
    pub host: String,

    /// The reqwest client that makes the HTTP requests.
    pub inner_client: ReqwestClient,
}

impl Client {
    /// Create a new instance of `Client`.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for the Bybit account. It can be `None` if the client is not for authenticated requests.
    /// * `secret_key` - The secret key for the Bybit account. It can be `None` if the client is not for authenticated requests.
    /// * `host` - The host to make requests to.
    ///
    /// # Returns
    ///
    /// A new instance of `Client`.
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        // Create a new instance of the reqwest client.
        let inner_client = ReqwestClient::builder()
            .build()
            .expect("Failed to build reqwest client");

        // Create a new instance of `Client` with the provided arguments.
        Client {
            api_key,
            secret_key,
            host,
            inner_client,
        }
    }

    /// Makes an unsigned HTTP GET request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint to make the request to.
    /// * `request` - The query string to append to the URL.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response deserialized to the specified type `T`.
    pub async fn get<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T, BybitError> {
        // Construct the full URL
        let mut url = format!("{}/{}", self.host, endpoint.as_ref());
        // If there is a query string, append it to the URL
        if let Some(request) = request {
            if !request.is_empty() {
                url.push('?');
                url.push_str(&request);
            }
        }

        // Make the request using the reqwest client
        let response = self.inner_client.get(url).send().await?;
        // Handle the response using the `handler` method
        self.handler(response).await
    }

    /// Makes a signed HTTP GET request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint to make the request to.
    /// * `recv_window` - The receive window for the request in milliseconds.
    /// * `request` - The query string to append to the URL.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response deserialized to the specified type `T`.
    pub async fn get_signed<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        recv_window: u64,
        request: Option<String>,
    ) -> Result<T, BybitError> {
        // Construct the full URL
        let mut url: String = format!("{}/{}", self.host, endpoint.as_ref());
        // If there is a query string, append it to the URL
        let query_string = request.unwrap_or_default();
        if !query_string.is_empty() {
            url.push_str(format!("?{}", query_string).as_str());
        }

        // Sign the request, passing the query string for signature
        // The request is signed with the API secret key and requires
        // the `recv_window` for the request to be within the specified timeframe.
        let headers = self.build_signed_headers(false, true, recv_window, Some(&query_string))?;

        // Make the signed HTTP GET request
        let client = &self.inner_client;
        let response = client.get(url.as_str()).headers(headers).send().await?;

        // Handle the response
        self.handler(response).await
    }

    /// Makes an unsigned HTTP POST request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint to make the request to.
    /// * `request` - The query string to append to the URL. Only used if provided.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response deserialized to the specified type `T`.
    pub async fn post<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T, BybitError> {
        // Construct the URL by appending the base host and endpoint to it
        let mut url: String = format!("{}/{}", self.host, endpoint.as_ref());

        // If a request is provided, append it to the URL as a query string
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        // Get a reference to the inner client
        let client = &self.inner_client;

        // Send the POST request to the constructed URL
        let response = client.post(url.as_str()).send().await?;

        // Handle the response by passing it to the handler method
        self.handler(response).await
    }

    /// Makes a signed HTTP POST request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint to make the request to.
    /// * `recv_window` - The receive window for the request in milliseconds.
    /// * `raw_request_body` - The raw request body to sign. Only used if provided.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response deserialized to the specified type `T`.
    pub async fn post_signed<T: DeserializeOwned + Send + 'static>(
        &self,
        endpoint: API,
        recv_window: u64,
        raw_request_body: Option<String>,
    ) -> Result<T, BybitError> {
        // Construct the full URL
        let url = format!("{}{}", self.host, endpoint.as_ref());

        // Sign the request, passing the raw request body for signature
        let headers =
            self.build_signed_headers(true, true, recv_window, raw_request_body.as_deref())?;

        // Make the signed HTTP POST request
        let client = &self.inner_client;
        let body = raw_request_body.unwrap_or_default();
        let response = client.post(url).headers(headers).body(body).send().await?;

        // Handle the response
        self.handler(response).await
    }

    /// Builds the signed headers for an HTTP request.
    ///
    /// # Arguments
    ///
    /// * `content_type` - Whether to include the `Content-Type` header.
    /// * `signed` - Whether to include the signature in the headers.
    /// * `recv_window` - The receive window for the request in milliseconds.
    /// * `request` - The request body to sign.
    ///
    /// # Returns
    ///
    /// A `Result` containing the signed headers.
    fn build_signed_headers(
        &self,
        content_type: bool,
        signed: bool,
        recv_window: u64,
        request: Option<&str>,
    ) -> Result<HeaderMap, BybitError> {
        // Initialize the custom headers map
        let mut custom_headers = HeaderMap::new();
        // Set the User-Agent header
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("bybit-rs"));
        // Get the current timestamp
        let timestamp = get_timestamp().to_string();
        // Get the receive window
        let window = recv_window.to_string();
        // Sign the request
        let signature = self.sign_message(&timestamp, &window, request)?;

        // Set the headers
        let signature_header = HeaderName::from_static("x-bapi-sign");
        let api_key_header = HeaderName::from_static("x-bapi-api-key");
        let timestamp_header = HeaderName::from_static("x-bapi-timestamp");
        let recv_window_header = HeaderName::from_static("x-bapi-recv-window");

        if signed {
            // Insert the signature header
            custom_headers.insert(signature_header, HeaderValue::from_str(&signature)?);
            // Insert the API key header
            if let Some(ref key) = self.api_key {
                custom_headers.insert(api_key_header, HeaderValue::from_str(key)?);
            }
        }
        // Insert the timestamp header
        custom_headers.insert(timestamp_header, HeaderValue::from_str(&timestamp)?);
        // Insert the receive window header
        custom_headers.insert(recv_window_header, HeaderValue::from_str(&window)?);
        // Insert the Content-Type header if required
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        // Return the signed headers
        Ok(custom_headers)
    }

    fn mac_from_secret_key(&self) -> Result<Hmac<Sha256>, BybitError> {
        let secret = self.secret_key.as_deref().unwrap_or("");
        Hmac::<Sha256>::new_from_slice(secret.as_bytes())
            .map_err(|e| BybitError::Base(format!("Failed to create Hmac, error: {:?}", e)))
    }

    /// Signs a POST request message.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The timestamp of the request.
    /// * `recv_window` - The receive window of the request.
    /// * `request` - The request body as an optional string.
    ///
    /// # Returns
    ///
    /// The signed message as a hex-encoded string.
    ///
    /// # Description
    ///
    /// This function takes the timestamp, receive window, and an optional request body as input.
    /// It creates a string by concatenating the timestamp, API key, and receive window.
    /// If a request body is provided, it appends it to the sign message.
    /// The function then uses the HMAC-SHA256 algorithm to sign the message.
    /// The result is hex-encoded and returned as a string.
    fn sign_message(
        &self,
        timestamp: &str,
        recv_window: &str,
        request: Option<&str>,
    ) -> Result<String, BybitError> {
        // Create a new HMAC SHA256 instance with the secret key
        let mut mac = self.mac_from_secret_key()?;

        // Create the sign message by concatenating the timestamp, API key, and receive window
        let api_key = self.api_key.as_deref().unwrap_or("");
        let mut sign_message = format!("{}{}{}", timestamp, api_key, recv_window);

        // If a request body is provided, append it to the sign message
        if let Some(req) = request {
            sign_message.push_str(req);
        }

        // Update the MAC with the sign message
        mac.update(sign_message.as_bytes());

        // Finalize the MAC and encode the result as a hex string
        let hex_signature = hex_encode(mac.finalize().into_bytes());

        Ok(hex_signature)
    }

    /// Internal function to handle the response from a HTTP request.
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response from the request.
    ///
    /// # Returns
    ///
    /// The result of deserializing the response body into a specific type.
    /// Returns `Ok(T)` if the response status is `StatusCode::OK`,
    /// returns `Err(BybitError::BybitError(BybitContentError))` if the response
    /// status is `StatusCode::BAD_REQUEST`, returns `Err(BybitError::InternalServerError)`
    /// if the response status is `StatusCode::INTERNAL_SERVER_ERROR`,
    /// returns `Err(BybitError::ServiceUnavailable)` if the response status is
    /// `StatusCode::SERVICE_UNAVAILABLE`, returns `Err(BybitError::Unauthorized)`
    /// if the response status is `StatusCode::UNAUTHORIZED`, and returns
    /// `Err(BybitError::StatusCode(status))` if the response status is any other
    /// value.
    async fn handler<T: DeserializeOwned + Send + 'static>(
        &self,
        response: ReqwestResponse,
    ) -> Result<T, BybitError> {
        // Match the status code of the response
        let status = response.status();
        let response_text = &response.text().await.map_err(BybitError::from)?;
        match status {
            StatusCode::OK => {
                // Deserialize the entire response into BybitApiResponseOpaquePayload first.
                let wrapper: BybitApiResponseOpaquePayload = serde_json::from_str(response_text)
                    .map_err(|e| {
                        BybitError::Base(format!(
                            "Failed to parse response wrapper: {} | Response: {}",
                            e, response_text
                        ))
                    })?;

                match wrapper.ret_code {
                    0 => {
                        // If ret_code is 0, the operation was successful at the API level.
                        // The actual data is in `response_json.result` (a serde_json::Value).
                        // Deserialize this `Value` into the target type `T`.
                        serde_json::from_str(response_text).map_err(|e| {
                            BybitError::Base(format!(
                                "Failed to parse full response into {}: {} | Response: {}",
                                type_name::<T>(),
                                e,
                                response_text
                            ))
                        })
                    }
                    _ => {
                        // If ret_code is non-zero, it's an API error.
                        Err(BybitError::BybitError(BybitContentError {
                            code: wrapper.ret_code,
                            msg: wrapper.ret_msg,
                        }))
                    }
                }
            }
            StatusCode::BAD_REQUEST => {
                let error: BybitContentError =
                    serde_json::from_str(response_text).map_err(BybitError::from)?;
                Err(BybitError::BybitError(error))
            }
            StatusCode::INTERNAL_SERVER_ERROR => Err(BybitError::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(BybitError::ServiceUnavailable),
            StatusCode::UNAUTHORIZED => Err(BybitError::Unauthorized),
            status => Err(BybitError::StatusCode(status.as_u16())),
        }
    }

    /// Connects to the Bybit WebSocket endpoint and sends an authentication message.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The WebSocket endpoint to connect to.
    /// * `request_body` - An optional request body to send after authenticating.
    /// * `private` - A boolean indicating whether to send the authentication message.
    /// * `alive_dur` - An optional duration in minutes for the auth session lifetime.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `WebSocketStream` if the connection and authentication
    /// are successful, or a `BybitError` if an error occurs.
    pub async fn wss_connect(
        &self,
        endpoint: WebsocketAPI,
        request_body: Option<String>,
        private: bool,
        alive_dur: Option<u16>,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, BybitError> {
        let url = WsUrl::parse(&format!("{}{}", self.host, endpoint.as_ref()))?;

        // Calculate the expiration time for the authentication message
        let expires = get_timestamp() + (alive_dur.unwrap_or(9) as u64 * 60_000);

        // Sign the challenge string for the auth message
        let signature = self.sign_auth_challenge(expires)?;

        let uuid = generate_random_uid(5);

        let (mut ws_stream, _) = connect_async(url.as_ref())
            .await
            .map_err(BybitError::Tungstenite)?;

        if private {
            self.send_auth(&mut ws_stream, uuid, expires, &signature)
                .await?;
        }

        // Send the request body if present and non-empty
        if let Some(body) = &request_body {
            if !body.is_empty() {
                ws_stream.send(WsMessage::Text(body.clone().into())).await?;
            }
        }

        Ok(ws_stream)
    }

    /// Builds the HMAC-SHA256 signature for the WebSocket auth challenge.
    fn sign_auth_challenge(&self, expires: u64) -> Result<String, BybitError> {
        let mut mac = self.mac_from_secret_key()?;
        mac.update(format!("GET/realtime{expires}").as_bytes());
        Ok(hex_encode(mac.finalize().into_bytes()))
    }

    /// Sends an authentication message over the WebSocket and waits for the response.
    async fn send_auth(
        &self,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
        uuid: String,
        expires: u64,
        signature: &str,
    ) -> Result<(), BybitError> {
        let api_key = self.api_key.as_deref().unwrap_or("");

        let auth_msg = json!({
            "req_id": uuid,
            "op": "auth",
            "args": [api_key, expires, signature]
        });

        ws_stream
            .send(WsMessage::Text(auth_msg.to_string().into()))
            .await?;

        let auth_response =
            tokio::time::timeout(Duration::from_secs(5), wait_for_auth_response(ws_stream))
                .await
                .map_err(|_| BybitError::Base("Authentication timeout".into()))??;

        if auth_response.is_failure() {
            return Err(BybitError::Base(format!(
                "Authentication failed: {} (code: {:?})",
                auth_response.ret_msg(),
                auth_response.error_code()
            )));
        }

        trace!(
            "WebSocket authentication successful: {}",
            auth_response.conn_id()
        );

        Ok(())
    }
}

/// Waits for and parses an authentication response from a WebSocket stream.
async fn wait_for_auth_response(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Result<AuthResponse, BybitError> {
    use futures::StreamExt;

    loop {
        match ws_stream.next().await {
            Some(Ok(WsMessage::Text(msg))) => {
                match serde_json::from_str::<AuthResponse>(&msg) {
                    Ok(auth_response) => return Ok(auth_response),
                    Err(_) => {
                        // Not an auth response — it might be a subscription confirmation
                        // that arrived before the auth response. Keep waiting.
                        trace!("Ignoring non-auth message while waiting for auth: {msg}");
                        continue;
                    }
                }
            }
            Some(Ok(_)) => {
                // Binary or other non-text message, ignore
                continue;
            }
            Some(Err(e)) => return Err(BybitError::Tungstenite(e)),
            None => {
                return Err(BybitError::Base(
                    "WebSocket stream closed while waiting for authentication".into(),
                ));
            }
        }
    }
}
