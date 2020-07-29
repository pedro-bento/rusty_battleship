use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use async_trait::async_trait;
use mini_redis::server;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use super::config;
use super::initial_state;
use super::state;

struct Button {
    body: Rect,
    color: Color,
    // text: Text // Todo.
}

impl Button {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Some(self.body)).unwrap();
        canvas.draw_rect(self.body).unwrap();
    }

    fn is_click(&self, x: i32, y: i32) -> bool {
        if x >= self.body.x()
            && x < self.body.x() + self.body.width() as i32
            && y >= self.body.y()
            && y < self.body.y + self.body.height() as i32
        {
            return true;
        }
        return false;
    }
}

pub struct ChooseState {
    create_button: Button,
    join_button: Button,
}

impl ChooseState {
    pub fn new() -> ChooseState {
        ChooseState {
            create_button: Button {
                // placeholders
                body: Rect::new(
                    config::WINDOW_WIDTH as i32 / 2 - 100,
                    config::WINDOW_HEIGHT as i32 / 3 - 20,
                    200,
                    40,
                ),

                color: Color::RGBA(0, 0, 255, 255),
            },

            join_button: Button {
                // placeholders
                body: Rect::new(
                    config::WINDOW_WIDTH as i32 / 2 - 100,
                    config::WINDOW_HEIGHT as i32 / 2 - 20,
                    200,
                    40,
                ),

                color: Color::RGBA(0, 255, 0, 255),
            },
        }
    }

    async fn start_server(&mut self) -> (SocketAddr, JoinHandle<mini_redis::Result<()>>) {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let handle =
            tokio::spawn(async move { server::run(listener, tokio::signal::ctrl_c()).await });

        (addr, handle)
    }
}

#[async_trait(?Send)]
impl state::State for ChooseState {
    async fn handle_events(
        &mut self,
        event_pump: &mut EventPump,
        next_state: &mut Option<state::NextState>,
    ) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    next_state.replace(state::NextState::Quit);
                    return;
                }

                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => {
                        next_state.replace(state::NextState::Quit);
                        return;
                    }

                    _ => {
                        println!("<ChooseState> unused key: {}", keycode.unwrap());
                    }
                },

                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    if self.create_button.is_click(x, y) {
                        let (_, server_handle) = self.start_server().await;

                        next_state.replace(state::NextState::Update(Box::new(
                            initial_state::InitialState::new(Some(server_handle)).await,
                        )));
                        return;
                    } else if self.join_button.is_click(x, y) {
                        next_state.replace(state::NextState::Update(Box::new(
                            initial_state::InitialState::new(None).await,
                        )));
                        return;
                    }
                }

                _ => {}
            }
        }

        next_state.replace(state::NextState::Continue);
    }

    async fn draw(&self, canvas: &mut Canvas<Window>) {
        self.create_button.draw(canvas);
        self.join_button.draw(canvas);
    }
}
