extern crate libc;

use api::ErrorCode;

use self::libc::{c_char, c_uchar};

/// Creates a new local pool ledger that can be used later to connect pool nodes.
///
/// #Params
/// name: Name of the pool
/// config (optional): Pool configuration json. if NULL, then default config will be used. Example:
/// {
///     "genesis_txn": string (optional), A path to genesis transaction file. If NULL, then a default one will be used.
///                    If file doesn't exists default one will be created.
/// }
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn sovrin_create_pool_ledger(command_handle: i32,
                                        name: *const c_char,
                                        config: *const c_char,
                                        cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

/// Opens pool ledger and performs connecting to pool nodes.
///
/// Pool with corresponded name must be previously created with sovrin_create_pool method.
/// It is impossible to open pool with the same name more than once.
///
/// name: Name of the pool.
/// config (optional): Runtime pool configuration json.
///                         if NULL, then default config will be used. Example:
/// {
///     "refreshOnOpen": bool (optional), Forces pool ledger to be refreshed immediately after opening.
///                      Defaults to true.
///     "autoRefreshTime": int (optional), After this time in minutes pool ledger will be automatically refreshed.
///                        Use 0 to disable automatic refresh. Defaults to 24*60.
///     "networkTimeout": int (optional), Network timeout for communication with nodes in milliseconds.
///                       Defaults to 20000.
/// }
///
/// #Returns
/// Handle to opened pool to use in methods that require pool connection.
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn sovrin_open_pool_ledger(command_handle: i32,
                                      name: *const c_char,
                                      config: *const c_char,
                                      cb: extern fn(xcommand_handle: i32, err: ErrorCode, pool_handle: i32)) -> ErrorCode {
    unimplemented!();
}

/// Refreshes a local copy of a pool ledger and updates pool nodes connections.
///
/// #Params
/// handle: pool handle returned by sovrin_open_pool_ledger
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn sovrin_refresh_pool_ledger(command_handle: i32,
                                         handle: i32,
                                         cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

/// Closes opened pool ledger, opened nodes connections and frees allocated resources.
///
/// #Params
/// handle: pool handle returned by sovrin_open_pool_ledger.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn sovrin_close_pool_ledger(command_handle: i32,
                                       handle: i32,
                                       cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

/// Deletes created pool ledger.
///
/// #Params
/// name: Name of the pool ledger to delete.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Ledger*
#[no_mangle]
pub extern fn sovrin_delete_pool_ledger(command_handle: i32,
                                        name: *const c_char,
                                        cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}