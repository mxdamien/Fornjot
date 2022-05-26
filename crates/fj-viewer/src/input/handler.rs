use std::time::Instant;

use fj_interop::mesh::Mesh;
use fj_math::{Point, Transform, Vector};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta},
};

use crate::{
    camera::{Camera, FocusPoint},
    screen::{Position, Size},
};

use super::{movement::Movement, rotation::Rotation, zoom::Zoom, Event, Key};

/// Input handling abstraction
///
/// Takes user input and applies them to application state.
pub struct Handler {
    cursor: Option<Position>,

    movement: Movement,
    rotation: Rotation,
    zoom: Zoom,
}

impl Handler {
    /// Returns a new Handler.
    ///
    /// # Examples
    /// ```rust no_run
    /// // Store initialization time for camera zoom calculations
    /// let instant = std::time::Instant::now();
    /// let input_handler = fj_viewer::input::Handler::new(instant);
    /// ```
    pub fn new(now: Instant) -> Self {
        Self {
            cursor: None,

            movement: Movement::new(),
            rotation: Rotation::new(),
            zoom: Zoom::new(now),
        }
    }

    /// Returns the state of the cursor position.
    pub fn cursor(&self) -> Option<Position> {
        self.cursor
    }

    /// Handle an input event
    pub fn handle_event(
        &mut self,
        event: Event,
        screen_size: Size,
        camera: &mut Camera,
        actions: &mut Actions,
    ) {
        match event {
            Event::CursorMoved(position) => {
                if let Some(previous) = self.cursor {
                    let diff_x = position.x - previous.x;
                    let diff_y = position.y - previous.y;

                    self.movement.apply(self.cursor, camera, screen_size);
                    self.rotation.apply(diff_x, diff_y, camera);
                }

                self.cursor = Some(position);
            }
            Event::KeyPressed(key) => match key {
                Key::Escape => actions.exit = true,

                Key::Key1 => actions.toggle_model = true,
                Key::Key2 => actions.toggle_mesh = true,
                Key::Key3 => actions.toggle_debug = true,
            },
        }
    }

    /// Updates `state` and `focus_point` when mouse is clicked.
    pub fn handle_mouse_input(
        &mut self,
        button: MouseButton,
        state: ElementState,
        focus_point: FocusPoint,
    ) {
        match (button, state) {
            (MouseButton::Left, ElementState::Pressed) => {
                self.rotation.start(focus_point);
            }
            (MouseButton::Left, ElementState::Released) => {
                self.rotation.stop();
            }
            (MouseButton::Right, ElementState::Pressed) => {
                self.movement.start(focus_point, self.cursor);
            }
            (MouseButton::Right, ElementState::Released) => {
                self.movement.stop();
            }
            _ => {}
        }
    }

    /// Updates zoom state from the scroll wheel.
    pub fn handle_mouse_wheel(
        &mut self,
        delta: MouseScrollDelta,
        now: Instant,
    ) {
        let delta = match delta {
            MouseScrollDelta::LineDelta(_, y) => y as f64 * 10.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => y,
        };

        self.zoom.push_input_delta(delta, now);
    }

    /// Update application state from user input.
    pub fn update(
        &mut self,
        delta_t: f64,
        now: Instant,
        camera: &mut Camera,
        size: Size,
        mesh: &Mesh<Point<3>>,
    ) {
        let focus_point = camera.focus_point(size, self.cursor, mesh);

        self.zoom.discard_old_events(now);
        self.zoom.update_speed(now, delta_t, focus_point, camera);

        camera.translation = camera.translation
            * Transform::translation(Vector::from([
                0.0,
                0.0,
                -self.zoom.speed(),
            ]));
    }
}

/// Intermediate input state container
///
/// Used as a per frame state container for sending application state to `winit`.
#[derive(Default)]
pub struct Actions {
    /// Application exit state.
    pub exit: bool,

    /// Toggle for the shaded display of the model.
    pub toggle_model: bool,
    /// Toggle for the model's wireframe.
    pub toggle_mesh: bool,
    /// Toggle for debug information.
    pub toggle_debug: bool,
}

impl Actions {
    /// Returns a new `Actions`.
    pub fn new() -> Self {
        Self::default()
    }
}
