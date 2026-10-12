use tokio::sync::{broadcast, mpsc};
use crate::{
    dispatcher::Dispatcher,
    ui_management::UiManager,
    termination::{create_termination, Interrupted, Terminator},
};
use tui::flux_engine::Action;
type ServerHandle = (EventStream, CommandWriter);

async fn create_server_handle(addr: &str) -> anyhow::Result<ServerHandle> {
    let stream = TcpStream::connect(addr).await?;
    let (event_stream, command_writer) = transport::client::split_tcp_stream(stream);

    Ok((event_stream, command_writer))
}

pub struct App {
    metainfo: String,
}

impl App {
    pub fn new(metainfo: String) -> Self {
        Self {
            metainfo,
        }
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let (terminator, mut interrupt_rx) = create_termination();
        let (dispatcher, store_rx) = Dispatcher::new();
        let (render, action_rx) = Render::new();

        tokio::try_join!(
            self.app_loop(terminator, dispatcher, action_rx, interrupt_rx.resubscribe()),
            render.render_loop(store_rx, interrupt_rx.resubscribe()),
        )?;

        match self.interrupt_rx.recv().await {
            Ok(Interrupted::UserInt) => println!("Завершено по запросу пользователя"),
            Ok(Interrupted::OsSigInt) => println!("Завершено по сигналу ОС (Ctrl+C)"),
            Err(_) => println!("Завершено из-за непредвиденной ошибки канала"),
        }

        Ok(())
    }

    async fn app_loop(&self, 
                mut terminator: Terminator,
                mut dispatcher: Dispatcher,
                mut action_rx: UnboundedReceiver<Action>,
                mut interrupt_rx: broadcast::Receiver<Interrupted>) -> anyhow::Result<Interrupted> {
        let mut opt_server_handle: Option<ServerHandle> = None;
        let mut ticker = tokio::time::interval(Duration::from_secs(1));
        let result = loop {
            if let Some((event_stream, command_writer)) = opt_server_handle.as_mut() {
                tokio::select! {
                    maybe_event = event_stream.next() => match maybe_event {
                        Some(Ok(event)) => {
                            let action: Action = event.into();
                            dispatcher.dispatch(action);
                        }
                        None => {
                            opt_server_handle = None;
                            dispatcher.resend();
                        }
                        _ => (),
                    },
                    Some(action) = action_rx.recv() => match action {
                        Action::SendMessage { content } => {
                            if let Some(active_room) = state.active_room.as_ref() {
                                command_writer
                                    .write(&command::UserCommand::SendMessage(
                                        command::SendMessageCommand {
                                            room: active_room.clone(),
                                            content,
                                        },
                                    ))
                                    .await
                                    .context("could not send message")?;
                            }
                        },
                        Action::SelectRoom { room } => {
                            if let Some(false) = state.try_set_active_room(room.as_str()).map(|room_data| room_data.has_joined) {
                                command_writer
                                    .write(&command::UserCommand::JoinRoom(command::JoinRoomCommand {
                                        room,
                                    }))
                                    .await
                                    .context("could not join room")?;
                            }
                        },
                        Action::Exit => {
                            let _ = terminator.terminate(Interrupted::UserInt);

                            break Interrupted::UserInt;
                        },
                        _ => (),
                    },
                    _ = ticker.tick() => {
                        state.tick_timer();
                    },
                    Ok(interrupted) = interrupt_rx.recv() => {
                        break interrupted;
                    }
                }
            } else {
                tokio::select! {
                    Some(action) = action_rx.recv() => match action {
                        Action::ConnectToServerRequest { addr } => {
                            dispatcher.dispatch(action);
                            dispatcher.resend();

                            match create_server_handle(&addr).await {
                                Ok(server_handle) => {
                                    let _ = opt_server_handle.insert(server_handle);
                                    dispatcher.dispatch(Action::ConnectionResult { addr });
                                    ticker.reset();
                                },
                                Err(err) => {
                                    dispatcher.dispatch(Action::ConnectionResult { err.to_string() });
                                }
                            }
                        },
                        Action::Exit => {
                            let _ = terminator.terminate(Interrupted::UserInt);

                            break Interrupted::UserInt;
                        },
                        _ => (),
                    },
                    Ok(interrupted) = interrupt_rx.recv() => {
                        break interrupted;
                    }
                }
            }
            dispatcher.resend();
        };
        Ok(result)
    }
}