use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Promise};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DroplinkedStorage {
    droplinked_account : AccountId, 
    fee : u128,
}

impl Default for DroplinkedStorage {
    fn default() -> Self {
        Self {
            droplinked_account : AccountId::new_unchecked("droplinked_payment.testnet".to_string()),
            fee : 1000u128 ,// 1000 = 1% fee
        }
    }
}

#[near_bindgen]
impl DroplinkedStorage {
    #[payable]
    pub fn direct_pay(&mut self, product_price : String, shipping_price : String, tax_price : String, recipient : AccountId) -> Promise {
        let product_price_u128 = product_price.parse::<u128>().unwrap();
        let shipping_price_u128 = shipping_price.parse::<u128>().unwrap();
        let tax_price_u128 = tax_price.parse::<u128>().unwrap();
        if near_sdk::env::attached_deposit() < product_price_u128 + shipping_price_u128 + tax_price_u128{
            near_sdk::env::panic_str("deposit is too low");
        }
        let droplinked_share = product_price_u128 * self.fee / 100000;
        Promise::new(recipient).transfer(((product_price_u128 + shipping_price_u128 + tax_price_u128 )-droplinked_share).into());
        Promise::new(self.droplinked_account.clone()).transfer(droplinked_share.into())
    }
}