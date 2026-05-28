use super::node_identity::NodeIdentity;
use super::pin_manager::PinManager;
use std::sync::{Arc, Mutex};

/// Состояние Tamagotchi-бутстрапа
pub struct TamagotchiBootstrap {
    stage: BootstrapStage,
    host_preferences: Option<HostPreferences>,
    node_identity: Option<NodeIdentity>,
    pin_manager: Arc<Mutex<PinManager>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BootstrapStage {
    AwaitingHost,
    GeneratingEntropy,
    RequestingPassport,
    JoiningMesh,
    DownloadingGenome,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostPreferences {
    pub node_name: String,
    pub owner_name: String,
    pub personality_hint: String,
}

impl TamagotchiBootstrap {
    /// Инициализация с менеджером PIN-кодов
    pub fn new(pin_manager: Arc<Mutex<PinManager>>) -> Self {
        TamagotchiBootstrap {
            stage: BootstrapStage::AwaitingHost,
            host_preferences: None,
            node_identity: None,
            pin_manager,
        }
    }

    /// Генерация PIN-кода для ноды
    pub fn generate_pin(&self) -> String {
        self.pin_manager.lock().unwrap().generate_pin()
    }

    /// Регистрация PIN-кода
    pub fn register_pin(&mut self, node_id: String, pin: String) {
        self.pin_manager.lock().unwrap().register_pin(node_id, pin);
    }

    /// Проверка PIN-кода
    pub fn verify_pin(&self, node_id: &str, input: &str) -> bool {
        self.pin_manager.lock().unwrap().verify_pin(node_id, input)
    }

    /// Получение максимального количества попыток
    pub fn get_max_attempts(&self) -> u8 {
        self.pin_manager.lock().unwrap().get_max_attempts()
    }
}
