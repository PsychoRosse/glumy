#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatFrame {
    Success,
    Suboptimal,
    Validation,
    Timeout,
    Outdated,
    Occluded,
    Lost,
}