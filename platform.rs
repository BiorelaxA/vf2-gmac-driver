use core::{alloc::Layout, ptr::{copy_nonoverlapping, NonNull}};

use alloc::{alloc::alloc_zeroed, boxed::Box, sync::Arc, vec::Vec};
use smoltcp::{phy::{DeviceCapabilities, Medium}, wire::EthernetAddress};
const NET_BUF_LEN: usize = 1536;
const BUF_LEN: usize = 1 << 12;
const QUEUE_SIZE: usize = 16;
/*
 * @Author: Peter/peterluck2021@163.com
 * @Date: 2025-08-10 11:39:56
 * @LastEditors: Peter/peterluck2021@163.com
 * @LastEditTime: 2025-08-20 21:51:03
 * @FilePath: /vf2-gmac-driver/platform.rs
 * @Description: 
 * 
 * Copyright (c) 2025 by peterluck2021@163.com, All Rights Reserved. 
 */
use crate::{arch::config::KERNEL_BASE, drivers::net::{netdevice::{NetBufPtr, NetDevice}, starfive::{drv_eth::{eth_handle_tx_over, eth_init, eth_rx, eth_tx}, eth_def::{VisionfiveGmac, DMA_CH0_RX_CONTROL, DMA_CH0_STATUS, DMA_CHAN_STATUS_AIS, DMA_CHAN_STATUS_ERI, DMA_CHAN_STATUS_ETI, DMA_CHAN_STATUS_FBE, DMA_CHAN_STATUS_RBU, DMA_CHAN_STATUS_RI, DMA_CHAN_STATUS_RPS, DMA_CHAN_STATUS_RWT, DMA_CHAN_STATUS_TBU, DMA_CHAN_STATUS_TI, DMA_CHAN_STATUS_TPS, EQOS_DMA_CH0_RX_CONTROL_SR}, eth_dev::{eth_mac_read_reg, eth_mac_set_bits}}, NetBuf, NetBufBox, NetBufPool}, task::wait_timeout, timer::TimeSpec};

pub fn plat_mdelay(m_times: usize) {
    //delay time m_times
}
pub fn plat_malloc_align(size:u64,align:u32)->u64 {
    //alloc align
    
}
pub fn plat_phys_to_virt(pa:u64)->u64 {
    //phy to virt
}
pub fn plat_virt_to_phys(va:u64)->u32 {
    //virt to phy
}

// #define RISCV_FENCE(p, s) \
//         __asm__ __volatile__ ("fence " #p "," #s : : : "memory")

// /* These barriers need to enforce ordering on both devices or memory. */
// #define mb()            RISCV_FENCE(iorw,iorw)
// #define rmb()           RISCV_FENCE(ir,ir)
// #define wmb()           RISCV_FENCE(ow,ow)
pub fn plat_fence(){
    unsafe {
        core::arch::asm!("fence iorw, iorw");
    }
}
pub fn plat_handle_tx_buffer(p: NetBufPtr, buffer: u64) -> u32 {
   //copy data from p to buffer,p may be your os struct
}
pub fn  plat_handle_rx_buffer(buffer: u64, length: u32) -> u64 {
    // buffer是接收到的数据，length是字节数
    // OS需要分配内存，memcpy接收到的数据，并将地址返回
    //这里直接分配空间，在后续的wrapper里面通过Netptr再返回即可
    //copy data from buffer to a pointer which will return 
}


