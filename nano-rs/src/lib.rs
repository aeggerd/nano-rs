
mod account_balance;
mod account_block_count;
mod account_get;
mod account_history;
mod account_info;
mod account_key;
mod account_representative;
mod account_weight;
mod accounts_balances;
mod accounts_frontiers;
mod accounts_pending;
mod active_difficulty;
mod available_supply;
mod block_account;
mod block_confirm;
mod block_count;
mod block_create;
mod block_hash;
mod block_info;
mod blocks;
mod blocks_info;
mod bootstrap;
mod bootstrap_any;
mod bootstrap_lazy;
mod bootstrap_status;
mod chain;
mod confirmation_active;
mod confirmation_height_currently_processing;
mod confirmation_history;
mod confirmation_info;
mod confirmation_quorum;
mod database_txn_tracker;
mod delegators;
mod delegators_count;
mod deterministic_key;
mod epoch_upgrade;
mod frontier_count;
mod frontiers;
mod keepalive;
mod key_create;
mod key_expand;
mod ledger;
mod node_id;
mod node_id_delete;
mod peers;
mod pending;
mod pending_exists;
mod process;
mod representatives;
mod representatives_online;
mod republish;
mod sign;
mod stats;
mod stats_clear;
mod stop;
mod successors;
mod telemetry;
mod validate_account_number;
mod version;
mod unchecked;
mod unchecked_clear;
mod unchecked_get;
mod unchecked_keys;
mod unopened;
mod uptime;
mod work_cancel;
mod work_generate;
mod work_peer_add;
mod work_peers;
mod work_peers_clear;
mod work_validate;
// use reqwest;

#[path = "base/base.rs"] mod base;
#[path = "base/action.rs"] mod action;

use base::Account;
 use action::Action;

pub struct RpcClient {
    pub rpc_base: String,
}

pub struct Walet {
    pub account: base::Account,
    pub key: base::AccountKey,
}



impl RpcClient {
    // https://docs.nano.org/commands/rpc-protocol/#account_balance
    pub fn get_account_balance(&self, account: String) -> Result<account_balance::AccountBalanceResponse, reqwest::Error> {
        let request = account_balance::AccountBalanceRequest {
            // action: "account_balance".to_string(),
            action: Action::AccountBalance.as_str(),
            account: account,
        };

        let client = reqwest::blocking::Client::new();
        let res: account_balance::AccountBalanceResponse = client
        .post(&self.rpc_base)
        .body(request.as_vec())
        .send()?
        .json()?;
        Ok(res)
    }

    // https://docs.nano.org/commands/rpc-protocol/#account_block_count
    pub fn get_account_block_count(&self, account: String) -> Result<account_block_count::AccountBlockCountResponse, reqwest::Error> {
        let request = account_block_count::AccountBlockCountRequest {
            action: Action::AccountBlockCount.as_str(),
            account: account
        };

        let client = reqwest::blocking::Client::new();
        let res: account_block_count::AccountBlockCountResponse = client
        .post(&self.rpc_base)
        .body(request.as_vec())
        .send()?
        .json()?;
        Ok(res)
    }

    // https://docs.nano.org/commands/rpc-protocol/#account_get
    pub fn get_account_get(&self, key: String) -> Result<account_get::AccountGetResult, reqwest::Error> {
        let request = account_get::AccountGetRequest {
            action: Action::AccountGet.as_str(),
            key: key
        };

        let client = reqwest::blocking::Client::new();
        let res: account_get::AccountGetResult = client
        .post(&self.rpc_base)
        .body(request.as_vec())
        .send()?
        .json()?;
        Ok(res)
    }

    // https://docs.nano.org/commands/rpc-protocol/#account_history
    pub fn get_account_history(&self, account: String, count: String) -> Result<account_history::AccountHistoryResult, reqwest::Error> {
        let request = account_history::AccountHistoryRequest {
            action: Action::AccountHistory.as_str(),
            account: account,
            count: count,
        };

        let client = reqwest::blocking::Client::new();
        let res: account_history::AccountHistoryResult = client
        .post(&self.rpc_base)
        .body(request.as_vec())
        .send()?
        .json()?;
        Ok(res)
    }

    // https://docs.nano.org/commands/rpc-protocol/#account_info
    pub fn get_account_info(&self, account: String) -> Result<account_info::AccountInfoResult, reqwest::Error> {
        let request = account_info::AccountInfoRequest {
            action: Action::AccountInfo.as_str(),
            account: account,
        };

        let client = reqwest::blocking::Client::new();
        let res: account_info::AccountInfoResult = client
        .post(&self.rpc_base)
        .body(request.as_vec())
        .send()?
        .json()?;
        Ok(res)
    }


    // https://docs.nano.org/commands/rpc-protocol/#account_key
    // gets the public key
    pub fn get_account_key(&self, account: String) -> Result<account_key::AccountKeyResult, reqwest::Error> {
        let request = account_key::AccountKeyRequest {
            action: Action::AccountKey.as_str(),
            account: account,
        };

        let client = reqwest::blocking::Client::new();
        let res: account_key::AccountKeyResult = client
        .post(&self.rpc_base)
        .body(request.as_vec())
        .send()?
        .json()?;
        Ok(res)
    }

