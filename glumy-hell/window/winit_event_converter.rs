use crate::{
    Button, ButtonStatus, Key, KeyStatus, KeyboardEvent, MouseEvent, MouseWheelEvent, WindowEvent,
};

use plam::{MousePosition, MouseScroll, MouseScrollDelta, SurfaceSize};

use winit::{event, keyboard};

pub struct WinitEventConverter;

impl WinitEventConverter {
    #[inline]
    pub fn from(event: event::WindowEvent) -> Option<WindowEvent> {
        match event {
            event::WindowEvent::CloseRequested => Some(WindowEvent::CloseRequested),
            event::WindowEvent::RedrawRequested => Some(WindowEvent::RedrawRequested),
            event::WindowEvent::SurfaceResized(size) => {
                let size = SurfaceSize::clamped(size.width, size.height);

                Some(WindowEvent::SurfaceResized(size))
            }
            event::WindowEvent::KeyboardInput { event, .. } => {
                let event = KeyboardEvent::new(
                    Self::key(event.physical_key)?,
                    Self::key_state(event.state),
                );

                Some(WindowEvent::KeyboardInput(event))
            }
            event::WindowEvent::PointerMoved { position, .. } => {
                let position =
                    MousePosition::new(position.x as f32, position.y as f32).unwrap_or_default();

                Some(WindowEvent::PointerMoved(position))
            }
            event::WindowEvent::PointerButton { button, state, .. } => {
                let event = MouseEvent::new(
                    Self::button(button.mouse_button()?)?,
                    Self::button_state(state),
                );

                Some(WindowEvent::MouseInput(event))
            }
            event::WindowEvent::MouseWheel { delta, .. } => match delta {
                event::MouseScrollDelta::LineDelta(x, y) => {
                    let delta = MouseScrollDelta::new(x, y).unwrap_or_default();
                    let event = MouseWheelEvent::from(MouseScroll::Lines(delta));

                    Some(WindowEvent::MouseWheel(event))
                }
                event::MouseScrollDelta::PixelDelta(delta) => {
                    let delta =
                        MouseScrollDelta::new(delta.x as f32, delta.y as f32).unwrap_or_default();
                    let event = MouseWheelEvent::from(MouseScroll::Pixels(delta));

                    Some(WindowEvent::MouseWheel(event))
                }
                _ => None,
            },
            _ => None,
        }
    }

    #[inline]
    fn key_state(state: event::ElementState) -> KeyStatus {
        match state {
            event::ElementState::Pressed => KeyStatus::Pressed,
            event::ElementState::Released => KeyStatus::Released,
        }
    }

    #[inline]
    fn button_state(state: event::ElementState) -> ButtonStatus {
        match state {
            event::ElementState::Pressed => ButtonStatus::Pressed,
            event::ElementState::Released => ButtonStatus::Released,
        }
    }

