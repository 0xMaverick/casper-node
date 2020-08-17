#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn mint() {
    mint_token::mint();
}

#[no_mangle]
pub extern "C" fn create() {
    mint_token::create();
}

#[no_mangle]
pub extern "C" fn balance() {
    mint_token::balance();
}

#[no_mangle]
pub extern "C" fn transfer() {
    mint_token::transfer();
}

#[no_mangle]
pub extern "C" fn bond() {
    mint_token::bond();
}

#[no_mangle]
pub extern "C" fn unbond() {
    mint_token::unbond();
}

#[no_mangle]
pub extern "C" fn process_unbond_requests() {
    mint_token::process_unbond_requests();
}

#[no_mangle]
pub extern "C" fn slash() {
    mint_token::slash();
}
