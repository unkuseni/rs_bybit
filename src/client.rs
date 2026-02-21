use crate::prelude::*;
use log::trace;
use std::time::Duration;

/// The main client struct that wraps the reqwest client.
///
/// It stores the API key, secret key, and host to make requests to the Bybit API.
#[derive(Clone)]
pub struct Client {
    /// The API key for the Bybit account.
    pub api_key: String,

    /// The secret key for the Bybit account.
    pub secret_key: String,

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
            // Set the API key. If `api_key` is `None`, set it to an empty string.
            api_key: match api_key {
                Some(api_key) => api_key,
                None => "".into(),
            },
            // Set the secret key. If `secret_key` is `None`, set it to an empty string.
            secret_key: match secret_key {
                Some(secret_key) => secret_key,
                None => "".into(),
            },
            // Set the host.
            host,
            // Set the reqwest client.
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
        recv_window: u16,
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
        let headers = self.build_signed_headers(false, true, recv_window, Some(query_string))?;

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
        recv_window: u16,
        raw_request_body: Option<String>,
    ) -> Result<T, BybitError> {
        // Construct the full URL
        let url = format!("{}{}", self.host, endpoint.as_ref());

        // Sign the request, passing the raw request body for signature
        // The request is signed with the API secret key and requires
        // the `recv_window` for the request to be within the specified timeframe.
        let headers =
            self.build_signed_headers(true, true, recv_window, raw_request_body.clone())?;

        // Make the signed HTTP POST request
        let client = &self.inner_client;
        let response = client
            .post(url)
            .headers(headers)
            .body(raw_request_body.unwrap_or_default())
            .send()
            .await?;

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
    fn build_signed_headers<'str>(
        &self,
        content_type: bool,
        signed: bool,
        recv_window: u16,
        request: Option<String>,
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
            custom_headers.insert(
                signature_header,
                HeaderValue::from_str(&signature.to_owned())?,
            );
            // Insert the API key header
            custom_headers.insert(
                api_key_header,
                HeaderValue::from_str(&self.api_key.to_owned())?,
            );
        }
        // Insert the timestamp header
        custom_headers.insert(
            timestamp_header,
            HeaderValue::from_str(&timestamp.to_owned())?,
        );
        // Insert the receive window header
        custom_headers.insert(
            recv_window_header,
            HeaderValue::from_str(&window.to_owned())?,
        );
        // Insert the Content-Type header if required
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        // Return the signed headers
        Ok(custom_headers).map_err(BybitError::ReqError)
    }

    fn mac_from_secret_key(&self) -> Result<Hmac<Sha256>, BybitError> {
        Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())
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
        request: Option<String>,
    ) -> Result<String, BybitError> {
        // Create a new HMAC SHA256 instance with the secret key
        let mut mac = self.mac_from_secret_key()?;

        // Create the sign message by concatenating the timestamp, API key, and receive window
        let mut sign_message = format!("{}{}{}", timestamp, self.api_key, recv_window);

        // If a request body is provided, append it to the sign message
        if let Some(req) = request {
            sign_message.push_str(&req);
        }

        // Update the MAC with the sign message
        mac.update(sign_message.as_bytes());

        // Finalize the MAC and encode the result as a hex string
        let hex_signature = hex_encode(mac.finalize().into_bytes());

        Ok(hex_signature)
    }

    /// Internal function to sign a POST request message.
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
    fn _sign_post_message(
        &self,
        timestamp: &str,
        recv_window: &str,
        request: Option<String>,
    ) -> Result<String, BybitError> {
        // Create a new HMAC SHA256 instance with the secret key
        let mut mac = self.mac_from_secret_key()?;

        // Update the MAC with the timestamp
        mac.update(timestamp.as_bytes());
        // Update the MAC with the API key
        mac.update(self.api_key.as_bytes());
        // Update the MAC with the receive window
        mac.update(recv_window.as_bytes());
        // Update the MAC with the request body, if provided
        if let Some(req) = request {
            mac.update(req.as_bytes());
        }

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
    /// * `alive_dur` - An optional duration in seconds to set the `alive` field of the
    ///   authentication message to.
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
        // Construct the WebSocket URL
        let unparsed_url = format!("{}{}", self.host, endpoint.as_ref()).to_string();
        let url = WsUrl::parse(unparsed_url.as_str())?;

        // Calculate the expiration time for the authentication message
        let expiry_time = alive_dur.unwrap_or(9) as u64 * 1000 * 60;
        let expires = get_timestamp() + expiry_time;

        // Calculate the signature for the authentication message
        let mut mac = self.mac_from_secret_key()?;
        mac.update(format!("GET/realtime{expires}").as_bytes());
        let signature = hex_encode(mac.finalize().into_bytes());

        // Generate a random UUID for the request ID
        let uuid = generate_random_uid(5);

        // Connect to the WebSocket endpoint
        match connect_async(url.as_ref()).await {
            // If the connection is successful, send the authentication message
            Ok((mut ws_stream, _)) => {
                let auth_msg = json!({
                    "req_id": uuid,
                    "op": "auth",
                    "args": [self.api_key, expires, signature]
                });

                if private {
                    // Send the authentication message if `private` is true
                    ws_stream
                        .send(WsMessage::Text(auth_msg.to_string()))
                        .await?;

                    // Wait for authentication response with timeout
                    let auth_response = tokio::time::timeout(
                        Duration::from_secs(5),
                        wait_for_auth_response(&mut ws_stream, endpoint),
                    )
                    .await
                    .map_err(|_| BybitError::Base("Authentication timeout".to_string()))??;

                    // Check if authentication was successful
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
                }

                // Send the request body if it is not empty
                let request = request_body.unwrap_or_default();
                if !request.is_empty() {
                    ws_stream.send(WsMessage::Text(request)).await?;
                }
                Ok(ws_stream)
            }
            // If the connection fails, return a BybitError
            Err(err) => Err(BybitError::Tungstenite(err)),
        }
    }
}

