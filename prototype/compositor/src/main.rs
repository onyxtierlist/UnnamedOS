// UNNAMED OS — Stage 1 nested compositor prototype.
// Initial protocol/render loop adapted from Smithay's MIT-licensed minimal example:
// https://github.com/Smithay/smithay/blob/master/examples/minimal.rs

use std::sync::Arc;

use smithay::{
    backend::{
        input::{InputEvent, InputTime, KeyboardKeyEvent},
        renderer::{
            Color32F, Frame, Renderer,
            element::{
                Kind,
                surface::{WaylandSurfaceRenderElement, render_elements_from_surface_tree},
            },
            gles::GlesRenderer,
            utils::{draw_render_elements, on_commit_buffer_handler},
        },
        winit::{self, WinitEvent},
    },
    input::{Seat, SeatHandler, SeatState, keyboard::FilterResult},
    reexports::wayland_server::{Display, protocol::wl_seat},
    utils::{Rectangle, Serial, Transform},
    wayland::{
        buffer::BufferHandler,
        compositor::{
            CompositorClientState, CompositorHandler, CompositorState, SurfaceAttributes,
            TraversalAction, with_surface_tree_downward,
        },
        pointer_constraints::PointerConstraintsHandler,
        selection::{
            SelectionHandler,
            data_device::{DataDeviceHandler, DataDeviceState, WaylandDndGrabHandler},
        },
        shell::xdg::{PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState},
        shm::{ShmHandler, ShmState},
    },
};
use wayland_protocols::xdg::shell::server::xdg_toplevel;
use wayland_server::{
    Client, ListeningSocket,
    backend::{ClientData, ClientId, DisconnectReason},
    protocol::{wl_buffer, wl_surface::{self, WlSurface}},
};

impl BufferHandler for App {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}

impl XdgShellHandler for App {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState { &mut self.xdg_shell_state }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        surface.with_pending_state(|state| state.states.set(xdg_toplevel::State::Activated));
        surface.send_configure();
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {}
    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {}
    fn reposition_request(&mut self, _surface: PopupSurface, _positioner: PositionerState, _token: u32) {}
}

impl SelectionHandler for App { type SelectionUserData = (); }

impl DataDeviceHandler for App {
    fn data_device_state(&mut self) -> &mut DataDeviceState { &mut self.data_device_state }
}
impl WaylandDndGrabHandler for App {}

impl CompositorHandler for App {
    fn compositor_state(&mut self) -> &mut CompositorState { &mut self.compositor_state }
    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }
    fn commit(&mut self, surface: &WlSurface) { on_commit_buffer_handler::<Self>(surface); }
}

impl ShmHandler for App {
    fn shm_state(&self) -> &ShmState { &self.shm_state }
}

impl SeatHandler for App {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;
    fn seat_state(&mut self) -> &mut SeatState<Self> { &mut self.seat_state }
    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {}
    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: smithay::input::pointer::CursorImageStatus) {}
}
impl PointerConstraintsHandler for App {}

struct App {
    compositor_state: CompositorState,
    xdg_shell_state: XdgShellState,
    shm_state: ShmState,
    seat_state: SeatState<Self>,
    data_device_state: DataDeviceState,
    seat: Seat<Self>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }
    run_winit()
}

fn run_winit() -> Result<(), Box<dyn std::error::Error>> {
    let mut display: Display<App> = Display::new()?;
    let dh = display.handle();
    let compositor_state = CompositorState::new::<App>(&dh);
    let shm_state = ShmState::new::<App>(&dh, vec![]);
    let mut seat_state = SeatState::new();
    let seat = seat_state.new_wl_seat(&dh, "unnamed-seat");

    let mut state = App {
        compositor_state,
        xdg_shell_state: XdgShellState::new::<App>(&dh),
        shm_state,
        seat_state,
        data_device_state: DataDeviceState::new::<App>(&dh),
        seat,
    };

    let socket_name = "wayland-5";
    let listener = ListeningSocket::bind(socket_name)?;
    let mut clients = Vec::new();
    let (mut backend, mut winit) = winit::init::<GlesRenderer>()?;
    let start_time = std::time::Instant::now();
    let keyboard = state.seat.add_keyboard(Default::default(), 200, 200)?;

    // This is a nested test compositor. Set the client display only for child processes.
    unsafe { std::env::set_var("WAYLAND_DISPLAY", socket_name); }
    if std::process::Command::new("weston-terminal").spawn().is_err() {
        tracing::info!("weston-terminal not found; start a Wayland client with WAYLAND_DISPLAY={socket_name}");
    }

    loop {
        let status = winit.dispatch_new_events(|event| match event {
            WinitEvent::Resized { .. } => {}
            WinitEvent::Input(event) => match event {
                InputEvent::Keyboard { event } => keyboard.input::<(), _>(
                    &mut state, event.key_code(), event.state(), 0.into(),
                    InputTime::now(), |_, _, _| FilterResult::Forward,
                ),
                InputEvent::PointerMotionAbsolute { .. } => {
                    if let Some(surface) = state.xdg_shell_state.toplevel_surfaces().iter().next().cloned() {
                        keyboard.set_focus(&mut state, Some(surface.wl_surface().clone()), 0.into());
                    }
                }
                _ => {}
            },
            _ => (),
        });

        if let winit::PumpStatus::Exit(_) = status { return Ok(()); }

        let size = backend.window_size();
        let damage = Rectangle::from_size(size);
        {
            let (renderer, mut framebuffer) = backend.bind().unwrap();
            let elements = state.xdg_shell_state.toplevel_surfaces().iter().flat_map(|surface| {
                render_elements_from_surface_tree(renderer, surface.wl_surface(), (0, 0), 1.0, 1.0, Kind::Unspecified)
            }).collect::<Vec<WaylandSurfaceRenderElement<GlesRenderer>>>();

            let mut frame = renderer.render(&mut framebuffer, size, Transform::Flipped180).unwrap();
            frame.clear(Color32F::new(0.025, 0.035, 0.065, 1.0), &[damage]).unwrap();
            draw_render_elements(&mut frame, 1.0, &elements, &[damage]).unwrap();
            let _ = frame.finish().unwrap();

            for surface in state.xdg_shell_state.toplevel_surfaces() {
                send_frames_surface_tree(surface.wl_surface(), start_time.elapsed().as_millis() as u32);
            }
            if let Some(stream) = listener.accept()? {
                tracing::info!("Connected Wayland client: {:?}", stream);
                let client = display.handle().insert_client(stream, Arc::new(ClientState::default())).unwrap();
                clients.push(client);
            }
            display.dispatch_clients(&mut state)?;
            display.flush_clients()?;
        }
        backend.submit(Some(&[damage])).unwrap();
    }
}

fn send_frames_surface_tree(surface: &wl_surface::WlSurface, time: u32) {
    with_surface_tree_downward(
        surface, (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |_surface, states, &()| {
            for callback in states.cached_state.get::<SurfaceAttributes>().current().frame_callbacks.drain(..) {
                callback.done(time);
            }
        },
        |_, _, &()| true,
    );
}

#[derive(Default)]
struct ClientState { compositor_state: CompositorClientState }
impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) { tracing::info!("Wayland client initialized"); }
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) { tracing::info!("Wayland client disconnected"); }
}

smithay::delegate_dispatch2!(App);
