use unbounded;
use UnboundedSocket;

#[deprecated(since = "0.1.2", note = "use `BoundedSocket` or `UnboundedSocket` instead")]
pub type Socket = UnboundedSocket;

#[deprecated(since = "0.1.2", note = "use `bounded` or `unbounded` instead")]
pub fn new() -> (UnboundedSocket, UnboundedSocket) {
    unbounded()
}