    #[inline]
    fn key(key: keyboard::PhysicalKey) -> Option<Key> {
        match key {
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyA) => Some(Key::A),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyB) => Some(Key::B),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyC) => Some(Key::C),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyD) => Some(Key::D),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyE) => Some(Key::E),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyF) => Some(Key::F),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyG) => Some(Key::G),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyH) => Some(Key::H),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyI) => Some(Key::I),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyJ) => Some(Key::J),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyK) => Some(Key::K),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyL) => Some(Key::L),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyM) => Some(Key::M),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyN) => Some(Key::N),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyO) => Some(Key::O),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyP) => Some(Key::P),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyQ) => Some(Key::Q),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyR) => Some(Key::R),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyS) => Some(Key::S),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyT) => Some(Key::T),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyU) => Some(Key::U),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyV) => Some(Key::V),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyW) => Some(Key::W),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyX) => Some(Key::X),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyY) => Some(Key::Y),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyZ) => Some(Key::Z),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit1) => Some(Key::One),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit2) => Some(Key::Two),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit3) => Some(Key::Three),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit4) => Some(Key::Four),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit5) => Some(Key::Five),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit6) => Some(Key::Six),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit7) => Some(Key::Seven),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit8) => Some(Key::Eight),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit9) => Some(Key::Nine),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Digit0) => Some(Key::Zero),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Backquote) => Some(Key::Backquote),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Backslash) => Some(Key::Backslash),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BracketLeft) => Some(Key::BracketLeft),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BracketRight) => Some(Key::BracketRight),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Comma) => Some(Key::Comma),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Equal) => Some(Key::Equal),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::IntlBackslash) => {
                Some(Key::IntlBackslash)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::IntlRo) => Some(Key::IntlRo),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::IntlYen) => Some(Key::IntlYen),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Minus) => Some(Key::Minus),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Period) => Some(Key::Period),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Quote) => Some(Key::Quote),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Semicolon) => Some(Key::Semicolon),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Slash) => Some(Key::Slash),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ShiftLeft) => Some(Key::LeftShift),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ShiftRight) => Some(Key::RightShift),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ControlLeft) => Some(Key::LeftCtrl),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ControlRight) => Some(Key::RightCtrl),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::AltLeft) => Some(Key::LeftAlt),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::AltRight) => Some(Key::RightAlt),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MetaLeft) => Some(Key::MetaLeft),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MetaRight) => Some(Key::MetaRight),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Fn) => Some(Key::Fn),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::FnLock) => Some(Key::FnLock),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ContextMenu) => Some(Key::ContextMenu),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Backspace) => Some(Key::Backspace),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::CapsLock) => Some(Key::Caps),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Enter) => Some(Key::Enter),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Escape) => Some(Key::Esc),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Tab) => Some(Key::Tab),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Space) => Some(Key::Space),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Delete) => Some(Key::Delete),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Insert) => Some(Key::Insert),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Home) => Some(Key::Home),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::End) => Some(Key::End),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::PageUp) => Some(Key::PageUp),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::PageDown) => Some(Key::PageDown),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::PrintScreen) => Some(Key::PrintScreen),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ScrollLock) => Some(Key::ScrollLock),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Pause) => Some(Key::Pause),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumLock) => Some(Key::NumLock),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Help) => Some(Key::Help),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ArrowUp) => Some(Key::UpArrow),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ArrowLeft) => Some(Key::LeftArrow),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ArrowDown) => Some(Key::DownArrow),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ArrowRight) => Some(Key::RightArrow),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad0) => Some(Key::Numpad0),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad1) => Some(Key::Numpad1),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad2) => Some(Key::Numpad2),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad3) => Some(Key::Numpad3),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad4) => Some(Key::Numpad4),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad5) => Some(Key::Numpad5),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad6) => Some(Key::Numpad6),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad7) => Some(Key::Numpad7),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad8) => Some(Key::Numpad8),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Numpad9) => Some(Key::Numpad9),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadAdd) => Some(Key::NumpadAdd),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadSubtract) => {
                Some(Key::NumpadSubtract)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadMultiply) => {
                Some(Key::NumpadMultiply)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadDivide) => Some(Key::NumpadDivide),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadDecimal) => {
                Some(Key::NumpadDecimal)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadEnter) => Some(Key::NumpadEnter),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadEqual) => Some(Key::NumpadEqual),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadClear) => Some(Key::NumpadClear),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadClearEntry) => {
                Some(Key::NumpadClearEntry)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadComma) => Some(Key::NumpadComma),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadHash) => Some(Key::NumpadHash),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadBackspace) => {
                Some(Key::NumpadBackspace)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadParenLeft) => {
                Some(Key::NumpadParenLeft)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadParenRight) => {
                Some(Key::NumpadParenRight)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadStar) => Some(Key::NumpadStar),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadMemoryAdd) => {
                Some(Key::NumpadMemoryAdd)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadMemoryClear) => {
                Some(Key::NumpadMemoryClear)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadMemoryRecall) => {
                Some(Key::NumpadMemoryRecall)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadMemoryStore) => {
                Some(Key::NumpadMemoryStore)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NumpadMemorySubtract) => {
                Some(Key::NumpadMemorySubtract)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F1) => Some(Key::F1),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F2) => Some(Key::F2),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F3) => Some(Key::F3),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F4) => Some(Key::F4),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F5) => Some(Key::F5),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F6) => Some(Key::F6),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F7) => Some(Key::F7),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F8) => Some(Key::F8),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F9) => Some(Key::F9),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F10) => Some(Key::F10),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F11) => Some(Key::F11),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F12) => Some(Key::F12),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F13) => Some(Key::F13),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F14) => Some(Key::F14),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F15) => Some(Key::F15),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F16) => Some(Key::F16),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F17) => Some(Key::F17),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F18) => Some(Key::F18),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F19) => Some(Key::F19),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F20) => Some(Key::F20),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F21) => Some(Key::F21),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F22) => Some(Key::F22),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F23) => Some(Key::F23),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F24) => Some(Key::F24),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F25) => Some(Key::F25),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F26) => Some(Key::F26),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F27) => Some(Key::F27),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F28) => Some(Key::F28),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F29) => Some(Key::F29),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F30) => Some(Key::F30),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F31) => Some(Key::F31),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F32) => Some(Key::F32),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F33) => Some(Key::F33),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F34) => Some(Key::F34),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::F35) => Some(Key::F35),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Convert) => Some(Key::Convert),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KanaMode) => Some(Key::KanaMode),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Lang1) => Some(Key::Lang1),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Lang2) => Some(Key::Lang2),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Lang3) => Some(Key::Lang3),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Lang4) => Some(Key::Lang4),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Lang5) => Some(Key::Lang5),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::NonConvert) => Some(Key::NonConvert),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Hiragana) => Some(Key::Hiragana),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Katakana) => Some(Key::Katakana),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaPlayPause) => {
                Some(Key::MediaPlayPause)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaStop) => Some(Key::MediaStop),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaTrackNext) => {
                Some(Key::MediaTrackNext)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaTrackPrevious) => {
                Some(Key::MediaTrackPrevious)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaFastForward) => {
                Some(Key::MediaFastForward)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaPause) => Some(Key::MediaPause),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaPlay) => Some(Key::MediaPlay),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaRecord) => Some(Key::MediaRecord),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaRewind) => Some(Key::MediaRewind),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MediaSelect) => Some(Key::MediaSelect),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::AudioVolumeDown) => {
                Some(Key::AudioVolumeDown)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::AudioVolumeMute) => {
                Some(Key::AudioVolumeMute)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::AudioVolumeUp) => {
                Some(Key::AudioVolumeUp)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrowserBack) => Some(Key::BrowserBack),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrowserFavorites) => {
                Some(Key::BrowserFavorites)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrowserForward) => {
                Some(Key::BrowserForward)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrowserHome) => Some(Key::BrowserHome),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrowserRefresh) => {
                Some(Key::BrowserRefresh)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrowserSearch) => {
                Some(Key::BrowserSearch)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrowserStop) => Some(Key::BrowserStop),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::LaunchApp1) => Some(Key::LaunchApp1),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::LaunchApp2) => Some(Key::LaunchApp2),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::LaunchAssistant) => {
                Some(Key::LaunchAssistant)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::LaunchControlPanel) => {
                Some(Key::LaunchControlPanel)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::LaunchMail) => Some(Key::LaunchMail),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::LaunchScreenSaver) => {
                Some(Key::LaunchScreenSaver)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MailForward) => Some(Key::MailForward),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MailReply) => Some(Key::MailReply),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MailSend) => Some(Key::MailSend),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Power) => Some(Key::Power),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Sleep) => Some(Key::Sleep),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::WakeUp) => Some(Key::WakeUp),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Eject) => Some(Key::Eject),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Abort) => Some(Key::Abort),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Resume) => Some(Key::Resume),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Suspend) => Some(Key::Suspend),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Again) => Some(Key::Again),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Copy) => Some(Key::Copy),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Cut) => Some(Key::Cut),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Find) => Some(Key::Find),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Open) => Some(Key::Open),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Paste) => Some(Key::Paste),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Props) => Some(Key::Props),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Select) => Some(Key::Select),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Undo) => Some(Key::Undo),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrightnessDown) => {
                Some(Key::BrightnessDown)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::BrightnessUp) => Some(Key::BrightnessUp),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::DisplayToggleIntExt) => {
                Some(Key::DisplayToggleIntExt)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyboardBacklightToggle) => {
                Some(Key::KeyboardBacklightToggle)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyboardLayoutSelect) => {
                Some(Key::KeyboardLayoutSelect)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::MicrophoneMuteToggle) => {
                Some(Key::MicrophoneMuteToggle)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::PrivacyScreenToggle) => {
                Some(Key::PrivacyScreenToggle)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::SelectTask) => Some(Key::SelectTask),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ShowAllWindows) => {
                Some(Key::ShowAllWindows)
            }
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ZoomToggle) => Some(Key::ZoomToggle),
            _ => None,
        }
    }

    #[inline]
    fn button(button: event::MouseButton) -> Option<Button> {
        match button {
            event::MouseButton::Left => Some(Button::Left),
            event::MouseButton::Right => Some(Button::Right),
            event::MouseButton::Middle => Some(Button::Middle),
            event::MouseButton::Back => Some(Button::Back),
            event::MouseButton::Forward => Some(Button::Forward),
            event::MouseButton::Button6 => Some(Button::Button6),
            event::MouseButton::Button7 => Some(Button::Button7),
            event::MouseButton::Button8 => Some(Button::Button8),
            event::MouseButton::Button9 => Some(Button::Button9),
            event::MouseButton::Button10 => Some(Button::Button10),
            event::MouseButton::Button11 => Some(Button::Button11),
            event::MouseButton::Button12 => Some(Button::Button12),
            event::MouseButton::Button13 => Some(Button::Button13),
            event::MouseButton::Button14 => Some(Button::Button14),
            event::MouseButton::Button15 => Some(Button::Button15),
            event::MouseButton::Button16 => Some(Button::Button16),
            event::MouseButton::Button17 => Some(Button::Button17),
            event::MouseButton::Button18 => Some(Button::Button18),
            event::MouseButton::Button19 => Some(Button::Button19),
            event::MouseButton::Button20 => Some(Button::Button20),
            event::MouseButton::Button21 => Some(Button::Button21),
            event::MouseButton::Button22 => Some(Button::Button22),
            event::MouseButton::Button23 => Some(Button::Button23),
            event::MouseButton::Button24 => Some(Button::Button24),
            event::MouseButton::Button25 => Some(Button::Button25),
            event::MouseButton::Button26 => Some(Button::Button26),
            event::MouseButton::Button27 => Some(Button::Button27),
            event::MouseButton::Button28 => Some(Button::Button28),
            event::MouseButton::Button29 => Some(Button::Button29),
            event::MouseButton::Button30 => Some(Button::Button30),
            event::MouseButton::Button31 => Some(Button::Button31),
            event::MouseButton::Button32 => Some(Button::Button32),
        }
    }
}
