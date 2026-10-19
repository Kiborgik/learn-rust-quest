//! Test helper (given, don't change): a tiny HTTP server on 127.0.0.1 for
//! the tests. It records every request and answers with your handler.
//! No internet is used.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    /// Path including the query string, like "/heroes?class=mage".
    pub path: String,
    /// Header names are lowercase.
    pub headers: HashMap<String, String>,
    pub body: String,
}

type Handler = Arc<dyn Fn(&Request) -> (u16, String) + Send + Sync>;

pub struct MockServer {
    pub url: String,
    pub requests: Arc<Mutex<Vec<Request>>>,
}

impl MockServer {
    /// Starts a server that answers every request with `handler(request)`
    /// = (status code, JSON or text body).
    pub async fn start(handler: impl Fn(&Request) -> (u16, String) + Send + Sync + 'static) -> Self {
        Self::start_with_delay(Duration::ZERO, handler).await
    }

    /// Like `start`, but waits `delay` before answering each request.
    pub async fn start_with_delay(
        delay: Duration,
        handler: impl Fn(&Request) -> (u16, String) + Send + Sync + 'static,
    ) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let handler: Handler = Arc::new(handler);
        let log = requests.clone();
        tokio::spawn(async move {
            while let Ok((mut sock, _)) = listener.accept().await {
                let handler = handler.clone();
                let log = log.clone();
                tokio::spawn(async move {
                    let mut buf = Vec::new();
                    let mut tmp = [0u8; 4096];
                    let end = loop {
                        let n = sock.read(&mut tmp).await.unwrap_or(0);
                        if n == 0 {
                            return;
                        }
                        buf.extend_from_slice(&tmp[..n]);
                        if let Some(end) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                            break end;
                        }
                    };
                    let head = String::from_utf8_lossy(&buf[..end]).to_string();
                    let mut lines = head.lines();
                    let mut first = lines.next().unwrap_or("").split(' ');
                    let method = first.next().unwrap_or("").to_string();
                    let path = first.next().unwrap_or("").to_string();
                    let headers: HashMap<String, String> = lines
                        .filter_map(|l| l.split_once(':'))
                        .map(|(k, v)| (k.trim().to_lowercase(), v.trim().to_string()))
                        .collect();
                    let len: usize = headers
                        .get("content-length")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0);
                    let mut body = buf[end + 4..].to_vec();
                    while body.len() < len {
                        let n = sock.read(&mut tmp).await.unwrap_or(0);
                        if n == 0 {
                            break;
                        }
                        body.extend_from_slice(&tmp[..n]);
                    }
                    let req = Request {
                        method,
                        path,
                        headers,
                        body: String::from_utf8_lossy(&body).to_string(),
                    };
                    log.lock().unwrap().push(req.clone());
                    let (status, reply) = handler(&req);
                    if !delay.is_zero() {
                        tokio::time::sleep(delay).await;
                    }
                    let resp = format!(
                        "HTTP/1.1 {status} OK\r\ncontent-type: application/json\r\n\
                         content-length: {}\r\nconnection: close\r\n\r\n{reply}",
                        reply.len()
                    );
                    let _ = sock.write_all(resp.as_bytes()).await;
                    let _ = sock.shutdown().await;
                });
            }
        });
        MockServer { url, requests }
    }

    pub fn requests(&self) -> Vec<Request> {
        self.requests.lock().unwrap().clone()
    }
}
