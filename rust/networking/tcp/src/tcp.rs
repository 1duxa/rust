use bitflags::bitflags;
use etherparse::{Ipv4Header, Ipv4HeaderSlice, TcpHeader, TcpHeaderSlice};
use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;
use std::{io, time};
use std::{io::Result, usize};

bitflags! {
    pub(crate) struct Availiable:u8 {
        const READ=0b00000001;
        const WRITE=0b00000010;
    }
}
#[derive(Debug)]
pub enum State {
    //Listen,
    SynRcvd,
    Established,
    FinWait1,
    FinWait2,
    TimeWait,
}
//
//impl Default for State {
//    fn default() -> Self {
//        State::Listen
//    }
//}
impl State {
    fn is_sync(&self) -> bool {
        match *self {
            State::SynRcvd => false,
            State::Established | State::FinWait1 | State::FinWait2 | State::TimeWait => true,
        }
    }
}
pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: RecvSequenceSpace,
    ip: Ipv4Header,
    tcp: TcpHeader,
    timers: Timers,

    pub(crate) incoming: VecDeque<u8>,
    pub(crate) unacked: VecDeque<u8>,
    pub(crate) closed: bool,
    closed_at: Option<u32>,
}
struct Timers {
    send_times: BTreeMap<u32, time::Instant>,
    srtt: f64,
}

impl Connection {
    pub(crate) fn is_rcv_closed(&self) -> bool {
        if let State::TimeWait = self.state {
            true
        } else {
            false
        }
    }
    fn availiable(&self) -> Availiable {
        let mut a = Availiable::empty();
        if self.is_rcv_closed() || !self.incoming.is_empty() {
            a |= Availiable::READ;
        }
        a
    }
}

pub struct SendSequenceSpace {
    // send unacknowledged
    una: u32,
    // send next
    nxt: u32,
    // send window
    wnd: u16,
    // urgent pointer
    up: bool,
    // seq num used last
    wl1: usize,
    //seq num acknowledged last
    wl2: usize,
    // Initial send seq num
    iss: u32,
}

//somewhat the same but recieve instead of send
pub struct RecvSequenceSpace {
    nxt: u32,
    wnd: u16,
    up: bool,
    irs: u32,
}

impl Connection {
    fn send_rst(&mut self, nic: &mut tun_tap::Iface) -> Result<()> {
        self.tcp.rst = true;
        self.tcp.sequence_number = 0;
        self.tcp.acknowledgment_number = 0;
        self.write(nic, self.send.nxt, 0)?;
        Ok(())
    }
    fn write<'a>(&mut self, nic: &mut tun_tap::Iface, seq: u32, mut limit: usize) -> Result<usize> {
        let mut buf = [0u8; 1500];
        self.tcp.sequence_number = seq;
        self.tcp.acknowledgment_number = self.recv.nxt;

        println!(
            "write(ack: {}, seq: {},limit: {}) syn {:?} fin {:?}",
            self.recv.nxt - self.recv.irs,
            seq,
            limit,
            self.tcp.syn,
            self.tcp.fin
        );
        let mut offset = seq.wrapping_sub(self.send.una) as usize;
        if let Some(closed_at) = self.closed_at {
            if seq == closed_at.wrapping_add(1) {
                offset = 0;
                limit = 0;
            }
        }
        println!(
            "using offset {} base {} in {:?}",
            offset,
            self.send.una,
            self.unacked.as_slices()
        );
        let (mut h, mut t) = self.unacked.as_slices();
        if h.len() >= offset {
            h = &h[offset..];
        } else {
            let skipped = h.len();
            h = &[];
            t = &t[(offset - skipped)..];
        }
        let max_data = std::cmp::min(limit, h.len() + t.len());

        let size = std::cmp::min(
            buf.len(),
            self.tcp.header_len() as usize + self.ip.header_len() as usize + max_data,
        );
        self.ip
            .set_payload_len(size - self.ip.header_len() as usize)
            .expect("BAD SEND");

