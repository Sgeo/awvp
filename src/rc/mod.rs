pub mod aw;
pub mod vp;

use std::os::raw::c_int;

pub fn rc(vp: c_int) -> c_int {
    let result = match vp {
        vp::RC_SUCCESS => aw::RC_SUCCESS,
        vp::RC_VERSION_MISMATCH => aw::RC_VERSION_MISMATCH,
        vp::RC_NOT_INITIALIZED => aw::RC_NOT_INITIALIZED,
        vp::RC_ALREADY_INITIALIZED => aw::RC_NOT_INITIALIZED, // Hopefully shouldn't matter too much, vp::RC_ALREADY_INITIALIZED supposedly unused
        vp::RC_STRING_TOO_LONG => aw::RC_STRING_TOO_LONG,
        vp::RC_INVALID_LOGIN => aw::RC_INVALID_PASSWORD, // VP gives too little information on login failure, we'll just say it's the password that's wrong
        vp::RC_WORLD_NOT_FOUND => aw::RC_NO_SUCH_WORLD,
        vp::RC_WORLD_LOGIN_ERROR => aw::RC_UNABLE_TO_CONTACT_WORLD,
        vp::RC_NOT_IN_WORLD => aw::RC_NOT_AVAILABLE, // ???
        vp::RC_CONNECTION_ERROR => aw::RC_NO_CONNECTION, // Could be aw::RC_CONNECTION_LOST?
        vp::RC_NO_INSTANCE => aw::RC_NO_INSTANCE,
        vp::RC_NOT_IMPLEMENTED => aw::RC_NOT_AVAILABLE, // Well, this is my default now.
        vp::RC_NO_SUCH_ATTRIBUTE => aw::RC_INVALID_ATTRIBUTE,
        vp::RC_NOT_ALLOWED => aw::RC_UNAUTHORIZED,
        vp::RC_DATABASE_ERROR => aw::RC_DATABASE_ERROR,
        vp::RC_NO_SUCH_USER => aw::RC_NO_SUCH_CITIZEN,
        vp::RC_TIMEOUT => aw::RC_TIMEOUT,
        vp::RC_NOT_IN_UNIVERSE => aw::RC_NO_CONNECTION, // I guess?
        vp::RC_INVALID_ARGUMENTS => aw::RC_INVALID_ARGUMENT,
        vp::RC_OBJECT_NOT_FOUND => aw::RC_NO_SUCH_OBJECT, // Despite the similar names, aw::RC_CANT_FIND_OLD_ELEMENT also seems to make sense?
        vp::RC_UNKNOWN_ERROR => aw::RC_NOT_AVAILABLE, // Oh, if only AW SDK had UNKNOWN_ERROR
        vp::RC_RECURSIVE_WAIT => aw::RC_NOT_AVAILABLE, // Uh. I believe AW allows recursive waits, much to everyone's detriment. That's going to be fun to translate.
        vp::RC_JOIN_DECLINED => aw::RC_NOT_AVAILABLE, // AW bots won't be able to make use of Join feature anyway
        vp::RC_SECURE_CONNECTION_REQUIRED => aw::RC_NO_CONNECTION,
        vp::RC_HANDSHAKE_FAILED => aw::RC_NO_CONNECTION,
        vp::RC_VERIFICATION_FAILED => aw::RC_NO_CONNECTION,
        vp::RC_NO_SUCH_SESSION => aw::RC_NO_SUCH_SESSION,
        _ => aw::RC_NOT_AVAILABLE
    };
    if(vp != 0) {
        debug!("rc({:?}) = {:?}", vp, result);
    }
    result
}