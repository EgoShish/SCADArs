use std::collections::{HashMap, HashSet};

use circular_queue::CircularQueue;
use comms::action;

#[derive(Debug, Clone)]
pub enum MessageBoxItem {
    Message { user_id: String, content: String },
    Notification(String),
}

const MAX_MESSAGES_TO_STORE_PER_ROOM: usize = 100;

/// RoomData - вспомогательный компонент store

#[derive(Debug, Clone)]
pub struct RoomData {
    /// наименование комнаты
    pub name: String,
    /// описание комнаты
    pub description: String,
    /// список участников комнаты
    pub users: HashSet<String>,
    /// история сообщений
    pub messages: CircularQueue<MessageBoxItem>,
    /// статус "подключен к комнате"
    pub has_joined: bool,
    /// статус "есть непрочитанное сообщение"
    pub has_unread: bool,
}

impl Default for RoomData {
    fn default() -> Self {
        RoomData {
            name: String::new(),
            description: String::new(),
            users: HashSet::new(),
            messages: CircularQueue::with_capacity(MAX_MESSAGES_TO_STORE_PER_ROOM),
            has_joined: false,
            has_unread: false,
        }
    }
}

impl RoomData {
    pub fn new(name: String, description: String) -> Self {
        RoomData {
            name,
            description,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum ServerConnectionStatus {
    Uninitalized,
    Connecting,
    Connected { addr: String },
    Errored { err: String },
}

/// Store - модель ui, содержащая как данные так и методы для их изменения
#[derive(Debug, Clone)]
pub struct Store {
    pub server_connection_status: ServerConnectionStatus,
    /// Текущая активная комната
    pub active_room: Option<String>,
    /// ID пользователя
    pub user_id: String,
    /// Ханилище чатов
    pub room_data_map: HashMap<String, RoomData>,
    /// Счетчик времени открытого приложения
    pub timer: usize,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            server_connection_status: ServerConnectionStatus::Uninitalized,
            active_room: None,
            user_id: String::new(),
            room_data_map: HashMap::new(),
            timer: 0,
        }
    }
}

impl Store {
    pub fn handle_server_action(&mut self, action: &Action) {
        match action {
            Action::LoginSuccessful(action) => {
                self.user_id = action.user_id.clone();
                self.room_data_map = action
                    .rooms
                    .clone()
                    .into_iter()
                    .map(|r| (r.name.clone(), RoomData::new(r.name, r.description)))
                    .collect();
            }
            Action::RoomParticipation(action) => {
                if let Some(room_data) = self.room_data_map.get_mut(&action.room) {
                    match action.status {
                    action::RoomParticipationStatus::Joined => {
                        room_data.users.insert(action.user_id.clone());
                            if action.user_id == self.user_id {
                                room_data.has_joined = true;
                            }
                    }
                    action::RoomParticipationStatus::Left => {
                            room_data.users.remove(&action.user_id);
                            if action.user_id == self.user_id {
                                room_data.has_joined = false;
                            }
                        }
                    }
                    room_data
                        .messages
                        .push(MessageBoxItem::Notification(format!(
                            "{} только что {}",
                            action.user_id,
                            match action.status {
                                action::RoomParticipationStatus::Joined => "был добавлен в комнату",
                                action::RoomParticipationStatus::Left => "вышел из комнаты",
                            }
                        )));
                }
                else {
                    println("Error RoomParticipation have None Option room_data_map");
                }
            }
            action::action::UserJoinedRoom(action) => {
                if let Some(room_data) = self.room_data_map.get_mut(&action.room) {
                    room_data.users = action.users.clone().into_iter().collect();
                }
                else {
                    println("Error UserJoinedRoom have None Option room_data_map");
                }
            }
            action::action::UserMessage(action) => {
                if let Some(room_data) = self.room_data_map.get_mut(&action.room) {
                    room_data.messages.push(MessageBoxItem::Message {
                        user_id: action.user_id.clone(),
                        content: action.content.clone(),
                    });
                }
                if let Some(active_room) = self.active_room.as_ref() {
                    if !active_room.eq(&action.room) {
                        room_data.has_unread = true;
                    }
                }
            }
        }
    }

    pub fn mark_connection_request_start(&mut self) {
        self.server_connection_status = ServerConnectionStatus::Connecting;
    }

    pub fn process_connection_request_result(&mut self, result: anyhow::Result<String>) {
        self.server_connection_status = match result {
            Ok(addr) => ServerConnectionStatus::Connected { addr: addr.clone() },
            Err(err) => ServerConnectionStatus::Errored { err: err.to_string() },
        }
    }

    pub fn try_set_active_room(&mut self, action: Action) -> Option<&RoomData> {
        if let Some(room_data) = self.room_data_map.get_mut(action::room) {
            room_data.has_unread = false;
            self.active_room = Some(action::room.clone());
            Some(room_data);
        }
    }

    pub fn tick_timer(&mut self) {
        self.timer += 1;
    }
}