        use std::io::Write;
        let buf_len = buf.len();
        let mut unwritten = &mut buf[..];

        self.ip.write(&mut unwritten)?;
        let ip_header_ends_at = buf_len - unwritten.len();

        unwritten = &mut unwritten[self.tcp.header_len() as usize..];
        let tcp_header_ends_at = buf_len - unwritten.len();

        let payload_bytes = {
            let mut written = 0;
            let mut limit = max_data;

            let p1l = std::cmp::min(limit, h.len());
            written += unwritten.write(&h[..p1l])?;
            limit -= written;

            let p2l = std::cmp::min(limit, t.len());
            written += unwritten.write(&t[..p2l])?;
            written
        };
        let payload_ends_at = buf_len - unwritten.len();
        self.tcp.checksum = self
            .tcp
            .calc_checksum_ipv4(&self.ip, &buf[tcp_header_ends_at..payload_ends_at])
            .expect("failed to calc checksum");
        let mut tcp_header_buf = &mut buf[ip_header_ends_at..tcp_header_ends_at];
        self.tcp.write(&mut tcp_header_buf);

        let mut next_seq = seq.wrapping_add(payload_bytes as u32);

        if self.tcp.syn {
            next_seq = next_seq.wrapping_add(1);
            self.tcp.syn = false;
        }
        if self.tcp.fin {
            next_seq = next_seq.wrapping_add(1);
            self.tcp.fin = false;
        }
        if wrapping_lt(self.send.nxt, next_seq) {
            self.send.nxt = next_seq;
        }
        self.timers.send_times.insert(seq, time::Instant::now());

        let _ = nic
            .send(&buf[..payload_ends_at])
            .expect("Failed to send unwritten");
        Ok(payload_bytes)

