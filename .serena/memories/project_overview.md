# Project Overview - vmix-rs

## Purpose
vmix-rs is a Rust library for interacting with vMix (video mixing software) via TCP and HTTP APIs. The library provides both asynchronous and synchronous communication with vMix instances for real-time video production control.

## Key Features
- TCP communication with vMix instances (default port 8099)
- Real-time state updates via subscription model (TALLY, ACTS)
- XML state queries for full vMix configuration snapshots
- Function commands for direct control (CUT, FADE, input switching)
- Thread-based architecture with proper memory management
- Graceful connection handling and timeout management

## Repository Information
- **Author**: Shugo Kawamura (@FlowingSPDG)
- **License**: MIT
- **Repository**: https://github.com/FlowingSPDG/vmix-rs
- **Version**: 0.1.0 (Rust 2021 edition)

## Current Status
This is an active development project with a comprehensive architecture already in place. The codebase includes core functionality for vMix communication, with some TODO items including HTTP API addition and better XML handling.