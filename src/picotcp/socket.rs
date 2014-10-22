#![allow(non_camel_case_types)]

extern crate libc;
use pico_ipv4::*;
use libc::{c_int};
use std::mem;

#[repr(C)]
pub struct pico_socket;

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_socket_open(net: u16, proto: u16, wakeup: fn(ev: u16, sock: &pico_socket))->*mut pico_socket; 
    pub fn pico_socket_bind(s: *mut pico_socket, address: *mut u8, port: *mut u16)->c_int; 
    pub fn pico_socket_listen(s: *mut pico_socket, backlog: c_int)->c_int; 
    pub fn pico_socket_accept(s: &pico_socket, address: *mut u8, port: *mut u16)->*mut pico_socket; 
    pub fn pico_socket_recv(s: &pico_socket, buf: *mut u8, len: c_int)->c_int;
}


/* 
 * Constants
 */

#[allow(dead_code)]
pub static PICO_PROTO_IPV4:u16 = 0;
#[allow(dead_code)]
pub static PICO_PROTO_IPV6:u16 = 41;
#[allow(dead_code)]
pub static PICO_PROTO_TCP:u16 = 6;
#[allow(dead_code)]
pub static PICO_PROTO_UDP:u16 = 17;

#[allow(dead_code)]
pub static PICO_SOCK_EV_RD:u16 = 1;
#[allow(dead_code)]
pub static PICO_SOCK_EV_WR:u16 = 2;
#[allow(dead_code)]
pub static PICO_SOCK_EV_CONN:u16 = 4;
#[allow(dead_code)]
pub static PICO_SOCK_EV_CLOSE:u16 = 8;
#[allow(dead_code)]
pub static PICO_SOCK_EV_FIN:u16 = 0x10;
#[allow(dead_code)]
pub static PICO_SOCK_EV_ERR:u16 = 0x80;

static MAXLEN:uint = 1500;

/* 
 * RUST FUNCTION INTERFACE
 */

pub fn socket(net:u16, proto:u16, wakeup: fn(u16, &pico_socket)) -> *mut pico_socket
{
    unsafe { pico_socket_open(net, proto, wakeup) as *mut pico_socket}
}

pub fn bind(s: *mut pico_socket, address: *mut pico_ip4, port: *mut u16) -> (int, u16)
{
    unsafe {
        let mut be_port: u16 = *port;
        be_port = be_port.to_be();
        (pico_socket_bind(s, address as *mut u8, &mut be_port) as int, be_port.to_be())
    }
}

pub fn listen(s: *mut pico_socket, backlog: int) -> int
{
    unsafe { pico_socket_listen(s, backlog as c_int) as int }
}

pub fn accept(s: &pico_socket, address: *mut pico_ip4, port: *mut u16) -> *mut pico_socket
{
    unsafe {
        pico_socket_accept(s, address as *mut u8, port)
    }
}

pub fn recv(s: &pico_socket)-> Vec<u8>
{
    unsafe {
        let mut buf:Vec<u8> = Vec::with_capacity(MAXLEN );
        let p = buf.as_mut_ptr();
        mem::forget(buf); /* Cast into the void to allow raw access */

        /* Recv stream/dgram */
        let mut r = pico_socket_recv(s, p, MAXLEN as i32);

        if r < 0 { r = 0;}
        Vec::from_raw_parts(r as uint, MAXLEN , p)
    }
}
