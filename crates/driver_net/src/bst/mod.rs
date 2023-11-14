use alloc::sync::Arc;
use bst_nic::Packet;
use driver_common::BaseDriverOps;
use driver_common::DevError;
use driver_common::DevResult;

use crate::bst::hardwrite_regs::reset;
use crate::NetDriverOps;
use crate::RxBuf;
use alloc::boxed::Box;
use core::any::Any;
use core::any::TypeId;

use core::marker::PhantomData;
use core::ptr;
use core::ptr::{read_volatile, write_volatile};

pub mod consts;
pub mod hardwrite_regs;

use hardwrite_regs::*;

use crate::TxBuf;

use super::BstNicDevice;
pub use super::BstNicTraits;

unsafe impl<A: BstNicTraits> Sync for BstNic<A> {}
unsafe impl<A: BstNicTraits> Send for BstNic<A> {}
pub struct BstNic<A>
where
    A: BstNicTraits,
{
    device: BstNicDevice<A>,
    phantom: PhantomData<A>,
}

impl<A> BstNic<A>
where
    A: BstNicTraits,
{
    pub fn init(traits_impl: A) -> Self {
        let pinctrlbase = 0x33001000;

        let vapinbase = A::phys_to_virt(pinctrlbase);

        info!("write pinctrl");

        unsafe {
            //rgmii0
            write_volatile((vapinbase + 0) as *mut u32, 0x80000000);
            write_volatile((vapinbase + 0) as *mut u32, 0);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffffffe);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffffffc);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffffff8);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffffff0);
            write_volatile((vapinbase + 4) as *mut u32, 0xffffffe0);
            write_volatile((vapinbase + 4) as *mut u32, 0xffffffc0);
            write_volatile((vapinbase + 4) as *mut u32, 0xffffff80);
            write_volatile((vapinbase + 4) as *mut u32, 0xffffff00);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffffe00);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffffc00);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffff800);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffff000);
            write_volatile((vapinbase + 4) as *mut u32, 0xffffe000);
            write_volatile((vapinbase + 4) as *mut u32, 0xffffc000);
            write_volatile((vapinbase + 4) as *mut u32, 0xffff8000);
            write_volatile((vapinbase + 4) as *mut u32, 0xffff0000);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffe0000);
            write_volatile((vapinbase + 4) as *mut u32, 0xfffc0000);
            write_volatile((vapinbase + 4) as *mut u32, 0xfff80000);
            write_volatile((vapinbase + 4) as *mut u32, 0xfff00000);
            write_volatile((vapinbase + 4) as *mut u32, 0xffe00000);
            write_volatile((vapinbase + 4) as *mut u32, 0xffa00000);
            write_volatile((vapinbase + 4) as *mut u32, 0xff800000);
            write_volatile((vapinbase + 8) as *mut u32, 0x1fdff);
        }

        info!("-----------------bst_nic init-----------------");

        let iobase = A::phys_to_virt(0x30000000);
        /*select phy*/
        bstgmac_select_phy::<A>(0);

        /*reset*/
        let top_crm = A::phys_to_virt(0x33000000);
        reset(top_crm);

        //dwmac_dma_reset
        dwmac_dma_reset::<A>(iobase, top_crm);

        //init_dma_desc_rings
        let device = BstNicDevice::new(0x30000000);

        dwmac_hard_write_regs(iobase);

        let mac_addr: [u8; 6] = [0xaa, 0xbb, 0xcc, 0xdd, 0x05, 0x06];
        let mut umac_tmp: u32 = (1 as u32) << 31;
        umac_tmp |= (mac_addr[5] as u32) << 8;
        umac_tmp |= mac_addr[4] as u32;
        unsafe {
            write_volatile((iobase + 0x300) as *mut u32, umac_tmp);
        }
        umac_tmp = ((mac_addr[3] as u32) << 24)
            | ((mac_addr[2] as u32) << 16)
            | ((mac_addr[1] as u32) << 8)
            | (mac_addr[0] as u32);
        unsafe {
            write_volatile((iobase + 0x304) as *mut u32, umac_tmp);
        }

        phylink_up::<A>(iobase);

        Self {
            device,
            phantom: PhantomData,
        }
    }
}

impl<A: BstNicTraits> BaseDriverOps for BstNic<A> {
    fn device_name(&self) -> &str {
        "bst_nic"
    }

    fn device_type(&self) -> driver_common::DeviceType {
        driver_common::DeviceType::Net
    }
}

impl<'a, A: BstNicTraits> NetDriverOps<'a> for BstNic<A> {
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

    fn fill_rx_buffers(&mut self, buf_pool: &'a crate::NetBufferPool) -> DevResult {
        Ok(())
    }

    fn prepare_tx_buffer(&self, tx_buf: &mut crate::NetBuffer, packet_len: usize) -> DevResult {
        Ok(())
    }

    fn recycle_rx_buffer(&mut self, rx_buf: crate::NetBufferBox<'a>) -> DevResult {
        Ok(())
    }

    fn receive(&mut self) -> DevResult<RxBuf<'a>> {
        use bst_nic::RxBuffer;
        if let Some(packet) = self.device.receive() {
            // info!("rxbuf.packet = {:x?}", packet.as_bytes());
            let rxbuf = RxBuffer { packet };
            Ok(RxBuf::BstNic(rxbuf))
        } else {
            Err(DevError::Again)
        }
    }

    fn transmit(&mut self, buf: TxBuf) -> DevResult {
        match buf {
            TxBuf::BstNic(tx_buf) => {
                self.device.transmit(tx_buf.packet);
                Ok(())
            }
            TxBuf::Virtio(_) => Err(DevError::BadState),
        }
    }

    fn alloc_tx_buffer(&self, size: usize) -> DevResult<TxBuf<'a>> {
        use bst_nic::TxBuffer;

        let idx = self.device.get_tx_idx();
        let skb_pa = 0x91000000 + idx * 0x1000;
        let skb_va = A::phys_to_virt(skb_pa);
        let new_va = skb_va + idx * 0x1000;
        let packet = Packet::new(new_va as *mut u8, size);
        let tx_buffer: TxBuffer = TxBuffer { packet };

        Ok(TxBuf::BstNic(tx_buffer))
    }
}

pub fn bstgmac_select_phy<A: BstNicTraits>(id: usize) {
    info!("bstgmac_select_phy id:{}", id);
    let addr = A::phys_to_virt(0x33000000);
    let new_addr = addr + 0x54;
    let mut reg_val = unsafe { read_volatile(new_addr as *mut u32) };
    reg_val |= 1 << id;
    unsafe {
        write_volatile(new_addr as *mut u32, reg_val as _);
    }

    let addr = A::phys_to_virt(0x33001008);
    let mut reg_val = unsafe { read_volatile(addr as *mut u32) };

    if id == 0 {
        reg_val &= !((1 << 11) | (1 << 15)); //gmac0: IN00:bit11 IN01:bit12  pps0:bit15
    } else if id == 1 {
        reg_val &= !((1 << 13) | (1 << 16)); //gmac1: IN10:bit13 IN11:bit14  pps1:bit16
    }
    unsafe {
        write_volatile(addr as *mut u32, reg_val as _);
    }
}

pub(crate) fn read_regs(iobase: usize) {
    let dma_ctrl_off = 0x1000;
    let dma_chan_off = 0x1100;
    let dwmac_reg_num = 55;

    {
        info!("=== === === DMA CHAN {}", 0);
        for j in 0..27 {
            let value = unsafe { read_volatile((iobase + dma_chan_off + j * 4) as *mut u32) };
            info!("Reg{} : {:08x}", j, value);
        }
    }
}
