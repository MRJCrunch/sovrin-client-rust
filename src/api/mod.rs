extern crate libc;

pub mod anoncreds;
pub mod signus;
pub mod ledger;
pub mod pool;
pub mod wallet;


use self::libc::{c_char};

#[repr(i32)]
pub enum ErrorCode {
    Success = 0,

    // Common errors
    // Caller passed invalid pool ledger handle
    CommonInvalidPoolLedgerHandle = 100,

    // Caller passed invalid wallet handle
    CommonInvalidWalletHandle,

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1,

    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2,

    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3,

    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5,

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState,

    // Wallet errors
    // Unknown type of wallet was passed on create_wallet
    WalletUnknownTypeError = 200,

    // Attempt to register already existing wallet type
    WalletTypeAlreadyRegisteredError,

    // Requested entity id isn't present in wallet
    WalletNotFoundError,

    // Wallet files referenced in open_wallet have invalid data format
    WalletInvalidDataFormat,

    // IO error during access wallet backend
    WalletIOError,

    // Trying to use wallet with pool that has different name
    WalletIncompatiblePoolError,

    // Ledger errors
    // Pool ledger files referenced in open_pool_ledger have invalid data format
    PoolLedgerInvalidDataFormat = 300,

    // IO error during access pool ledger files
    PoolILedgerOError,

    // No concensus during ledger operation
    LedgerNoConsensusError,

    // Attempt to send unknown or incomplete transaction message
    LedgerInvalidTransaction,

    // Attempt to send transaction without the necessary privileges
    LedgerSecurityError,

    // IO error during sending of ledger transactions or catchup process
    LedgerIOError,

    // Crypto errors
    // Invalid structure of any crypto promitives (keys, signatures, seeds and etc...)
    CryptoInvalidStructure = 400,

    // Unknown crypto type was requested for signing/verifiyng or encoding/decoding
    CryptoUnknownTypeError,

    // Revocation registry is full and creation of new registry is necessary
    CryptoRevocationRegistryFullError,

    CryptoInvalidUserRevocIndex,

    CryptoBackendError,

    AnoncredsNotIssuedError,

    // Attempt to generate master secret with dupplicated name
    AnoncredsMasterSecretDuplicateNameError
}
