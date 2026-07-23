mod app;
mod color;
mod mesh;
mod scene;
mod ui;

use anyhow::Result;
use app::AppState;
use scene::Scene;
use three_d::egui;
use three_d::{
    Camera, ClearState, Event, FrameOutput, GUI, InnerSpace, MouseButton, OrbitControl,
    PhysicalPoint, Viewport, Window, WindowSettings, degrees, vec3,
};

/// Number of idle frames before the selection overlay is rebuilt.
const IDLE_FRAMES: u32 = 6;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let window = Window::new(WindowSettings {
        title: "ecitygml viewer".into(),
        min_size: (900, 600),
        initial_size: Some((1600, 900)),
        ..Default::default()
    })?;

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(200.0, -200.0, 200.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, 1.0),
        degrees(45.0),
        0.1,
        100_000.0,
    );
    let mut orbit = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 100_000.0);
    let mut gui = GUI::new(&context);
    gui.context().set_visuals(three_d::egui::Visuals::light());
    let mut state = AppState::default();
    let mut scene: Option<Scene> = None;
    // Counts frames since the camera last moved; resets on motion.
    let mut idle_frames: u32 = IDLE_FRAMES;

    window.render_loop(move |mut frame_input| {
        // Poll background load
        if state.poll_load()
            && let Some(graphics) = state.graphics.clone()
        {
            let new_scene = Scene::from_store(&context, graphics);
            new_scene.fit_camera(&mut camera);
            scene = Some(new_scene);
            idle_frames = IDLE_FRAMES;
        }

        camera.set_viewport(frame_input.viewport);

        // Collect viewport click before egui consumes events
        let mut viewport_pick: Option<PhysicalPoint> = None;
        for event in &frame_input.events {
            if let Event::MousePress {
                button: MouseButton::Left,
                position,
                handled: false,
                ..
            } = event
            {
                viewport_pick = Some(*position);
            }
        }

        let mut sidebar_action = ui::sidebar::SidebarAction {
            selected: None,
            toggled_type: None,
            select_all: None,
        };
        let mut toolbar_action = ui::toolbar::ToolbarAction {
            open_file: None,
            recenter: false,
        };

        // Rect actually left over for the 3D viewport after toolbar/sidebar/inspector have
        // claimed their space this frame — captured inside the closure since `Panel::show`
        // shrinks `ctx.available_rect()` as a side effect, not the `root_ui` passed to us.
        let mut viewport_rect: Option<egui::Rect> = None;

        let gui_wants_input = gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ui| {
                let ctx = ui.ctx().clone();
                toolbar_action = ui::toolbar::show(&ctx, &state);
                sidebar_action = ui::sidebar::show(&ctx, &state);
                ui::inspector::show(&ctx, &state);
                // `available_rect` is deprecated in favor of `viewport_rect`/`content_rect`,
                // but those never shrink as panels are added — `available_rect` is the only
                // one that actually reflects space left after `Panel::show` calls.
                #[expect(deprecated)]
                {
                    viewport_rect = Some(ctx.available_rect());
                }
            },
        );

        // `GUI::update`/`is_pointer_over_egui` can't detect our panels (they're shown via
        // `Panel::show(ctx, ..)`, not nested inside the `root_ui` egui tracks for that check),
        // so it never marks MouseWheel handled while hovering them — check the pointer against
        // the actual leftover viewport rect ourselves instead.
        let pointer_over_panel = viewport_rect
            .zip(gui.context().pointer_latest_pos())
            .is_some_and(|(rect, pos)| !rect.contains(pos));

        if gui_wants_input || pointer_over_panel {
            for event in frame_input.events.iter_mut() {
                if let Event::MouseWheel { handled, .. } = event {
                    *handled = true;
                }
            }
        }

        // On left press, move orbit pivot to the picked world point so rotation
        // always happens around the geometry under the cursor.
        if let Some(pos) = viewport_pick
            && let Some(s) = &scene
            && let Some(pivot) = s.pick_world_point(&camera, pos, &state.hidden_feature_types)
        {
            orbit.target = pivot;
        }

        // Detect camera motion by comparing position before/after orbit control.
        let pos_before = camera.position();
        orbit.handle_events(&mut camera, &mut frame_input.events);

        // Right-button pan: translate camera and orbit target together in the view plane.
        for event in frame_input.events.iter_mut() {
            if let Event::MouseMotion {
                delta,
                button: Some(MouseButton::Right),
                handled,
                ..
            } = event
                && !*handled
            {
                let distance = (camera.position() - orbit.target).magnitude();
                let scale = distance / frame_input.viewport.height as f32;
                let right = camera.right_direction();
                let up = camera.up();
                let pan = right * (-delta.0 * scale) + up * (delta.1 * scale);
                let new_pos = camera.position() + pan;
                let new_target = orbit.target + pan;
                camera.set_view(new_pos, new_target, up);
                orbit.target = new_target;
                *handled = true;
            }
        }

        let camera_moved = camera.position() != pos_before;

        if camera_moved {
            if idle_frames >= IDLE_FRAMES {
                // Transition: idle → moving
                if let Some(s) = &mut scene {
                    s.on_camera_moving();
                }
            }
            idle_frames = 0;
        } else if idle_frames < IDLE_FRAMES {
            idle_frames += 1;
            if idle_frames == IDLE_FRAMES {
                // Transition: moving → idle — rebuild the highlight
                if let Some(s) = &mut scene {
                    s.on_camera_idle(state.selected_id.as_ref());
                }
            }
        }

        if let Some(path) = toolbar_action.open_file {
            state.start_load(path);
            scene = None;
        }

        if toolbar_action.recenter
            && let Some(s) = &scene
        {
            s.fit_camera(&mut camera);
            orbit.target = vec3(0.0, 0.0, 0.0);
        }

        if let Some(ft) = sidebar_action.toggled_type {
            state.toggle_feature_type(ft);
        }

        if let Some(show) = sidebar_action.select_all {
            if show {
                state.show_all_feature_types();
            } else {
                state.hide_all_feature_types();
            }
        }

        if let Some(id) = sidebar_action.selected {
            state.selected_id = Some(id.clone());
            if let Some(s) = &mut scene {
                s.set_selected(Some(&id));
            }
        }

        if let Some(pos) = viewport_pick
            && let Some(s) = scene.as_mut()
        {
            let hit = s.pick_at(&camera, pos, &state.hidden_feature_types);
            state.selected_id = hit.clone();
            s.set_selected(hit.as_ref());
        }

        let screen = frame_input.screen();
        if let Some(s) = &scene {
            s.render(&screen, &camera, &state.hidden_feature_types);
        } else {
            screen.clear(ClearState::color_and_depth(0.9, 0.9, 0.9, 1.0, 1.0));
        }
        screen.write(|| gui.render()).unwrap();

        FrameOutput::default()
    });

    Ok(())
}
