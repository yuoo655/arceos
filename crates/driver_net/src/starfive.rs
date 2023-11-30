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

        let device = StmmacDevice::new();

        device.dma_reset();
        device.dma_set_bus_mode();
        device.set_mac_addr();
        device.set_rxtx_base();
        device.set_mac_addr();
        device.core_init();
        device.stmmac_set_mac(true);
        device.dma_rxtx_enable();
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

        device.stmmac_mac_link_up();

        device.stmmac_set_mac(true);

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

        // self.device.tx_clean();
        Ok(())
    }

    fn receive(&mut self) -> DevResult<NetBufPtr> {
        use core::ptr::NonNull;
        if let Some((skb, len)) = self.device.receive() {
            let buffer_ptr = NonNull::new(skb).expect("-------");
            let packet_ptr = NonNull::new(skb).expect("-------");
            let net_buf = NetBufPtr::new(buffer_ptr, packet_ptr, len as usize);
            Ok(net_buf)
        } else {
            Err(DevError::Again)
        }
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        log::info!("--------transmit----------------");
        let packet_va: *mut u8 = tx_buf.raw_ptr();
        let packet_pa = A::virt_to_phys(packet_va as usize);
        let len = tx_buf.len;
        self.device.transmit(packet_pa as usize, len);
        Ok(())
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        use core::ptr::NonNull;
        let tskb_start = 0x1801_0000;
        let idx = self.device.tx_ring.idx;
        let buff_addr = A::phys_to_virt(tskb_start + 0x1000 * idx);

        log::info!("----------alloc_tx_buffer------idx{:#x?}", idx);
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
