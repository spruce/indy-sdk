use crate::{ErrorCode, IndyError};

use ffi::metrics;

use crate::utils::callbacks::{ClosureHandler, ResultHandler};

use crate::CommandHandle;
use ffi::ResponseStringCB;

/// Collect metrics from libindy.
///
/// # Returns
/// String with a dictionary of metrics in JSON format. Where keys are names of metrics.
pub async fn collect_metrics() -> Result<String, IndyError> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _collect_metrics(command_handle, cb);

    ResultHandler::str(command_handle, err, receiver).await
}

fn _collect_metrics(command_handle: CommandHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe { metrics::indy_collect_metrics(command_handle, cb) })
}
