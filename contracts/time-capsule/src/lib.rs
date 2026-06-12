#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

// 1. Định nghĩa cấu trúc dữ liệu cho Hộp thời gian
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capsule {
    pub owner: Address,
    pub item_id: u32,
    pub unlock_timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Capsule(u32), 
}

#[contract]
pub struct TimeCapsuleContract;

#[contractimpl]
impl TimeCapsuleContract {
    // 2. Tạo hộp mới và khóa lại theo thời gian
    pub fn create_capsule(env: Env, owner: Address, item_id: u32, duration_seconds: u64) {
        owner.require_auth();

        let key = DataKey::Capsule(item_id);
        if env.storage().persistent().has(&key) {
            panic!("Capsule with this item_id already exists");
        }

        let current_time = env.ledger().timestamp();
        let unlock_timestamp = current_time + duration_seconds;

        let capsule = Capsule {
            owner,
            item_id,
            unlock_timestamp,
        };

        env.storage().persistent().set(&key, &capsule);
    }

    // 3. Chuyển nhượng quyền sở hữu hộp
    pub fn transfer_ownership(env: Env, item_id: u32, new_owner: Address) {
        let key = DataKey::Capsule(item_id);
        
        let mut capsule: Capsule = env.storage().persistent().get(&key)
            .expect("Capsule not found");

        capsule.owner.require_auth();

        capsule.owner = new_owner;
        env.storage().persistent().set(&key, &capsule);
    }

    // 4. Kiểm tra xem hộp đã đến giờ mở chưa
    pub fn is_accessible(env: Env, item_id: u32) -> bool {
        let key = DataKey::Capsule(item_id);
        let capsule: Capsule = env.storage().persistent().get(&key)
            .expect("Capsule not found");

        let current_time = env.ledger().timestamp();
        current_time >= capsule.unlock_timestamp
    }
}