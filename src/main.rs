#![no_std]
#![no_main]
#![allow(unused)]

use flash_algorithm::*;
use hpm_metapac as pac;
use hpm_metapac::SYSCTL;
use panic_halt as _;

use self::romapi::*;

#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
pub mod romapi;

pub const XPI0_MEM_START: u32 = 0x80000000;
pub const XPI1_MEM_START: u32 = 0x90000000;

pub const ROM_API_TABLE_ROOT: *const bootloader_api_table_t =
    0x2001FF00 as *const bootloader_api_table_t;

pub const FLASH_BASE: u32 = XPI0_MEM_START;
// pub const MEM_START: u32 = XPI1_MEM_START;

struct Algorithm {
    nor_config: xpi_nor_config_t,
    cfg_option: xpi_nor_config_option_t,
    xpi_base: *mut XPI_Type,
}

algorithm!(Algorithm, {
    flash_address: 0x80000000,
    flash_size: 0x100000, // 1M
    page_size: 256,
    empty_value: 0xFF,
    sectors: [{
        size: 4096,
        address: 0x0,
    }]
});

unsafe fn flash_auth_probe(
    base: *mut XPI_Type,
) -> Option<(xpi_nor_config_t, xpi_nor_config_option_t)> {
    // [15:12] Pin group
    // 0 - 1st group / 1 - 2nd group
    // [11:8] Connection selection
    // 0 - CA_CS0 / 1 - CB_CS0 / 2 - CA_CS0 + CB_CS0 (Two FLASH connected to CA and CB respectively)
    let opt1_select = [
        0x1000, // Pin group 1 & CA
        0x0000, // Pin group 0 & CA
        0x0100, // Pin group 0 & CB
    ];
    const XPI_USE_PORT_B_MASK: u32 = 0x100;

    let romapi_table = &*ROM_API_TABLE_ROOT;

    // OTP: word 7
    let otp7 = (&*romapi_table.otp_driver_if).read_from_shadow.unwrap()(7);
    let chip_has_sip_flash = otp7 & (1 << 24) != 0;

    let mut opt: xpi_nor_config_option_t = core::mem::zeroed();
    let mut cfg: xpi_nor_config_t = core::mem::zeroed();

    opt.header.U = 0xFCF90002; // 2 option bytes
    opt.option0.U = 0x05; // Frequency option, 5 = 100MHz

    for &opt1 in &opt1_select {
        opt.option1.U = opt1;

        if (&*romapi_table.xpi_nor_driver_if).auto_config.unwrap()(base, &mut cfg, &mut opt) == 0 {
            // config ok
            return Some((cfg, opt));
        } else {
            if chip_has_sip_flash {
                // do not continue to auto probe flash if the chip is sip part
                return None;
            }
        }
    }

    None
}

impl FlashAlgorithm for Algorithm {
    fn new(address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        // mcu clock and peripheral re-init
        SYSCTL.group0(0).value().modify(|w| w.set_link(0xFFFFFFFF));
        SYSCTL.group0(1).value().modify(|w| w.set_link(0xFFFFFFFF));
        // connect group0 to cpu0
        SYSCTL.affiliate(0).set().write(|w| w.set_link(1));

        let xpi_base = pac::XPI0.as_ptr() as *mut XPI_Type;

        unsafe {
            let mut nor_cfg: romapi::xpi_nor_config_t = core::mem::zeroed();
            let mut option: xpi_nor_config_option_t = core::mem::zeroed();
            /* HPM5300
            #define BOARD_APP_XPI_NOR_CFG_OPT_HDR  (0xfcf90002U)
            #define BOARD_APP_XPI_NOR_CFG_OPT_OPT0 (0x00000006U)
            #define BOARD_APP_XPI_NOR_CFG_OPT_OPT1 (0x00001000U)
            */
            option.header.U = 0xfcf90002;
            option.option0.U = 0x00000006;
            option.option1.U = 0x00001000;

            let xpi_nor_driver = &*(&*ROM_API_TABLE_ROOT).xpi_nor_driver_if;

            let ret = xpi_nor_driver.auto_config.unwrap()(xpi_base, &mut nor_cfg, &mut option);
            if ret != 0 {
                return Err(ErrorCode::new(ret as _).unwrap());
            }

            /*
            let addr = address - FLASH_BASE;
            let ret = xpi_nor_driver.enable_write.unwrap()(
                xpi_base,
                xpi_xfer_channel_auto,
                &nor_cfg,
                addr,
            );
            if ret != 0 {
                return Err(ErrorCode::new(ret as _).unwrap());
            }
            */

            nor_cfg.device_info.clk_freq_for_non_read_cmd = 0;

            Ok(Self {
                nor_config: nor_cfg,
                cfg_option: option,
                xpi_base,
            })
        }
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        unsafe {
            let xpi_nor_driver = &*(&*ROM_API_TABLE_ROOT).xpi_nor_driver_if;
            let ret = xpi_nor_driver.erase_chip.unwrap()(
                self.xpi_base,
                xpi_xfer_channel_auto,
                &self.nor_config,
            );
            if ret != 0 {
                return Err(ErrorCode::new(ret as _).unwrap());
            }
        }
        Ok(())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        unsafe {
            let xpi_nor_driver = &*(&*ROM_API_TABLE_ROOT).xpi_nor_driver_if;

            let addr = addr - FLASH_BASE;

            let ret = xpi_nor_driver.erase_sector.unwrap()(
                self.xpi_base,
                xpi_xfer_channel_auto,
                &mut self.nor_config,
                addr,
            );
            if ret != 0 {
                return Err(ErrorCode::new(ret as _).unwrap());
            }
        }
        Ok(())
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        unsafe {
            let xpi_nor_driver = &*(&*ROM_API_TABLE_ROOT).xpi_nor_driver_if;

            let addr = addr - FLASH_BASE;

            let ret = xpi_nor_driver.program.unwrap()(
                self.xpi_base,
                xpi_xfer_channel_auto,
                &mut self.nor_config,
                data.as_ptr() as *const _,
                addr,
                data.len() as u32,
            );
            if ret != 0 {
                return Err(ErrorCode::new(ret as _).unwrap());
            }
        }
        Ok(())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {}
}
