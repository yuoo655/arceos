//use kernel::str::CStr;
//use kernel::c_str;

pub (crate) const DWMAC_CHAN_NUM: usize = 0x00000004;

/* DMA CRS Control and Status Register Mapping */
pub (crate) const DMA_BUS_MODE: usize = 0x00001000;  /* Bus Mode */
pub (crate) const DMA_XMT_POLL_DEMAND: usize = 0x00001004;  /* Transmit Poll Demand */
pub (crate) const DMA_RCV_POLL_DEMAND: usize = 0x00001008;  /* Received Poll Demand */
pub (crate) const DMA_RCV_BASE_ADDR: usize = 0x0000100c;  /* Receive List Base */
pub (crate) const DMA_TX_BASE_ADDR: usize = 0x00001010;  /* Transmit List Base */
pub (crate) const DMA_STATUS: usize = 0x00001014;  /* Status Register */
pub (crate) const DMA_CONTROL: usize = 0x00001018;  /* Ctrl (Operational Mode) */
pub (crate) const DMA_INTR_ENA: usize = 0x0000101c;  /* Interrupt Enable */
pub (crate) const DMA_MISSED_FRAME_CTR: usize = 0x00001020;  /* Missed Frame Counter */


pub (crate) const DMA_BUS_MODE_SFT_RESET:u32 = 0x00000001;	/* Software Reset */

pub (crate) const DMA_CHAN_TX_BASE_ADDR_HI:usize = 0x00001110;
pub (crate) const DMA_CHAN_TX_BASE_ADDR:usize = 0x00001114;
pub (crate) const DMA_CHAN_TX_RING_LEN:usize = 0x0000112C;
pub (crate) const DMA_CHAN_TX_END_ADDR:usize = 0x00001120;


pub (crate) const DMA_CHAN_RX_BASE_ADDR_HI:usize = 0x00001118;
pub (crate) const DMA_CHAN_RX_BASE_ADDR:usize = 0x0000111C;
pub (crate) const DMA_CHAN_RX_RING_LEN:usize = 0x00001130;
pub (crate) const DMA_CHAN_RX_END_ADDR:usize = 0x00001128;


pub (crate) const ETH_QUEUE_LENGTH: usize = 512;

pub (crate) const EMAC_DES3_OWN: u32 = 0x80000000;
pub (crate) const ETH_BUF_SIZE: u32 = 1536;
pub (crate) const EMAC_DES3_FD: u32 = 0x20000000;
pub (crate) const EMAC_DES3_LD: u32 = 0x10000000;
pub (crate) const EMAC_DES3_CSUM: u32 = 3 << 16;
pub (crate) const EMAC_TDES3_PL: u32 = 0x00007FFF;

pub (crate) const EMAC_TDES2_B1L: u32 = 0x00003FFF;
pub (crate) const EMAC_DES3_CTXT: u32 = 0x40000000;
pub (crate) const EMAC_DES3_ES: u32 = 0x00008000;

pub (crate) const EMAC_TDES2_IOC: u32 = 0x80000000;
pub (crate) const EMAC_RDES3_IOC: u32 = 0x40000000;
pub (crate) const EMAC_RDES3_PL: u32 = 0x00007FFF;
pub (crate) const EMAC_RDES3_BUF1V: u32 = 0x01000000;

pub (crate) const IRQ_SBD:u32 = 159;
pub (crate) const IRQ_TX_CH0:u32 = 162;
pub (crate) const IRQ_RX_CH0:u32 = 166;

pub (crate) const GMAC_CONFIG_RE:u32 = 0x01;
pub (crate) const GMAC_CONFIG_TE:u32 = 0x10;

//pub (crate) const BSTGMAC_RESOURCE_NAME: &CStr = c_str!("bstgmaceth");

pub (crate) const RDES3_INT_ON_COMPLETION_EN: u32 = 1 << 30;


pub (crate) const TDES2_INTERRUPT_ON_COMPLETION: u32 = 1<< 31;
pub (crate) const DWMAC_CHAN_REG_LEN: u32 = 0x80;
pub (crate) const DMA_CHAN_STATUS_TBU: u32 = 1 << 2; //bit 2
pub (crate) const DMA_CHAN_STATUS_AIS: u32 = 1 << 14; //bit 14
pub (crate) const DMA_CHAN_INTR_ENA_TIE: u32 = 1; //bit 0
pub (crate) const DMA_CHAN_STATUS_NIS: u32 = 1 << 15; //bit 15
pub (crate) const DMA_CHAN_STATUS_RBU: u32 = 1 << 7; // bit 7
pub (crate) const DMA_CHAN_INTR_ENA_RIE: u32 = 1 << 6; // bit 6


pub (crate) const TX_HARD_ERROR: u32 = 0x1;
pub (crate) const TX_HARD_ERROR_BUMP_TC: u32 = 0x2;
pub (crate) const HANDLE_RX: u32 = 0x4;
pub (crate) const HANDLE_TX: u32 = 0x8;


pub (crate) const DMA_CHAN_STATUS_RI: u32 = 1 << 6; // bit 6
pub (crate) const DMA_CHAN_STATUS_TI: u32 = 1 << 0; // bit 0
pub (crate) const RDES3_OWN: u32 = 1 << 31; // bit 0
pub (crate) const RDES3_BUFFER1_VALID_ADDR: u32 = 1 << 24; // bit 0
pub (crate) const RX_RING_LEN: u32 = 512; // bit 0

pub (crate) const GMAC_PHYIF_CONTROL_STATUS : u32 = 0x000000f8;
pub (crate) const GMAC_PHYIF_CTRLSTATUS_LNKSTS : u32 = 		1 << 19 ;
pub (crate) const GMAC_PHYIF_CTRLSTATUS_SPEED : u32	 =	60000;

pub (crate) const GMAC_PHYIF_CTRLSTATUS_SPEED_SHIFT :u32 = 17;
pub (crate) const GMAC_PHYIF_CTRLSTATUS_SPEED_125 : u32 = 0x2;
pub (crate) const GMAC_PHYIF_CTRLSTATUS_SPEED_25 :u32	=	0x1;
pub (crate) const GMAC_PHYIF_CTRLSTATUS_SPEED_2_5:u32 	=	0x0;

pub (crate) const GMAC_PHYIF_CTRLSTATUS_LNKMOD_MASK:u32 =	0x1;
pub (crate) const SPEED_10:u32	=	10;
pub (crate) const SPEED_100:u32	=	100;
pub (crate) const SPEED_1000:u32	=	1000;