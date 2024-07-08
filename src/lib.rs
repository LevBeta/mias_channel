//! # Mias
//!
//! `MiasChannel` is a library for creating channels that can be used to send and receive messages between tasks.
//!
//! Allowing for easy bi-directional communication between tasks. (Normal channel)
//! And allowing for oneshot responses to be sent back to the sender. (Responder channel)
//!
//! `MiasChannel` at the moment only supports tokio. By wrapping the tokio channels in a more user friendly API for bi-directional communication.
//! It has a goal of supporting more channels in the future easly by using features to enable different channel implementations.
//!
//! ## Responder Channel
//!
//! The responder channel is a channel that allows for a request to be sent to a receiver and for a response to be sent back to the sender.
//! This is useful for when you want to send a request to a task and get a response back.
//! The responder channel is implemented using a normal channel and oneshot channels.
//!
//! Is built on top of the normal channel and oneshot channels. In the future i want to replace the oneshot channels with a more efficient implementation.
//!
//! ### Responder Channel Example:
//! ```rust
//! use mias_channel::responder_channel;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let (tx, mut rx) = responder_channel::<i64, f64>(10);
//!
//!     tokio::task::spawn(async move {
//!         while let Some((req, tx)) = rx.recv().await {
//!             tx.send(req as f64).unwrap();
//!         }
//!     });
//!
//!     let res = tx.send(10).await;
//! }
//! ```
//!

/// Implements the normal channel.
pub mod normal;

/// Implements the responder channel.
pub mod responder;

/// Errors that can occur when using the library.
pub mod error;

/// Re-exports the Responder channel for easier access.
pub use responder::responder_channel;

/// Re-exports the normal channel for easier access.
pub use normal::channel;
