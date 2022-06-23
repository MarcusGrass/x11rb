use crate::errors::ReplyError;
use crate::x11_utils::{TryParse, X11Error};
use crate::SocketConnection;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct VoidCookie {
    seq: u16,
}

impl VoidCookie {
    #[must_use]
    pub(crate) fn new(seq: u16) -> Self {
        Self { seq }
    }

    pub fn check(self, connection: &mut SocketConnection) -> Result<(), ReplyError> {
        if let Some(err) = connection.block_check_for_err(self.seq)? {
            return Err(ReplyError::X11Error(X11Error::try_parse(
                &err,
                &connection.extensions,
            )?));
        }
        Ok(())
    }

    #[must_use]
    pub fn sequence_number(&self) -> u16 {
        self.seq
    }

    pub fn forget(self, connection: &mut SocketConnection) {
        connection.forget(self.seq);
    }
}

#[derive(Debug)]
pub struct Cookie<R>
where
    R: TryParse,
{
    seq: u16,
    phantom: PhantomData<R>,
}

impl<R> Cookie<R>
where
    R: TryParse,
{
    #[must_use]
    pub(crate) fn new(seq: u16) -> Self {
        Self {
            seq,
            phantom: PhantomData::default(),
        }
    }

    pub fn reply(self, connection: &mut SocketConnection) -> Result<R, ReplyError> {
        let buf = connection.block_for_reply(self.seq)?;
        if let Ok(res) = R::try_parse(&buf) {
            Ok(res.0)
        } else {
            Err(ReplyError::X11Error(X11Error::try_parse(
                &buf,
                &connection.extensions,
            )?))
        }
    }

    #[must_use]
    pub fn sequence_number(&self) -> u16 {
        self.seq
    }

    pub fn forget(self, connection: &mut SocketConnection) {
        connection.forget(self.seq);
    }
}
