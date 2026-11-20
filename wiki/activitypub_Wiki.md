# SigmaOS-activitypub Component Wiki

## Overview
SigmaOS-activitypub provides ActivityPub federation support for the SigmaOS operating system. It demonstrates the decentralized social networking protocol implementation within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic protocol initialization.

## Principles
- Single Responsibility
- ActivityPub Abstraction
- Fail-Fast on Protocol Initialization Errors
- Deterministic Protocol Setup
- Clear Abstraction Over ActivityPub APIs
- Composition over Inheritance for protocol extensions

## Architecture
SigmaOS-activitypub follows an ActivityPub pattern:
- **Protocol Core**: Provides ActivityPub object models, inbox handling, and outbox delivery primitives
- **Federation Management**: Supports actor discovery, relationship handling, and cross-server communication
- **Activity Processing**: Supports Create, Read, Update, Delete (CRUD) operations on activities
- **Error Handling**: Fail-fast on invalid protocol configurations or unsupported federation types

## API
The component provides the following public APIs:
- `federation::actor::create(id, inbox, outbox)` - Create a new ActivityPub actor
- `federation::deliver(activity, recipient)` - Deliver an activity to a remote recipient
- `federation::inbox::process(message)` - Process an incoming activitypub message
- `federation::outbox::status()` - Get outbox delivery status and pending deliveries
- Error types for protocol failure scenarios or unsupported federation types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific ActivityPub configuration
2. **Act**: Execute the code under test (actor creation, message delivery)
3. **Assert**: Verify the expected ActivityPub results

Example:
```rust
#[test]
fn test_activitypub_actor_create_aaa() {
    // Arrange
    let actor_id = "https://sigmaos.example/users/alice";
    let inbox = "https://sigmaos.example/users/alice/inbox";
    let outbox = "https://sigmaos.example/users/alice/outbox";
    
    // Act
    let actor = federation::actor::create(actor_id, inbox, outbox);
    
    // Assert
    assert!(actor.is_ok());
    assert!(actor.unwrap().is_valid());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Protocol initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all protocol failure scenarios
- Unit tests provided for actor creation and message delivery
- Integration tests for full ActivityPub federation with remote servers

## TODO
- Add support for additional ActivityPub vocabulary types and extensions
- Expand federation management capabilities (outbound federation, connection pooling)
- Add activity signing and verification features
- Create integration tests with remote ActivityPub servers (Mastodon, Pleroma)
- Implement connection persistence and retry logic for delivery