use jsonrpc_core::{Error as RpcError, ErrorCode};

pub fn misc_rpc_error<T: std::fmt::Debug>(e: T) -> RpcError {
    RpcError {
        code: ErrorCode::ServerError(9876), // No real reason for this value
        message: "A miscellanous error occured".into(),
        data: Some(format!("{:?}", e).into()),
    }
}
