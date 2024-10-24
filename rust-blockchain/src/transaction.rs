use std::collections::HashMap;

use crate::{blockchain::Blockchain, errors::Result, tx::{TXInput, TXOutput}, wallet::Wallets};
use crypto::{digest::Digest, ed25519, ripemd160::Ripemd160, sha2::Sha256};
use failure::format_err;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {
    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
        if data == String::from("") {
            data += &format!("Reward to '{}'", to);
        }
        let mut tx = Transaction {
            id: String::new(),
            vin: vec![TXInput {
                txid: String::new(),
                vout: -1,
                signature:Vec::new(),
                pub_key: Vec::from(data.as_bytes())
            }],
            vout: vec![TXOutput::new(100, to)?],
        };
        tx.id = tx.hash()?;
        
        Ok(tx)
    }
    pub fn is_coinbase(&self) ->bool{
        self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }
    pub fn new_UTXO(from:&str,to:&str,amount:i32,bc:&Blockchain) -> Result<Self>{
        let mut vin = Vec::new();
        let wallets = Wallets::new()?;
        
        let wallet = match wallets.get_wallet(from) {
            Some(w) => w,
            None => return Err(format_err!("from wallet nf"))
        };
        if let None = wallets.get_wallet(&to){
            return Err(format_err!("to wallet nf"));
        }
        let mut pub_key_hash = wallet.public_key.clone();
       
        hash_pub_key(&mut pub_key_hash);
       
        let acc_v = bc.find_spendable_outputs(&pub_key_hash, amount);

        if acc_v.0 < amount {
            error!("Not enought balance");
            return Err(format_err!("Not enough balance {}",acc_v.0));
        }

        for tx in acc_v.1 {
            for out in tx.1{
                let input = TXInput{
                    txid:tx.0.clone(),
                    vout:out,
                    signature:Vec::new(),
                    pub_key: wallet.public_key.clone(),
                };
                vin.push(input)
            }
        }
        let mut vout = vec![TXOutput::new(amount,to.into())?];
        
        if acc_v.0 > amount {
            vout.push(TXOutput::new(acc_v.0 - amount,from.into())?);
        }
        let mut tx = Transaction {
            id:String::new(),
            vin,
            vout
        };
        tx.id = tx.hash()?;
        bc.sign_transaction(&mut tx, &wallet.secret_key)?;



        Ok(tx)
    }
    pub fn sign(&mut self,private_key:&[u8],prev_TXs:HashMap<String,Transaction>) -> Result<()>{
        if self.is_coinbase(){
            return Ok(());
        }
        for vin in &self.vin {
            if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("ERO PREV TRANSACTION IS NOt CORRECT :P"));
            }

        }
        let mut tx_copy = self.trim_copy();

        for in_id in 0..tx_copy.vin.len() {
            let prev_Tx = prev_TXs.get(&tx_copy.vin[in_id].txid).unwrap();
            tx_copy.vin[in_id].signature.clear();
            tx_copy.vin[in_id].pub_key = prev_Tx.vout[tx_copy.vin[in_id].vout as usize]
                .pub_key_hash
                .clone();

            tx_copy.id = tx_copy.hash()?;
            tx_copy.vin[in_id].pub_key = Vec::new();
            let signature = ed25519::signature(tx_copy.id.as_bytes(), private_key);
            self.vin[in_id].signature =signature.to_vec();
        }
        Ok(())
    }
    fn trim_copy(&self) -> Transaction {
        let mut vin = Vec::new();
        let mut vout = Vec::new();

        for v in &self.vin{
            vin.push(TXInput{
                txid:v.txid.clone(),
                vout:v.vout.clone(),
                signature:Vec::new(),
                pub_key:Vec::new()
            })
        }
        for v in &self.vout {
            vout.push(TXOutput{
                value:v.value,
                pub_key_hash:v.pub_key_hash.clone()
            })
        }
        Transaction {
            id:self.id.clone(),
            vin,
            vout
        }
    }
pub fn hash(&mut self) -> Result<String> {
    self.id = String::new();
    let data = bincode::serialize(self)?;
    let mut hasher = Sha256::new();
    hasher.input(&data[..]);
    Ok(hasher.result_str())
}
pub fn verify(&mut self,prev_TXs:HashMap<String,Transaction>) -> Result<bool> {
    if self.is_coinbase() {
        return Ok(true);
    }
    for vin in &self.vin {
        if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
            return Err(format_err!("Prev trans is not correct D:"))
        }
    }
    for vin in &self.vin {
        if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
            return Err(format_err!("ERO PREV TRANSACTION IS NOt CORRECT :P"));
        }

    }
    let mut tx_copy = self.trim_copy();

    for in_id in 0..tx_copy.vin.len() {
        let prev_Tx = prev_TXs.get(&tx_copy.vin[in_id].txid).unwrap();
        tx_copy.vin[in_id].signature.clear();
        tx_copy.vin[in_id].pub_key = prev_Tx.vout[tx_copy.vin[in_id].vout as usize]
            .pub_key_hash
            .clone();


        tx_copy.id = tx_copy.hash()?;
        tx_copy.vin[in_id].pub_key = Vec::new();
        

        if !ed25519::verify(
        &tx_copy.id.as_bytes(),
     &self.vin[in_id].pub_key, 
                &self.vin[in_id].signature){
            return Ok(false);
        }
        }
    Ok(true)
}
}
pub fn hash_pub_key(pub_key:&mut Vec<u8>) {
    let mut hasher1 = Sha256::new();
    hasher1.input(pub_key);
    hasher1.result(pub_key);

    let mut hasher2 = Ripemd160::new();
    hasher2.input(pub_key);
    pub_key.resize(20, 0);
    hasher2.result(pub_key);
}

