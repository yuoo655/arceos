pub const DMA_BUS_MODE: usize = 0x00001000; /* Bus Mode */
pub const DMA_XMT_POLL_DEMAND: usize = 0x00001004; /* Transmit Poll Demand */
pub const DMA_RCV_POLL_DEMAND: usize = 0x00001008; /* Received Poll Demand */
pub const DMA_RCV_BASE_ADDR: usize = 0x0000100c; /* Receive List Base */
pub const DMA_TX_BASE_ADDR: usize = 0x00001010; /* Transmit List Base */
pub const DMA_STATUS: usize = 0x00001014; /* Status Register */
pub const DMA_CONTROL: usize = 0x00001018; /* Ctrl (Operational Mode) */
pub const DMA_INTR_ENA: usize = 0x0000101c; /* Interrupt Enable */
pub const DMA_MISSED_FRAME_CTR: usize = 0x00001020; /* Missed Frame Counter */

pub const DMA_BUS_MODE_SFT_RESET: usize = 0x00000001; /* Software Reset */

pub const DMA_CHAN_TX_BASE_ADDR_HI: usize = 0x00001110;
pub const DMA_CHAN_TX_BASE_ADDR: usize = 0x00001114;
pub const DMA_CHAN_TX_RING_LEN: usize = 0x0000112C;
pub const DMA_CHAN_TX_END_ADDR: usize = 0x1120;

pub const DMA_CHAN_RX_BASE_ADDR_HI: usize = 0x00001118;
pub const DMA_CHAN_RX_BASE_ADDR: usize = 0x0000111C;
pub const DMA_CHAN_RX_RING_LEN: usize = 0x00001130;
pub const DMA_CHAN_RX_END_ADDR: usize = 0x00001128;

pub const ETH_QUEUE_LENGTH: usize = 512;

pub const EMAC_DES3_OWN: u32 = 0x80000000;
pub const ETH_BUF_SIZE: u32 = 1536;
pub const EMAC_DES3_FD: u32 = 0x20000000;
pub const EMAC_DES3_LD: u32 = 0x10000000;
pub const EMAC_DES3_CSUM: u32 = 3 << 16;
pub const EMAC_TDES3_PL: u32 = 0x00007FFF;

pub const EMAC_TDES2_B1L: u32 = 0x00003FFF;
pub const EMAC_DES3_CTXT: u32 = 0x40000000;
pub const EMAC_DES3_ES: u32 = 0x00008000;

pub const EMAC_TDES2_IOC: u32 = 0x80000000;
pub const EMAC_RDES3_IOC: u32 = 0x40000000;
pub const EMAC_RDES3_PL: u32 = 0x00007FFF;
pub const EMAC_RDES3_BUF1V: u32 = 0x01000000;

pub const IRQ_SBD: u32 = 159;
pub const IRQ_TX_CH0: u32 = 162;
pub const IRQ_RX_CH0: u32 = 166;

pub const GMAC_CONFIG_RE: u32 = 0x01;
pub const GMAC_CONFIG_TE: u32 = 0x10;

pub const RDES3_INT_ON_COMPLETION_EN: u32 = 1 << 30;

pub const TDES2_INTERRUPT_ON_COMPLETION: u32 = 1 << 31;
pub const DWMAC_CHAN_REG_LEN: u32 = 0x80;
pub const DMA_CHAN_STATUS_TBU: u32 = 1 << 2; //bit 2
pub const DMA_CHAN_STATUS_AIS: u32 = 1 << 14; //bit 14
pub const DMA_CHAN_INTR_ENA_TIE: u32 = 1; //bit 0
pub const DMA_CHAN_STATUS_NIS: u32 = 1 << 15; //bit 15
pub const DMA_CHAN_STATUS_RBU: u32 = 1 << 7; // bit 7
pub const DMA_CHAN_INTR_ENA_RIE: u32 = 1 << 6; // bit 6

pub const TX_HARD_ERROR: u32 = 0x1;
pub const TX_HARD_ERROR_BUMP_TC: u32 = 0x2;
pub const HANDLE_RX: u32 = 0x4;
pub const HANDLE_TX: u32 = 0x8;

pub const DMA_CHAN_STATUS_RI: u32 = 1 << 6; // bit 6
pub const DMA_CHAN_STATUS_TI: u32 = 1 << 0; // bit 0
pub const RDES3_OWN: u32 = 1 << 31; // bit 0
pub const RDES3_BUFFER1_VALID_ADDR: u32 = 1 << 24; // bit 0
pub const RX_RING_LEN: u32 = 512; // bit 0




pub const NUM_RX_QUEUE: usize = 1;
pub const NUM_TX_QUEUE: usize = 1;


pub const TDES3_OWN:u32 = 1 << 31;


pub const TDES2_BUFFER1_SIZE_MASK:u32 = 0b11_1111_1111_1111 as u32;  //0~13
pub const TDES3_PACKET_SIZE_MASK:u32 = 0b111_1111_1111_1111 as u32;  //0~14

pub const TDES3_FIRST_DESCRIPTOR:u32 = 1 << 29;
pub const TDES3_LAST_DESCRIPTOR:u32 = 1 << 28;