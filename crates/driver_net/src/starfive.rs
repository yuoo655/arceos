pub use starfive_eth::{mdio_write, StarfiveHal, StmmacDevice};

use crate::{net_buf, EthernetAddress, NetBufPtr, NetDriverOps};
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};

unsafe impl<A: StarfiveHal> Sync for StarfiveNic<A> {}
unsafe impl<A: StarfiveHal> Send for StarfiveNic<A> {}

extern crate alloc;
use core::ptr::{self, read_volatile, write_volatile};
use core::{fmt::write, marker::PhantomData};

pub struct StarfiveNic<A>
where
    A: StarfiveHal,
{
    device: StmmacDevice<A>,
    phantom: PhantomData<A>,
}

impl<A: StarfiveHal> StarfiveNic<A> {
    pub fn init() -> Self {
        let ioaddr = A::phys_to_virt(0x1002_0000);

        dump_reg(ioaddr);

        log::info!("----------starfivenic init--------");
        mdio_write::<A>(ioaddr, 0xa001, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0x0, 0x7c3);
        mdio_write::<A>(ioaddr, 0xa012, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0x9000, 0x3);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa001, 0x783);
        mdio_write::<A>(ioaddr, 0xa003, 0x783);
        mdio_write::<A>(ioaddr, 0xfd, 0x7c3);
        mdio_write::<A>(ioaddr, 0xa001, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0x27, 0x783);
        mdio_write::<A>(ioaddr, 0xa00a, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);

        let mut device = StmmacDevice::new();

        log::info!("reset-----------------------");
        unsafe {
            log::info!("-------------dwmac_dma_reset--------------------");
            let mut value = read_volatile((ioaddr + DMA_BUS_MODE) as *mut u32);
            log::info!("write before value = {:#x?}", value);
            value |= DMA_BUS_MODE_SFT_RESET as u32;

            write_volatile((ioaddr + DMA_BUS_MODE) as *mut u32, value);
            A::mdelay(100);
            loop {
                let value = read_volatile((ioaddr + DMA_BUS_MODE) as *mut u32);
                log::info!("value = {:#x?}", value);
                if value != value & DMA_BUS_MODE_SFT_RESET as u32 {
                    break;
                }
            }
        }
        log::info!("-------------dwmac_dma_reset done--------------------");

        unsafe {
            write_volatile((ioaddr + DMA_BUS_MODE) as *mut u32, 0x910880);
            write_volatile((ioaddr + 0x1028) as *mut u32, 0xf0);
        }

        log::info!("set mac addr");
        let macid_lo = 0xddccbbaa;

        let macid_hi = 0x0605;

        unsafe {
            write_volatile((ioaddr + 0x40) as *mut u32, macid_hi);
        }

        unsafe {
            write_volatile((ioaddr + 0x44) as *mut u32, macid_lo);
        }

        device.set_rxtx_base();

        log::info!("enable mac core init ------------------");

        unsafe {
            write_volatile((ioaddr) as *mut u32, 0x618000);
        }

        device.stmmac_set_mac(true);


        // log::info!("-----------------------------stmmac_dma_operation_mode---------------------");
        // unsafe{
        //     write_volatile((ioaddr + DMA_CONTROL) as *mut u32, 0x900);
        //     write_volatile((ioaddr + DMA_CONTROL) as *mut u32, 0x0);
        // }

        unsafe {
            let mut value = read_volatile((ioaddr + DMA_CONTROL) as *mut u32);
            value |= 0x2 | DMA_CONTROL_ST;
            write_volatile((ioaddr + DMA_CONTROL) as *mut u32, value);
        }

        log::info!("---------phy starup---------");
        mdio_write::<A>(ioaddr, 0x27, 0x783);
        mdio_write::<A>(ioaddr, 0xa00a, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0x1de1, 0x103);
        mdio_write::<A>(ioaddr, 0x200, 0x243);
        mdio_write::<A>(ioaddr, 0x1200, 0x3);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);
        mdio_write::<A>(ioaddr, 0xa000, 0x783);

        // unsafe{
        //     write_volatile((ioaddr + 0x18) as *mut u32, 0xe);
        // }

        unsafe {
            write_volatile((ioaddr) as *mut u32, 0x61080c);
        }
        device.stmmac_set_mac(true);

        // loop {
        //     let rx_ring = &mut device.rx_ring;
        //     let rd_dma = &mut rx_ring.rd;
        //     let idx = rx_ring.idx;
        //     let rd = rd_dma.read_volatile(idx).unwrap();

        //     let rdes0 = rd.rdes0;

        //     let status = rdes0 & (1 << 31);

        //     if status >> 31 == 1 {
        //         log::info!("dma own");
        //         A::mdelay(1000);
        //         continue;
        //     }
        //     log::info!("rd {:x?}", rd);
        //     pub const DESC_RXSTS_FRMLENMSK: u32 = 0x3FFF << 16;
        //     pub const DESC_RXSTS_FRMLENSHFT: u32 = 16;

        //     let len = (rdes0 & DESC_RXSTS_FRMLENMSK) >> DESC_RXSTS_FRMLENSHFT;

        //     // get data from skb
        //     let skb_va = rx_ring.skbuf[idx] as *mut u8;

        //     // let packet: Packet = Packet::new(skb_va, len as usize);

        //     rx_ring.idx = (idx + 1) % 512;

        //     A::mdelay(100);
        //     // log::info!("packet {:x?}", packet.as_bytes());
        // }

        // log::info!("----------net init done");
        Self {
            device: device,
            phantom: PhantomData,
        }
    }
}