/// Waits for and parses an authentication response from a WebSocket stream.
///
/// This function reads messages from the WebSocket stream until it receives
/// an authentication response, then parses it into an `AuthResponse`.
async fn wait_for_auth_response(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    endpoint: WebsocketAPI,
) -> Result<AuthResponse, BybitError> {
    use futures::StreamExt;

    loop {
        match ws_stream.next().await {
            Some(Ok(WsMessage::Text(msg))) => {
                // Try to parse as AuthResponse
                if let Ok(auth_response) = serde_json::from_str::<AuthResponse>(&msg) {
                    return Ok(auth_response);
                }

                // If it's not an auth response, check if it's a subscription response
                // (for public streams that don't require auth but send subscription confirmation)
                let value: Value = serde_json::from_str(&msg)
                    .map_err(|e| BybitError::Base(format!("Failed to parse message: {}", e)))?;

                // Check if this is a subscription response for public streams
                if let Some(op) = value.get("op").and_then(|v| v.as_str()) {
                    if op == "subscribe" {
                        // This is a subscription confirmation, not an auth response
                        // For public streams, this is expected
                        if endpoint == WebsocketAPI::TradeStream {
                            // Trade stream should send auth response, not subscribe
                            continue;
                        }
                        // For public non-trade streams, create a success response
                        let conn_id = value
                            .get("conn_id")
                            .or_else(|| value.get("connId"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");

                        return Ok(AuthResponse::PrivateAuth(PrivateAuthData::new(
                            true,
                            "subscription confirmed",
                            conn_id,
                            None,
                        )));
                    }
                }

                // If we get here, it's not an auth or subscribe response
                // Continue waiting
            }
            Some(Ok(_)) => {
                // Binary or other non-text message, ignore
                continue;
            }
            Some(Err(e)) => {
                return Err(BybitError::Tungstenite(e));
            }
            None => {
                return Err(BybitError::Base(
                    "WebSocket stream closed while waiting for authentication".to_string(),
                ));
            }
        }
    }
}
