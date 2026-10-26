use crate::{
    Button, ButtonStatus, Key, KeyStatus, KeyboardEvent, MouseEvent, MouseWheelEvent, WindowEvent,
};

use plam::{MousePosition, MouseScroll, MouseScrollDelta, SurfaceSize};

use winit::{event, keyboard};

#[derive(Clone, Copy)]
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
            _ => Some(WindowEvent::Unknown),
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
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyW) => Some(Key::W),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyA) => Some(Key::A),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyS) => Some(Key::S),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyD) => Some(Key::D),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyQ) => Some(Key::Q),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyE) => Some(Key::E),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ArrowUp) => Some(Key::UpArrow),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ArrowLeft) => Some(Key::LeftArrow),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ArrowDown) => Some(Key::DownArrow),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ArrowRight) => Some(Key::RightArrow),
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
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Escape) => Some(Key::Esc),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Tab) => Some(Key::Tab),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::CapsLock) => Some(Key::Caps),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ShiftLeft) => Some(Key::LeftShift),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ShiftRight) => Some(Key::RightShift),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ControlLeft) => Some(Key::LeftCtrl),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::ControlRight) => Some(Key::RightCtrl),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::AltLeft) => Some(Key::LeftAlt),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::AltRight) => Some(Key::RightAlt),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::Space) => Some(Key::Space),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyH) => Some(Key::H),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyJ) => Some(Key::J),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyK) => Some(Key::K),
            keyboard::PhysicalKey::Code(keyboard::KeyCode::KeyL) => Some(Key::L),
            _ => None,
        }
    }

    #[inline]
    fn button(button: event::MouseButton) -> Option<Button> {
        match button {
            event::MouseButton::Left => Some(Button::Left),
            event::MouseButton::Middle => Some(Button::Middle),
            event::MouseButton::Right => Some(Button::Right),
            _ => None,
        }
    }
}
