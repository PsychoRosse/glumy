use clama::Event;

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatFrame {
    Success,
    Suboptimal,
    Unknown,
}
