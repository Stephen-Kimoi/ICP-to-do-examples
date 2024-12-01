use candid::Principal;
use ic_cdk::update;

#[update]
pub fn who_am_i() -> Principal {
    let caller = ic_cdk::caller();
    return caller;
}

ic_cdk::export_candid!();