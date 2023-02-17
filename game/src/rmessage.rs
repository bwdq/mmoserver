
const RMSG_NEWWDG: u8 = 0;
const RMSG_WDGMSG: u8 = 1;
const RMSG_DSTWDG: u8 = 2;
const RMSG_MAPIV: u8 = 3;
const RMSG_GLOBLOB: u8 = 4;
/* const RMSG_PAGINAE: u8 = 5; -- Deprecated */
const RMSG_RESID: u8 = 6;
/* const RMSG_PARTY: u8 = 7; -- Deprecated */
const RMSG_SFX: u8 = 8;
/* const RMSG_CATTR: u8 = 9; -- Deprecated */
const RMSG_MUSIC: u8 = 10;
/* const RMSG_TILES: u8 = 11; -- Deprecated */
/* const RMSG_BUFF: u8  = 12; -- Deprecated */

const RMSG_SESSKEY: u8 = 13;
const RMSG_FRAGMENT: u8 = 14;
const RMSG_ADDWDG: u8 = 15;
const RMSG_WDGBAR: u8 = 16;

struct RMessage {
    pub last: u64,
    //int retx
    //int seq
    pub data: Vec<u8>,
}