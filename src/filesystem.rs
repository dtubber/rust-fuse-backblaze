use fuse::*;

use std::os::raw::*;
use std::ffi::*;

pub struct BblzFilesystem {
    client: reqwest::Client,
    auth_key: String
}

impl Filesystem for BblzFilesystem {
    fn init(&mut self, _req: &Request) -> Result<(), i32> {
        Ok(())
    }

    fn readdir(&mut self, _req: &Request, _ino: u64, _fh: u64, _offset: i64, reply: ReplyDirectory) {
        
    }
}
