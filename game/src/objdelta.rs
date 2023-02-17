use std::net::SocketAddr;
use bytes::{Buf, BufMut};
use crate::coord::{Coord, Coord2d};
use crate::message::{MessageType, Message, PMessage};

const OD_REM: u8 = 0;//remove
const OD_MOVE: u8 = 1;
const OD_RES: u8 = 2;
const OD_LINBEG: u8 = 3;
const OD_LINSTEP: u8 = 4;
const OD_SPEECH: u8 = 5;
const OD_COMPOSE: u8 = 6;
const OD_ZOFF: u8 = 7;
const OD_LUMIN: u8 = 8;
const OD_AVATAR: u8 = 9;
const OD_FOLLOW: u8 = 10;
const OD_HOMING: u8 = 11;
const OD_OVERLAY: u8 = 12;
/* const OD_AUTH = 13; -- Removed */
const OD_HEALTH: u8 = 14;
const OD_BUDDY: u8 = 15;
const OD_CMPPOSE: u8 = 16;
const OD_CMPMOD: u8 = 17;
const OD_CMPEQU: u8 = 18;
const OD_ICON: u8 = 19;
const OD_RESATTR: u8 = 20;
const OD_END: u8 = 255;


const COMP_OD_MAP: [u8; 8] = [OD_REM, OD_RESATTR, OD_FOLLOW, OD_MOVE, OD_RES, OD_LINBEG, OD_LINSTEP, OD_HOMING];

//rep u8
#[repr(u8)]
#[derive(Clone, Copy)]
enum obj_type {
    OD_REM = OD_REM,
    OD_MOVE = OD_MOVE,
    OD_RES = OD_RES,
    OD_LINBEG = OD_LINBEG,
    OD_LINSTEP = OD_LINSTEP,
    OD_SPEECH = OD_SPEECH,
    OD_COMPOSE = OD_COMPOSE,
    OD_ZOFF = OD_ZOFF,
    OD_LUMIN = OD_LUMIN,
    OD_AVATAR = OD_AVATAR,
    OD_FOLLOW = OD_FOLLOW,
    OD_HOMING = OD_HOMING,
    OD_OVERLAY = OD_OVERLAY,
    OD_HEALTH = OD_HEALTH,
    OD_BUDDY = OD_BUDDY,
    OD_CMPPOSE = OD_CMPPOSE,
    OD_CMPMOD = OD_CMPMOD,
    OD_CMPEQU = OD_CMPEQU,
    OD_ICON = OD_ICON,
    OD_RESATTR = OD_RESATTR,
    OD_END = OD_END,
}



struct ObjDelta {
    flags: u8,
    id: u32,
    frame: i32,
    deltas: Vec<obj_type>
}
impl ObjDelta {

    pub fn ser(len: u8, o_type: obj_type) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        let mut obj_type: u8 = 0;
        //if type is in compod and len is 0 or 2-16 use compressed format
        if len == 0 || (len > 1 && len <= 16) && COMP_OD_MAP.contains(&(o_type as u8)) {
            obj_type = len << 3;
            //get index of obj_type in compodmap
            let pos = COMP_OD_MAP.iter().position(|&r| r == (o_type as u8)).unwrap();
            obj_type = obj_type | (pos as u8 & 0x07);
            buf.push(obj_type);
        } else {//use expanded format, set high bit of obj_type
            obj_type = 0x80 | o_type as u8;
            buf.push(obj_type);
            //len u8 or u16
            if len > 0x7f {
                buf.push(0x80);
                buf.put_u16(len as u16);
            } else {
                buf.push(len & 0x7F);
            }
        }
        return buf;
    }
}

impl ObjDelta {

    pub fn parse(mut msg: Message) -> ObjDelta {
        let flags = msg.msg_data.get_u8();
        let id = msg.msg_data.get_u32();
        let frame = msg.msg_data.get_i32();
        let mut deltas: Vec<obj_type> = Vec::new();
        while msg.msg_data.len() > 0 {
            let mut obj_type = msg.msg_data.get_u8();
            let mut len: u16 = 0;
            if obj_type & 0x80 == 0 {//compact mode
                len = ((obj_type & 0x78) >> 3) as u16;
                if len > 0 {
                    len = len +1;
                }
                obj_type = COMP_OD_MAP[((obj_type as u8) & 0x07) as usize];
            }
            else {
                obj_type = obj_type & 0x7f;//0111 1111
                let afl = msg.msg_data.get_u8();
                if (afl & 0x80) == 0 {
                    len = (afl & 0x7f) as u16
                } else {
                    len = msg.msg_data.get_u16();
                }
            }
            let buf = msg.msg_data.split_to(len as usize);
            let mut attr = PMessage::new(obj_type);
            attr.msg.put(buf);
        }
        ObjDelta {
            flags,
            id,
            frame,
            deltas
        }
    }
}

