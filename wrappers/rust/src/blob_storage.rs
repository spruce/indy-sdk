use crate::{ErrorCode, IndyError};

use std::ffi::CString;

use ffi::blob_storage;
use ffi::ResponseI32CB;

use crate::utils::callbacks::{ClosureHandler, ResultHandler};
use crate::{CommandHandle, IndyHandle};

pub async fn open_reader(xtype: &str, config_json: &str) -> Result<IndyHandle, IndyError> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_handle();

    let err = _open_reader(command_handle, xtype, config_json, cb);

    ResultHandler::handle(command_handle, err, receiver).await
}

fn _open_reader(
    command_handle: CommandHandle,
    xtype: &str,
    config_json: &str,
    cb: Option<ResponseI32CB>,
) -> ErrorCode {
    let xtype = c_str!(xtype);
    let config_json = c_str!(config_json);

    ErrorCode::from(unsafe {
        blob_storage::indy_open_blob_storage_reader(
            command_handle,
            xtype.as_ptr(),
            config_json.as_ptr(),
            cb,
        )
    })
}

pub async fn open_writer(xtype: &str, config_json: &str) -> Result<CommandHandle, IndyError> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_handle();

    let err = _open_writer(command_handle, xtype, config_json, cb);

    ResultHandler::handle(command_handle, err, receiver).await
}

fn _open_writer(
    command_handle: CommandHandle,
    xtype: &str,
    config_json: &str,
    cb: Option<ResponseI32CB>,
) -> ErrorCode {
    let xtype = c_str!(xtype);
    let config_json = c_str!(config_json);

    ErrorCode::from(unsafe {
        blob_storage::indy_open_blob_storage_writer(
            command_handle,
            xtype.as_ptr(),
            config_json.as_ptr(),
            cb,
        )
    })
}
