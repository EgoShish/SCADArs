use comms::event::Event;

#[derive(Debug, Clone)]
pub enum Action {
    LoginSuccessful(event::LoginSuccessfulReplyEvent),
    RoomParticipation(event::RoomParticipationBroacastEvent),
    UserJoinedRoom(event::UserJoinedRoomReplyEvent),
    UserMessage(event::UserMessageBroadcastEvent),

    ConnectToServerRequest { addr: String },
    ConnectionResultOk { addr: String },
    ConnectionResultErr { err: String },
    SendMessage { content: String },
    SelectRoom { room: String },
    Exit,
}

impl From<Event> for Action {
    fn from(event: Event) -> Self {
        match event {
            Event::LoginSuccessful(data) => Action::LoginSuccessful(data),
            Event::RoomParticipation(data) => Action::RoomParticipation(data),
            Event::UserJoinedRoom(data) => Action::UserJoinedRoom(data),
            Event::UserMessage(data) => Action::UserMessage(data),
        }
    }
}