/*

if type is in compodmap, thentype is the index in the compodmap, 3 bits
len must be 0 or 2-16  (0 through 15 shifted to 0, 2-16)

else set lead bit of type
 */


// impl ObjDelta {
//
//     // pub fn add_rem(&self) {
//     //     self.bytes.put()
//     // }
//
//     pub fn serialize(&self, sock_addr: SocketAddr) -> Message {
//
//         let mut buf: Vec<u8> = Vec::new();
//         buf.put_u8(self.flags);
//         buf.put_u32(self.id);
//         buf.put_i32(self.frame);
//
//         for delta in &self.deltas {
//             match delta {
//                 obj::OD_REM() => {
//                     buf.put_u8(OD_REM);
//                 },
//                 obj::OD_MOVE(name) => {
//                     buf.put_u8(OD_MOVE);
//                     buf.put_u8(name.len() as u8);
//                     buf.put(name.as_bytes());
//                 }
//                 obj::OD_RES() => {}
//                 obj::OD_LINBEG() => {}
//                 obj::OD_LINSTEP() => {}
//                 obj::OD_SPEECH() => {}
//                 obj::OD_COMPOSE() => {}
//                 obj::OD_ZOFF() => {}
//                 obj::OD_LUMIN() => {}
//                 obj::OD_AVATAR() => {}
//                 obj::OD_FOLLOW() => {}
//                 obj::OD_HOMING() => {}
//                 obj::OD_OVERLAY() => {}
//                 obj::OD_HEALTH() => {}
//                 obj::OD_BUDDY() => {}
//                 obj::OD_CMPPOSE() => {}
//                 obj::OD_CMPMOD() => {}
//                 obj::OD_CMPEQU() => {}
//                 obj::OD_ICON() => {}
//                 obj::OD_RESATTR() => {}
//                 obj::OD_END() => {}
//             }
//         }
//
//         let mut msg = Message::new(sock_addr, MessageType::ObjectData);
//         msg.msg_data = buf;
//         return msg;
//     }
// }

struct Speech {//speaking class
    zo: i16,//z offset, i16 * 100.0f
    text: String,
}

struct composite {//composite class
    res: u16
}

struct zoff {//draw offset class
    off: i16//z offset, i16 * 100.0f
}

struct lumin {//lumin class
    off: Coord,
    sz: u16,//size
    str: u8,//strength
}

struct avatar {//avatar class
    layers: Vec<u16>,//65535 is end of list
}

struct follow {//following class
    oid: u32,//0xffffffff delete follow.class?
    res: u16,//?
    xfname: String,
}

struct homing {//homing class
    oid: u32,//0xffffffff delete homing.class?
    tc: Coord2d,//Coord2d tc = msg.coord().mul(OCache.posres);
    v: f32,//double v = msg.int32() * 0x1p-10 * 11;
}

struct Overlay {//OCache class //TODO figure out overlay OCache 273-318
    olidf: i32,
    resid: u16
}

struct health {//GobHealth class
    hp: u8,//g.setattr(new GobHealth(g, hp / 4.0f));
}

struct buddy {//KinInfo class
    name: String,//length 0 g.delattr(KinInfo.class);
    group: u8,
    btype: u8,
}

struct cmppose {//Composite class //TODO figure out composite 198-246
    pfl: u8,
    pseq: u8,
    resids: Vec<u16>,
    //interp: bool,//0001 last bit
    //pflag 2nd bit 0010 parse list
}

struct cmpmod {//Composite class //TODO figure out composite 250-278
    modid: u16,//65535 is end of list
    resids: Vec<u16>,//65535 is end of list
}

struct cmpequ {//Composite class //TODO figure out composite

}

