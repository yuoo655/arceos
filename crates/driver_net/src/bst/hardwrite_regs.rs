use driver_common::{DevError, DevResult};
use core::ptr::{write_volatile,read_volatile};
use super::consts::*;
use super::BstNicTraits;


/// Memory Barrier
pub fn dmb() {
    unsafe {
        core::arch::asm!("dmb sy");
    }
}
fn bst_assert(top_crm: usize)
{
    let addr=top_crm + 0x2180;
    let reg = 0xc03;
    unsafe{ 
        let reg_read_value = read_volatile(addr as *mut u32);
        write_volatile(addr as *mut u32, reg as _); 
    }
}
fn bst_deassert(top_crm: usize)
{

    let addr=top_crm + 0x2180;
    let reg = 0xc0b;
    unsafe{ 
        let reg_read_value = read_volatile(addr as *mut u32);
        write_volatile(addr as *mut u32, reg as _); 
    }
}
pub fn reset(top_crm: usize)
{
    bst_assert(top_crm);
    bst_deassert(top_crm);
}
pub fn dwmac_dma_reset<A: BstNicTraits>(iobase: usize, top_crm: usize)
{
    unsafe{
        let mut value:u32 = read_volatile((iobase + DMA_BUS_MODE) as *mut u32);
        value |= DMA_BUS_MODE_SFT_RESET;
        write_volatile((iobase + DMA_BUS_MODE) as *mut u32 , value as _);
        let mut reg_value:u32 = read_volatile((top_crm + 0x80) as *mut u32);
        reg_value &= 1 << 7;
        if reg_value!=0 {
            A::mdelay(100);
            write_volatile( (iobase + 0xce0) as *mut u32 , 0x5);
            A::mdelay(1);
            write_volatile((iobase + 0xcc0) as *mut u32 , 0x11f);
            A::mdelay(1);
            write_volatile((iobase + 0xcc8) as *mut u32 , 0x1111);
            A::mdelay(10);
        }

        let mut limit = 10;


        while limit > 0{
            let value = read_volatile((iobase + DMA_BUS_MODE) as *mut u32);
            if value & (DMA_BUS_MODE_SFT_RESET as u32) == 0 {
                break;
            }

            A::mdelay(10);
            limit -= 1;
        }
        if limit < 0 {
            info!("dwmac dma reset fail limit{:?}", limit);
        }
        else
        {
            info!("dwmac dma reset success limit{:?}", limit);
        }
    }
}
pub fn dwmac_hard_write_regs(iobase: usize)
{
    let mut offset:usize=0;
	let mut i:u32=0;

    unsafe
    {
        write_volatile((iobase + 4 * 40) as *mut u32, 0x000000AA);
        write_volatile((iobase + 4 * 42) as *mut u32, 0x03020100);
        write_volatile((iobase + 4 * 44) as *mut u32, 0x00040008);
        write_volatile((iobase + 4 * 45) as *mut u32, 0x00000010);
        write_volatile((iobase + 4 * 53) as *mut u32, 0x03e80000);
    /*MTL regs*/

        write_volatile((iobase + 0xc30) as *mut u32,0x03020100);
        offset=0x00000d00;

        write_volatile((iobase + offset ) as *mut u32, 0x000f000a);
        write_volatile((iobase + offset + 0x18) as *mut u32, 0x00000010);
        write_volatile((iobase + offset + 0x30 ) as *mut u32, 0x00f0c1b0);

    /*DMA ctrl*/

        offset = 0x00001000;
        write_volatile((iobase + offset) as *mut u32, 0x00010000);
        write_volatile((iobase + offset + 4) as *mut u32, 0x0f0f080f);
    /*DMA chan0*/	

        offset = 0x00001100;

            write_volatile((iobase + offset ) as *mut u32, 0x00010000);
            write_volatile((iobase + offset + 4 *2) as *mut u32, 0x00080C01);
            write_volatile((iobase + offset + 4*11) as *mut u32, 0x000001FF);
            write_volatile((iobase + offset + 4*12) as *mut u32, 0x000001FF);
            
            let mut value = 0;
            value |= 1 << 0;
            value |= 1 << 6;

            write_volatile((iobase + offset + 4*13 ) as *mut u32, value);
            write_volatile((iobase + offset + 4*15) as *mut u32, 0x000007C0);
            write_volatile((iobase + offset + 4) as *mut u32, 0x00080011);

        write_volatile(iobase as *mut u32, 0x0800E003);
    }
}
pub fn dwmac_dma_set_rx_tail_ptr(iobase:usize, tail_ptr:u32, chan:u32)
{
	// writel(tail_ptr , ioaddr + 0x1100 + chan * DWMAC_CHAN_REG_LEN + 0x28);
    unsafe{
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x28) as *mut u32, tail_ptr as _);
    }
}
pub fn dwmac_dma_set_tx_tail_ptr(iobase:usize, tail_ptr:u32, chan:u32)
{
	// writel(tail_ptr , ioaddr + 0x1100 + chan * DWMAC_CHAN_REG_LEN + 0x20);
    unsafe{
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x20) as *mut u32, tail_ptr as _);
    }
}
pub fn dwmac_dma_init_tx_desc(iobase:usize, dma_tx_phy:usize,chan:u32)
{
	// writel(upper_32_bits(dma_tx_phy), ioaddr + 0x1100 + chan * DWMAC_CHAN_REG_LEN + 0x10);
	// writel(lower_32_bits(dma_tx_phy), ioaddr + 0x1100 + chan * DWMAC_CHAN_REG_LEN + 0x14);
    let upper_bits = (dma_tx_phy>>32) & 0xffffffff;
    let lower_bits = dma_tx_phy & 0xffffffff;
    unsafe{
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x10) as *mut u32, upper_bits as _);
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x14) as *mut u32, lower_bits as _);
    }
}
pub fn dwmac_dma_init_rx_desc(iobase:usize, dma_tx_phy:usize,chan:u32)
{
	// writel(upper_32_bits(dma_rx_phy), ioaddr + 0x1100 + chan * DWMAC_CHAN_REG_LEN + 0x18);
	// writel(lower_32_bits(dma_rx_phy), ioaddr + 0x1100 + chan * DWMAC_CHAN_REG_LEN + 0x1c);
    let upper_bits = (dma_tx_phy>>32) & 0xffffffff;
    let lower_bits = dma_tx_phy & 0xffffffff;
    unsafe{
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x18) as *mut u32, upper_bits as _);
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x1c) as *mut u32, lower_bits as _);
    }
}
pub fn gmac_dma_ti_interrupt(iobase:usize,chan:u32) -> u32
{
    unsafe{
        let mut ret:u32=0;
        let mut intr_status:u32 = read_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x60) as *mut u32);
        let intr_status_tbu = intr_status & DMA_CHAN_STATUS_TBU;
        if intr_status_tbu != 0 {
            ret |= HANDLE_TX;
            write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x60) as *mut u32,DMA_CHAN_STATUS_TBU | DMA_CHAN_STATUS_AIS);
        }
        let intr_status_ti=intr_status & DMA_CHAN_STATUS_TI;
        if intr_status_ti!=0 {
            ret |= HANDLE_TX;
            write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x60) as *mut u32, DMA_CHAN_STATUS_TI | DMA_CHAN_STATUS_NIS);
        }
        return ret;	
    }
}
pub fn gmac_dma_ri_interrupt(iobase:usize,chan:u32) -> u32
{
    unsafe{
        let mut ret:u32=0;
        let mut intr_status:u32 = read_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x60) as *mut u32);

        let intr_status_rbu = intr_status & DMA_CHAN_STATUS_RBU;
        if intr_status_rbu != 0 {
            ret |= HANDLE_RX;
            write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x60) as *mut u32, DMA_CHAN_STATUS_RBU | DMA_CHAN_STATUS_AIS);
        }
        let intr_status_ri = intr_status & DMA_CHAN_STATUS_RI;
        if intr_status_ri != 0 {
            ret |= HANDLE_RX;
            write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x60) as *mut u32, DMA_CHAN_STATUS_RI | DMA_CHAN_STATUS_NIS);
        }
        return ret;
    }
}
pub fn enable_dma_irq_bits(iobase:usize, chan:u32, bits:u32)
{
    unsafe{
        let mut value:u32 = read_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x34) as *mut u32);
        value |= bits;
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x34) as *mut u32, value as _);

        dmb();
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x34) as *mut u32, value as _);
        dmb();

        value = read_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x34) as *mut u32);
        value &= bits;
        if value == 0 {
            value |= bits;
            write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x34) as *mut u32, value as _);
            dmb();
        }
    }
}
pub fn disable_dma_irq_bits(iobase:usize, chan:u32, bits:u32)
{
    unsafe{
        let mut value:u32 = read_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x34) as *mut u32);
        value &= !bits;
        write_volatile((iobase + 0x1100 + (chan * DWMAC_CHAN_REG_LEN) as usize + 0x34) as *mut u32, value as _);
    }
}
pub fn select_phy(id: usize,top_crm: usize) {
    let addr = top_crm;
    let new_addr = addr + 0x54 ;
    let mut reg_val = unsafe { read_volatile(new_addr as *mut u32) };
    reg_val |= 1 << id;
    unsafe {
        write_volatile(new_addr as *mut u32, reg_val as _);
    }
    let addr = top_crm +0x1008;
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
pub fn phylink_up<A: BstNicTraits>(iobase:usize)
{
    let ctrl:u32 = 0x8002003;
    unsafe{
        write_volatile(iobase as *mut u32, ctrl);
        /*delay 10 ms*/
        A::mdelay(10);
        let mut value = read_volatile(iobase as *mut u32);
        value |= GMAC_CONFIG_RE | GMAC_CONFIG_TE;
        write_volatile(iobase as *mut u32, value);
    }
}




pub fn test_link_status(iobase:usize)
{
	let mut status:u32 =0;
    unsafe{
        status= read_volatile((iobase + GMAC_PHYIF_CONTROL_STATUS as usize) as *mut u32);
    }
	/* Check the link status */
    let link_status = status & GMAC_PHYIF_CTRLSTATUS_LNKSTS;
    let mut speed = 0;
	if link_status != 0 {
		let speed_value:u32 = (status & GMAC_PHYIF_CTRLSTATUS_SPEED) >> GMAC_PHYIF_CTRLSTATUS_SPEED_SHIFT;
		if speed_value == GMAC_PHYIF_CTRLSTATUS_SPEED_125
        {
            speed = SPEED_1000;
        }
		else if speed_value == GMAC_PHYIF_CTRLSTATUS_SPEED_25 {
            speed = SPEED_100;
        }
		else{
            speed = SPEED_10;
        }
		let pcs_duplex = status & GMAC_PHYIF_CTRLSTATUS_LNKMOD_MASK;
        if pcs_duplex != 0 {
            info!("Link is Up - {}/Full",speed);
        }
        else{
            info!("Link is Up - {}/Half",speed);
        }
	} else {
		info!("Link is Down\n");
	}
}