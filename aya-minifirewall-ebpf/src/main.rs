#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::HashMap,
    programs::XdpContext,
};
use aya_log_ebpf::info;
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};

#[map]
static BLOCKED_IPS: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[map]
static BLOCKED_PORTS: HashMap<u16, u32> = HashMap::with_max_entries(1024, 0);

#[xdp]
pub fn aya_minifirewall(ctx: XdpContext) -> u32 {
    match try_aya_minifirewall(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_aya_minifirewall(ctx: XdpContext) -> Result<u32, u32> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => {}
        _ => return Ok(xdp_action::XDP_PASS),
    }

    let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
    let source_ip = u32::from_be(unsafe { (*ipv4hdr).src_addr });
    let dest_ip = u32::from_be(unsafe { (*ipv4hdr).dst_addr });
    let protocol = unsafe { (*ipv4hdr).proto };

    // 检查源IP是否被阻止
    if BLOCKED_IPS.get(&source_ip, 0).is_some() {
        info!(&ctx, "Blocked packet from blocked source IP: {}", source_ip);
        return Ok(xdp_action::XDP_DROP);
    }

    // 检查目标IP是否被阻止
    if BLOCKED_IPS.get(&dest_ip, 0).is_some() {
        info!(
            &ctx,
            "Blocked packet to blocked destination IP: {}", dest_ip
        );
        return Ok(xdp_action::XDP_DROP);
    }

    // 检查端口（仅对TCP和UDP）
    if protocol == IpProto::Tcp || protocol == IpProto::Udp {
        let source_port = get_source_port(&ctx, ipv4hdr, protocol)?;
        let dest_port = get_dest_port(&ctx, ipv4hdr, protocol)?;

        if BLOCKED_PORTS.get(&source_port, 0).is_some() {
            info!(
                &ctx,
                "Blocked packet from blocked source port: {}", source_port
            );
            return Ok(xdp_action::XDP_DROP);
        }

        if BLOCKED_PORTS.get(&dest_port, 0).is_some() {
            info!(
                &ctx,
                "Blocked packet to blocked destination port: {}", dest_port
            );
            return Ok(xdp_action::XDP_DROP);
        }
    }

    Ok(xdp_action::XDP_PASS)
}

fn get_source_port(
    ctx: &XdpContext,
    ipv4hdr: *const Ipv4Hdr,
    protocol: IpProto,
) -> Result<u16, u32> {
    let offset = EthHdr::LEN + (unsafe { (*ipv4hdr).ihl } as usize * 4);
    match protocol {
        IpProto::Tcp => {
            let tcphdr: *const TcpHdr = unsafe { ptr_at(ctx, offset)? };
            Ok(u16::from_be(unsafe { (*tcphdr).source }))
        }
        IpProto::Udp => {
            let udphdr: *const UdpHdr = unsafe { ptr_at(ctx, offset)? };
            Ok(u16::from_be(unsafe { (*udphdr).source }))
        }
        _ => Err(1),
    }
}

fn get_dest_port(ctx: &XdpContext, ipv4hdr: *const Ipv4Hdr, protocol: IpProto) -> Result<u16, u32> {
    let offset = EthHdr::LEN + (unsafe { (*ipv4hdr).ihl } as usize * 4);
    match protocol {
        IpProto::Tcp => {
            let tcphdr: *const TcpHdr = unsafe { ptr_at(ctx, offset)? };
            Ok(u16::from_be(unsafe { (*tcphdr).dest }))
        }
        IpProto::Udp => {
            let udphdr: *const UdpHdr = unsafe { ptr_at(ctx, offset)? };
            Ok(u16::from_be(unsafe { (*udphdr).dest }))
        }
        _ => Err(1),
    }
}

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, u32> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(1);
    }

    Ok((start + offset) as *const T)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
