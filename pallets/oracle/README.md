
# Oracle Module

This is a dummy oracle module which demonstrates how to write a simple Substrate pallet.

## Overview

This module demonstrates:

- How to write a Substrate pallet, including hooks, storage, events, etc.
- How to write unit tests for a pallet
- How to benchmark a pallet

## Interface

### Dispatchable Functions

- `submit_event` - Submit a new feed event, only callable by the sudo account

### Storage

- `Feed` - A bounded vector of feed events

### Types

- `FeedEvent` - A feed event type is simply a struct with a value as arbitrary bytes and a block number tracking when the event was recorded

### Events

- `NewFeedEvent` - A new feed event is recorded
