#![no_std]
#![no_main]

use panic_halt as _;

use flash_algorithm::*;
use hpm_metapac::SYSCTL;

//use defmt_rtt as _;

pub mod romapi;

struct Algorithm {
    nor_cfg: romapi::xpi_nor_config_t,
}

algorithm!(Algorithm, {
    flash_address: 0x80000000,
    flash_size: 0x100000, // 1M
    page_size: 256,
    empty_value: 0xFF,
    sectors: [{
        size: 4096,
        address: 0x80000000,
    }]
});

const SECTOR_SIZE: u32 = 4096;
const FLASH_START_ADDR: u32 = 0x80000000;

impl FlashAlgorithm for Algorithm {
    fn new(address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        // board init
        SYSCTL.group0(0).value().modify(|w| w.set_link(0xFFFFFFFF));
        SYSCTL
            .group0(1)
            .value()
            .modify(|w: &mut hpm_metapac::sysctl::regs::Group0Value| w.set_link(0xFFFFFFFF));

        // connect group0 to cpu0
        SYSCTL.affiliate(0).set().write(|w| w.set_link(1));

        unsafe {
            use romapi::*;

            let mut nor_cfg: romapi::xpi_nor_config_t = core::mem::zeroed();
            let mut option: xpi_nor_config_option_t = core::mem::zeroed();
            /* HPM5300
            #define BOARD_APP_XPI_NOR_CFG_OPT_HDR  (0xfcf90002U)
            #define BOARD_APP_XPI_NOR_CFG_OPT_OPT0 (0x00000006U)
            #define BOARD_APP_XPI_NOR_CFG_OPT_OPT1 (0x00001000U)
            */
            option.header = 0xfcf90002;
            option.option0 = 0x00000006;
            option.option1 = 0x00001000;

            let xpi_nor_driver = &*(&*ROM_API_TABLE_ROOT).xpi_nor_driver_if;

            let ret = xpi_nor_driver.auto_config.unwrap()(HPM_XPI0, &mut nor_cfg, &mut option);

            // assert_eq!(ret, 0);

            let addr = address - FLASH_START_ADDR;

            let ret = xpi_nor_driver.enable_write.unwrap()(
                HPM_XPI0,
                xpi_xfer_channel_t_xpi_xfer_channel_auto,
                &nor_cfg,
                addr,
            );

            Ok(Self { nor_cfg })
        }
    }

    /*
    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        unsafe {
            use romapi::*;

            let xpi_nor_driver = &*(&*ROM_API_TABLE_ROOT).xpi_nor_driver_if;

            let ret = xpi_nor_driver.erase_chip.unwrap()(
                HPM_XPI0,
                xpi_xfer_channel_t_xpi_xfer_channel_auto,
                &self.nor_cfg,
            );

            //        assert_eq!(ret, 0);
        }
        Ok(())
    }
    */

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        unsafe {
            use romapi::*;

            let xpi_nor_driver = &*(&*ROM_API_TABLE_ROOT).xpi_nor_driver_if;

            let addr = addr - FLASH_START_ADDR;

            let ret = xpi_nor_driver.erase.unwrap()(
                HPM_XPI0,
                xpi_xfer_channel_t_xpi_xfer_channel_auto,
                &mut self.nor_cfg,
                addr,
                SECTOR_SIZE,
            );

            // assert_eq!(ret, 0);
        }
        Ok(())
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        unsafe {
            use romapi::*;

            let xpi_nor_driver = &*(&*ROM_API_TABLE_ROOT).xpi_nor_driver_if;

            let addr = addr - FLASH_START_ADDR;

            let ret = xpi_nor_driver.program.unwrap()(
                HPM_XPI0,
                xpi_xfer_channel_t_xpi_xfer_channel_auto,
                &mut self.nor_cfg,
                data.as_ptr() as *const _,
                addr,
                data.len() as u32,
            );

            //  assert_eq!(ret, 0);
        }
        Ok(())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {}
}