        // https://docs.nano.org/commands/rpc-protocol/#account_representative
        pub fn get_account_representative(&self, account: String) -> Result<account_representative::Result, reqwest::Error> {
            let request = account_representative::Request {
                action: Action::AccountRepresentative.as_str(),
                account: account,
            };
    
            let client = reqwest::blocking::Client::new();
            let res: account_representative::Result = client
            .post(&self.rpc_base)
            .body(request.as_vec())
            .send()?
            .json()?;
            Ok(res)
        }

        // https://docs.nano.org/commands/rpc-protocol/#account_weight
        pub fn get_account_weight(&self, account: String) -> Result<account_weight::Result, reqwest::Error> {
            let request = account_weight::Request {
                action: Action::AccountWeight.as_str(),
                account: account,
            };
    
            let client = reqwest::blocking::Client::new();
            let res: account_weight::Result = client
            .post(&self.rpc_base)
            .body(request.as_vec())
            .send()?
            .json()?;
            Ok(res)
        }

        // https://docs.nano.org/commands/rpc-protocol/#accounts_balances
        // pub fn get_accounts_balances(&self, accounts: Vec<String>) -> Result<accounts_balances::Result, reqwest::Error> {
        //     let request = accounts_balances::Request {
        //         action: "accounts_balances".to_string(),
        //         accounts: accounts,
        //     };
    
        //     let client = reqwest::blocking::Client::new();
        //     let res: accounts_balances::Result = client
        //     .post(&self.rpc_base)
        //     .body(request.as_vec())
        //     .send()?
        //     .json()?;
        //     Ok(res)
        // }

        // https://docs.nano.org/commands/rpc-protocol/#accounts_frontiers
        // pub fn get_account_weight(&self, account: String) -> Result<accounts_frontiers::Result, reqwest::Error> {
        //     let request = accounts_frontiers::Request {
        //         action: "account_weight".to_string(),
        //         account: account,
        //     };
    
        //     let client = reqwest::blocking::Client::new();
        //     let res: accounts_frontiers::Result = client
        //     .post(&self.rpc_base)
        //     .body(request.as_vec())
        //     .send()?
        //     .json()?;
        //     Ok(res)
        // }

                // https://docs.nano.org/commands/rpc-protocol/#accounts_pending
        // pub fn get_account_weight(&self, account: String) -> Result<accounts_pending::Result, reqwest::Error> {
        //     let request = accounts_pending::Request {
        //         action: "account_weight".to_string(),
        //         account: account,
        //     };
    
        //     let client = reqwest::blocking::Client::new();
        //     let res: accounts_pending::Result = client
        //     .post(&self.rpc_base)
        //     .body(request.as_vec())
        //     .send()?
        //     .json()?;
        //     Ok(res)
        // }

        // https://docs.nano.org/commands/rpc-protocol/#active_difficulty
        pub fn get_active_difficulty(&self) -> Result<active_difficulty::Result, reqwest::Error> {
            let request = active_difficulty::Request {
                action: Action::ActiveDifficulty.as_str(),
            };
    
            let client = reqwest::blocking::Client::new();
            let res: active_difficulty::Result = client
            .post(&self.rpc_base)
            .body(request.as_vec())
            .send()?
            .json()?;
            Ok(res)
        }

        // https://docs.nano.org/commands/rpc-protocol/#available_supply
        pub fn get_available_supply(&self) -> Result<available_supply::Result, reqwest::Error> {
            let request = available_supply::Request {
                action: Action::AvailableSupply.as_str(),
            };
    
            let client = reqwest::blocking::Client::new();
            let res: available_supply::Result = client
            .post(&self.rpc_base)
            .body(request.as_vec())
            .send()?
            .json()?;
            Ok(res)
        }

        // https://docs.nano.org/commands/rpc-protocol/#block_account
        pub fn get_block_account(&self, hash: String) -> Result<block_account::Result, reqwest::Error> {
            let request = block_account::Request {
                action: Action::BlockAccount.as_str(),
                hash: hash,
            };
    
            let client = reqwest::blocking::Client::new();
            let res: block_account::Result = client
            .post(&self.rpc_base)
            .body(request.as_vec())
            .send()?
            .json()?;
            Ok(res)
        }

        // https://docs.nano.org/commands/rpc-protocol/#block_confirm
        pub fn get_block_confirm(&self, hash: String) -> Result<block_confirm::Result, reqwest::Error> {
            let request = block_confirm::Request {
                action: Action::BlockConfirm.as_str(),
                hash: hash,
            };
    
            let client = reqwest::blocking::Client::new();
            let res: block_confirm::Result = client
            .post(&self.rpc_base)
            .body(request.as_vec())
            .send()?
            .json()?;
            Ok(res)
        }



}