impl<A: StarfiveHal> BaseDriverOps for StarfiveNic<A> {
    fn device_name(&self) -> &str {
        "starfive"
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }
}

impl<A: StarfiveHal> NetDriverOps for StarfiveNic<A> {
    fn mac_address(&self) -> crate::EthernetAddress {
        crate::EthernetAddress([0xaa, 0xbb, 0xcc, 0xdd, 0x05, 0x06])
    }

    fn tx_queue_size(&self) -> usize {
        1
    }

    fn rx_queue_size(&self) -> usize {
        1
    }

    fn can_receive(&self) -> bool {
        true
    }

    fn can_transmit(&self) -> bool {
        true
    }

    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {

        self.device.rx_clean();
        
        Ok(())
    }

    fn recycle_tx_buffers(&mut self) -> DevResult {
        Ok(())
    }

    fn receive(&mut self) -> DevResult<NetBufPtr> {
        use core::ptr::NonNull;
        if let Some((skb, len)) = self.device.receive() {
            let buffer_ptr = NonNull::new(skb).expect("-------");
            let packet_ptr = NonNull::new(skb).expect("-------");
            let net_buf = NetBufPtr::new(buffer_ptr, packet_ptr, len as usize);
            // log::info!("packet {:x?}", net_buf.packet());
            Ok(net_buf)
        } else {
            Err(DevError::Again)
        }
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        log::info!("---------transmit----------------");
        let packet_va: *mut u8 = tx_buf.raw_ptr();
        let packet_pa = A::virt_to_phys(packet_va as usize);
        let len = tx_buf.len;

        self.device.transmit(packet_pa as usize, len);

        Ok(())
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        log::info!("---------alloc_tx_buffer----------------");
        use core::ptr::NonNull;
        let tskb_start = 0x1801_0000;
        let idx = self.device.tx_ring.idx;
        let buff_addr = A::phys_to_virt(tskb_start + 0x1000 * idx);
        let raw_ptr = buff_addr as *mut u8;
        let buffer_ptr = NonNull::new(raw_ptr).expect("-------");
        let packet_ptr = NonNull::new(raw_ptr).expect("-------");
        let net_buf = NetBufPtr::new(buffer_ptr, packet_ptr, size as usize);
        Ok(net_buf)
    }
}

pub fn dump_reg(ioaddr: usize) {
    log::info!("------------------------------dumpreg--------------------------------------");
    for i in 0..23 {
        let value = unsafe { read_volatile((ioaddr + 0x00001000 + i * 4) as *mut u32) };
        log::info!("reg {:?} = {:#x?}", i, value);
    }
}

// pub const MII_BUSY: u32 = (1 << 0);
// pub fn mdio_write<A: StarfiveHal>(ioaddr: usize, data: u32, value: u32) {

//     loop {
//         let value = unsafe { read_volatile((ioaddr + 0x10) as *mut u32) };

//         if value & MII_BUSY != 1 {
//             break;
//         }
//         A::mdelay(10);
//     }

//     unsafe{
//         write_volatile((ioaddr + 0x14) as *mut u32, data);
//         write_volatile((ioaddr + 0x10) as *mut u32, value);
//     }

//     loop {
//         let value = unsafe { read_volatile((ioaddr + 0x10) as *mut u32) };

//         if value & MII_BUSY != 1 {
//             break;
//         }
//         A::mdelay(10);
//     }
// }

pub const DMA_BUS_MODE: usize = 0x1000;

pub const DMA_CONTROL: usize = 0x00001018;

pub const DMA_BUS_MODE_SFT_RESET: usize = 0x00000001; /* Software Reset */

pub const DMA_RCV_BASE_ADDR: usize = 0x0000100c; /* Receive List Base */
pub const DMA_TX_BASE_ADDR: usize = 0x00001010; /* Transmit List Base */

/* DMA Control register defines */
pub const DMA_CONTROL_ST: u32 = 0x00002000; /* Start/Stop Transmission */
pub const DMA_CONTROL_SR: u32 = 0x00000002; /* Start/Stop Receive */

pub const MAC_ENABLE_TX: u32 = 1 << 3; /* Transmitter Enable */
pub const MAC_ENABLE_RX: u32 = 1 << 2; /* Receiver Enable */

pub const DMA_XMT_POLL_DEMAND: u32 = 0x00001004; /* Transmit Poll Demand */
pub const DMA_RCV_POLL_DEMAND: u32 = 0x00001008; /* Received Poll Demand */

pub const DESC_RXSTS_FRMLENMSK: u32 = 0x3FFF << 16;
pub const DESC_RXSTS_FRMLENSHFT: u32 = 16;

// mdio
pub const MII_BUSY: u32 = 1 << 0;
pub const MII_WRITE: u32 = 1 << 1;
pub const MII_CLKRANGE_60_100M: u32 = 0;
pub const MII_CLKRANGE_100_150M: u32 = 0x4;
pub const MII_CLKRANGE_20_35M: u32 = 0x8;
pub const MII_CLKRANGE_35_60M: u32 = 0xC;
pub const MII_CLKRANGE_150_250M: u32 = 0x10;
pub const MII_CLKRANGE_250_300M: u32 = 0x14;
pub const MIIADDRSHIFT: u32 = 11;
pub const MIIREGSHIFT: u32 = 6;
pub const MII_REGMSK: u32 = 0x1F << 6;
pub const MII_ADDRMSK: u32 = 0x1F << 11;

pub const SIFIVE_CCACHE_WAY_ENABLE: usize = 0x8;
