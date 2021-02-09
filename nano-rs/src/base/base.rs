pub mod action;
// pub use action;

#[derive(PartialEq)]
pub struct AccountKey(String);
impl From<String> for AccountKey {
    fn from(accountKey: String) -> Self {
        AccountKey(accountKey)
    }
}

#[derive(PartialEq)]
pub struct Account(String);
impl From<String> for Account {
    fn from(account: String) -> Self {
        Account(account)
    }
}

pub struct Count(i64);
impl From<String> for Count {
    fn from(count: String) -> Self {
        Count(count.parse::<i64>().unwrap())
    }
}

pub struct Height(i64);
impl From<String> for Height {
    fn from(height: String) -> Self {
        Height(height.parse::<i64>().unwrap())
    }
}

#[derive(PartialEq)]
pub struct Hash(String);
impl From<String> for Hash {
    fn from(hash: String) -> Self {
        Hash(hash)
    }
}

#[derive(PartialEq)]
pub struct IncludeActive(bool);
impl From<bool> for IncludeActive {
    fn from(incluce_active: bool) -> Self {
        IncludeActive(incluce_active)
    }
}