        //  (2^8 - 1) 255
        //  int8
        //
        //
        //  SYN ACK UNACK  0...........................SYN..........................ACK....10
        //
        //
        //
    }
    pub fn accept<'a>(
        nic: &mut tun_tap::Iface,
        iph: Ipv4HeaderSlice<'a>,
        tcph: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> Result<Option<Self>> {
        let buf = [0u8; 1504];

        if !tcph.syn() {
            eprintln!("Blocking non syn");
            //expect only packets SYN
            return Ok(None);
        }

        // ----------------------------------
        // \''''''''''''''''''''''''''''''''|
        // \''''''''''''''''''''''''''''''''|
        // \''''''''BLA BLA BLA'''''''''''''|
        // \''''''''''''''''''''''''''''''''|
        // \'''''''''''HELLO !''''''''''''''|
        // \'''''''''''MY NAME''''''''''''''|
        // \'''''''''''IS'''''''''''''''''''|
        // \'''''''''''ANDREW'''''''''''''''|
        // \''''''''''''''''''''''''''''''''|
        // \''''''''''''''''''''''''''''''''|
        // \'RECIEVING THE CONNECTION'''''''|
        // \''''''''''''''''''''''''''''''''|
        // ----------------------------------
        let wnd = 1024;
        let iss = 0;
        let mut c = Connection {
            timers: Timers {
                send_times: Default::default(),
                srtt: time::Duration::from_secs(1 * 60).as_secs_f64(),
            },
            state: State::SynRcvd,
            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: iss,
                wnd,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            recv: RecvSequenceSpace {
                irs: tcph.sequence_number(),
                nxt: (tcph.sequence_number() + 1),
                wnd: tcph.window_size(),
                up: false,
            },
            ip: etherparse::Ipv4Header::new(
                0,
                64,
                etherparse::IpNumber::TCP,
                [
                    iph.destination()[0],
                    iph.destination()[1],
                    iph.destination()[2],
                    iph.destination()[3],
                ],
                [
                    iph.source()[0],
                    iph.source()[1],
                    iph.source()[2],
                    iph.source()[3],
                ],
            )
            .expect("ERROR!! WHILE CREATING IP |:P|"),
            tcp: etherparse::TcpHeader::new(
                tcph.destination_port(),
                tcph.source_port(),
                iss as u32,
                10 as u16,
            ),
            incoming: Default::default(),
            unacked: Default::default(),
            closed: false,
            closed_at: None,
        };
        // keep track pf sender info
        /*        let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    c.send.iss as u32,
                    c.send.wnd as u16,
                );
        */
        c.tcp.syn = true;
        c.tcp.ack = true;
        c.write(nic, c.send.nxt, 0)?;
        // responding / reading
        //eprintln!("responding with {:02x?}", &buf[..&buf.len() - unwritten]);
        //eprintln!("reading tcp {:02x?}", tcph);
        //eprintln!("reading eth {:02x?}", iph);

        /* KERNEL DOES IT FOR US
        *     syn_ack.checksum = syn_ack
                    .calc_checksum_ipv4(&ip, &[])
                    .expect("FAILED TO COMPUTE CECKSUM XD");
        */

        Ok(Some(c))
    }
    pub(crate) fn on_tick(&mut self, nic: &mut tun_tap::Iface) -> Result<()> {
        if let State::FinWait2 | State::TimeWait = self.state {
            return Ok(());
        }
        let nunacked_data = self
            .closed_at
            .unwrap_or(self.send.nxt)
            .wrapping_sub(self.send.una);
        let nunsent_data = self.unacked.len() as u32 - nunacked_data;

        let waited_for = self
            .timers
            .send_times
            .range(self.send.una..)
            .next()
            .map(|t| t.1.elapsed());

        let should_retransmit = if let Some(waited_for) = waited_for {
            waited_for > time::Duration::from_secs(1)
                && waited_for.as_secs_f64() > 1.5 * self.timers.srtt
        } else {
            false
        };

        if should_retransmit {
            let resend = std::cmp::min(self.unacked.len() as u32, self.send.wnd as u32);
            if resend < self.send.wnd as u32 && self.closed {
                self.tcp.fin = true;
                self.closed_at = Some(self.send.una.wrapping_add(self.unacked.len() as u32));
            }
            self.write(nic, self.send.una, resend as usize)?;
        } else {
            if nunacked_data == 0 && self.closed_at.is_some() {
                return Ok(());
            }
            let allowed = self.send.wnd as u32 - nunacked_data;
            if allowed == 0 {
                return Ok(());
            }
            let send = std::cmp::min(nunsent_data, allowed);
            if send < allowed && self.closed && self.closed_at.is_none() {
                self.tcp.fin = true;
                self.closed_at = Some(self.send.una.wrapping_add(self.unacked.len() as u32));
            }
            self.write(nic, self.send.nxt, send as usize)?;
        }
        Ok(())
    }
    pub(crate) fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: Ipv4HeaderSlice<'a>,
        tcph: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> Result<Availiable> {
        //*
        //**UNA*->***ACK*->**NXT******************
        //0--U--------A-------N------------------> Than in goes again to zero
        //*
        //**ACK*->*NXT************************UNA*->*
        let seqn = tcph.sequence_number();
        let mut slen = data.len() as u32;

        if tcph.fin() {
            slen += 1;
        }
        if tcph.syn() {
            slen += 1;
        }
        // 0 len seg has separate rules
        let wend = self.recv.nxt.wrapping_add(self.recv.wnd as u32);
        let okay = if slen == 0 {
            if self.recv.wnd == 0 {
                if seqn != self.recv.nxt {
                    false
                } else {
                    true
                }
            } else if !is_wrapped_between(self.recv.nxt.wrapping_sub(1), seqn, wend) {
                false
            } else {
                true
            }
        } else {
            if self.recv.wnd == 0 {
                false
            } else if !is_wrapped_between(self.recv.nxt.wrapping_sub(1), seqn, wend)
                && !is_wrapped_between(
                    self.recv.nxt.wrapping_sub(1),
                    seqn.wrapping_add(slen - 1),
                    wend,
                )
            {
                false
            } else {
                true
            }
        };
        if !okay {
            eprintln!("!OKAY");
            self.write(nic, self.send.nxt, 0)?;
            return Ok(self.availiable());
        }
        if !tcph.ack() {
            if tcph.syn() {
                assert!(data.is_empty());
                self.recv.nxt = seqn.wrapping_add(1);
            }
            return Ok(self.availiable());
        }
        let ackn = tcph.acknowledgment_number();
        if let State::SynRcvd = self.state {
            if is_wrapped_between(
                self.send.una.wrapping_sub(1),
                ackn,
                self.send.nxt.wrapping_add(1),
            ) {
                self.state = State::Established;
            } else {
            }
        }

        if let State::Established | State::FinWait1 | State::FinWait2 = self.state {
            if is_wrapped_between(self.send.una, ackn, self.send.nxt.wrapping_add(1)) {
                println!(
                    "ack for {} (last: {}); prune in {:?}",
                    ackn, self.send.una, self.unacked
                );
                if !self.unacked.is_empty() {
                    let data_start = if self.send.una == self.send.iss {
                        self.send.una.wrapping_add(1)
                    } else {
                        self.send.una
                    };
                    let acked_data_end =
                        std::cmp::min(ackn.wrapping_sub(data_start) as usize, self.unacked.len());
                    self.unacked.drain(..acked_data_end);
                    let old = std::mem::replace(&mut self.timers.send_times, BTreeMap::new());

                    let una = self.send.una;
                    let srtt = &mut self.timers.srtt;
                    self.timers
                        .send_times
                        .extend(old.into_iter().filter_map(|(seq, send)| {
                            if is_wrapped_between(una, seq, ackn) {
                                *srtt = 0.8 * *srtt + (1.0 - 0.8) * send.elapsed().as_secs_f64();
                                None
                            } else {
                                Some((seq, send))
                            }
                        }));
                }
                self.send.una = ackn;
            }
        }
        if let State::FinWait1 = self.state {
            if let Some(closed_at) = self.closed_at {
                if self.send.una == closed_at.wrapping_add(1) {
                    self.state = State::FinWait2;
                }
            }
        }

        if !data.is_empty() {
            if let State::Established | State::FinWait1 | State::FinWait2 = self.state {
                let mut unread_data_at = self.recv.nxt.wrapping_sub(seqn) as usize;
                if unread_data_at > data.len() {
                    assert_eq!(unread_data_at, data.len() + 1);
                    unread_data_at = 0;
                }
                self.incoming.extend(&data[unread_data_at..]);
                self.recv.nxt = seqn.wrapping_add(data.len() as u32);
                self.write(nic, self.send.nxt, 0)?;
            }
        }
        if tcph.fin() {
            match self.state {
                State::FinWait2 => {
                    self.recv.nxt = self.recv.nxt.wrapping_add(1);
                    self.write(nic, self.send.nxt, 0)?;
                    self.state = State::TimeWait;
                }
                _ => unimplemented!(),
            }
        }
        Ok(self.availiable())
    }
    pub(crate) fn close(&mut self) -> Result<()> {
        self.closed = true;
        match self.state {
            State::SynRcvd | State::Established => {
                self.state = State::FinWait1;
            }
            State::FinWait1 | State::FinWait2 => {}
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "already closing",
                ))
            }
        };
        Ok(())
    }
}
pub fn wrapping_lt(lhs: u32, rhs: u32) -> bool {
    lhs.wrapping_sub(rhs) > (1 << 31)
}
pub fn is_wrapped_between(start: u32, x: u32, end: u32) -> bool {
    wrapping_lt(start, x) && wrapping_lt(x, end)
}
