#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

#[doc = "  @brief XPI_Type definitions for\n  @note For compatibility"]
pub type XPI_Type = u32;

// Register block of XPI0
pub const HPM_XPI0: *mut XPI_Type = 0xF3000000 as *mut XPI_Type;

pub const ROM_API_TABLE_ROOT: *const bootloader_api_table_t =
    0x2001FF00 as *const bootloader_api_table_t;

pub type hpm_stat_t = u32;

#[doc = " Definitions\n/\n/**\n @brief OTP region definitions"]
pub type otp_region_t = ::core::ffi::c_uint;

pub const otp_lock_option_t_otp_no_lock: otp_lock_option_t = 0;
pub const otp_lock_option_t_otp_read_only: otp_lock_option_t = 1;
pub const otp_lock_option_t_otp_permanent_no_lock: otp_lock_option_t = 2;
pub const otp_lock_option_t_otp_disable_access: otp_lock_option_t = 3;
pub const otp_lock_option_t_otp_lock_option_max: otp_lock_option_t = 3;
#[doc = " @brief OTP lock options"]
pub type otp_lock_option_t = ::core::ffi::c_uint;

#[doc = "< Total size in bytes"]
pub const xpi_nor_property_total_size: u32 = 0;
#[doc = "< Page size in bytes"]
pub const xpi_nor_property_page_size: u32 = 1;
#[doc = "<sector size in bytes"]
pub const xpi_nor_property_sector_size: u32 = 2;
#[doc = "< block size in bytes"]
pub const xpi_nor_property_block_size: u32 = 3;

#[doc = " @brief XPI configuration structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_config_t {
    #[doc = "< Read sample clock source"]
    pub rxclk_src: u8,
    #[doc = "< Reserved"]
    pub reserved0: [u8; 7usize],
    #[doc = "< Tx watermark in double words"]
    pub tx_watermark_in_dwords: u8,
    #[doc = "< Rx watermark in double words"]
    pub rx_watermark_in_dwords: u8,
    #[doc = "< Enable differential clock"]
    pub enable_differential_clk: u8,
    #[doc = "< Reserved"]
    pub reserved1: [u8; 5usize],
    #[doc = "< Access flags"]
    pub access_flags: u32,
}

#[doc = " @brief XPI Device Configuration structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_device_config_t {
    #[doc = "< Device size in kbytes"]
    pub size_in_kbytes: u32,
    #[doc = "< XPI serial root clock frequency"]
    pub serial_root_clk_freq: u32,
    #[doc = "< Enable write mask, typically for PSRAM/HyperRAM"]
    pub enable_write_mask: u8,
    #[doc = "< Data valid time, Unit 0.1ns"]
    pub data_valid_time: u8,
    pub reserved0: [u8; 2usize],
    #[doc = "< CS hold time, cycles in terms of FLASH clock"]
    pub cs_hold_time: u8,
    #[doc = "< CS setup time, cycles in terms of FLASH clock"]
    pub cs_setup_time: u8,
    #[doc = "< CS interval, cycles in terms of FLASH clock"]
    pub cs_interval: u16,
    pub reserved1: u8,
    #[doc = "< Column address bits"]
    pub column_addr_size: u8,
    #[doc = "< Enable word address, for HyperFLASH/HyperRAM"]
    pub enable_word_address: u8,
    #[doc = "< Delay target"]
    pub dly_target: u8,
    #[doc = "< AHB write sequence index"]
    pub ahb_write_seq_idx: u8,
    #[doc = "< AHB write sequence number"]
    pub ahb_write_seq_num: u8,
    #[doc = "< AHB read sequence index"]
    pub ahb_read_seq_idx: u8,
    #[doc = "< AHB read sequence number"]
    pub ahb_read_seq_num: u8,
    #[doc = "< AHB write wait interval, in terms of FLASH clock"]
    pub ahb_write_wait_interval: u8,
    pub reserved2: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_ahb_buffer_cfg_t {
    pub entry: [xpi_ahb_buffer_cfg_t__bindgen_ty_1; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_ahb_buffer_cfg_t__bindgen_ty_1 {
    pub priority: u8,
    pub master_idx: u8,
    pub buf_size_in_dword: u8,
    pub enable_prefetch: bool,
}

#[doc = "< Port:  Channel A1"]
pub const xpi_channel_t_xpi_channel_a1: xpi_channel_t = 0;
#[doc = "< Port:  Channel A2"]
pub const xpi_channel_t_xpi_channel_a2: xpi_channel_t = 1;
#[doc = "< Port:  Channel B1"]
pub const xpi_channel_t_xpi_channel_b1: xpi_channel_t = 2;
#[doc = "< Port:  Channel B2"]
pub const xpi_channel_t_xpi_channel_b2: xpi_channel_t = 3;
#[doc = " @brief XPI Channel defitions"]
pub type xpi_channel_t = ::core::ffi::c_uint;

#[doc = "< The address is based on the device connected to Channel A1"]
pub const xpi_xfer_channel_t_xpi_xfer_channel_a1: xpi_xfer_channel_t = 0;
#[doc = "< The address is based on the device connected to Channel A2"]
pub const xpi_xfer_channel_t_xpi_xfer_channel_a2: xpi_xfer_channel_t = 1;
#[doc = "< The address is based on the device connected to Channel B1"]
pub const xpi_xfer_channel_t_xpi_xfer_channel_b1: xpi_xfer_channel_t = 2;
#[doc = "< The address is based on the device connected to Channel B2"]
pub const xpi_xfer_channel_t_xpi_xfer_channel_b2: xpi_xfer_channel_t = 3;
#[doc = "< The channel is auto determined"]
pub const xpi_xfer_channel_t_xpi_xfer_channel_auto: xpi_xfer_channel_t = 4;
#[doc = " @brief XPI Transfer Channel type definitions"]
pub type xpi_xfer_channel_t = ::core::ffi::c_uint;

#[doc = " @brief XPI Xfer context"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_xfer_ctx_t {
    #[doc = "< device address for XPI transfer"]
    pub addr: u32,
    #[doc = "< channel for XPI transfer"]
    pub channel: u8,
    #[doc = "< command type for XPI transfer"]
    pub cmd_type: u8,
    #[doc = "< Sequence index for XPI transfer"]
    pub seq_idx: u8,
    #[doc = "< Sequence number for XPI transfer"]
    pub seq_num: u8,
    #[doc = "< Buffer for XPI transfer"]
    pub buf: *mut u32,
    #[doc = "< Transfer size in bytes"]
    pub xfer_size: u32,
}

#[doc = " @brief XPI instruction sequence"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_instr_seq_t {
    pub entry: [u32; 4usize],
}

#[doc = " @brief XPI NOR configuration structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_nor_config_t {
    #[doc = "< Must be \"XNOR\", offset 0x000"]
    pub tag: u32,
    #[doc = "< Reserved for future use, offset 0x004"]
    pub reserved0: u32,
    #[doc = "< RXCLKSRC value, offset 0x008"]
    pub rxclk_src: u8,
    #[doc = "< Clock frequency, offset 0x009"]
    pub clk_freq: u8,
    #[doc = "< Drive strength, offset 0x0a"]
    pub drive_strength: u8,
    #[doc = "< Column address size, offset 0x0b"]
    pub column_addr_size: u8,
    #[doc = "< RXCLKSRC during FLASH initialization, offset 0x0c"]
    pub rxclk_src_for_init: u8,
    #[doc = "< Indicate whether device configuration is in progress, offset: 0x0d"]
    pub config_in_progress: u8,
    #[doc = "< Reserved for future use, offset 0x00f"]
    pub reserved: [u8; 2usize],
    #[doc = "< Device connection information"]
    pub chn_info: [xpi_nor_config_t__bindgen_ty_1; 4usize],
    #[doc = "< Device info, offset 0x20"]
    pub device_info: xpi_device_info_t,
    #[doc = "< Standard instruction sequence table, offset 0x70"]
    pub instr_set: [xpi_instr_seq_t; 9usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_nor_config_t__bindgen_ty_1 {
    #[doc = "<  Port enable flag, 0 - not enabled, 1 - enabled"]
    pub enable: u8,
    #[doc = "< 0 - 1st IO group, 1 - 2nd IO group"]
    pub group: u8,
    pub reserved: [u8; 2usize],
}

#[doc = " @brief Device Mode confiuration structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct device_mode_cfg_t {
    #[doc = "< Configuration command type"]
    pub cfg_cmd_type: u8,
    #[doc = "< Size for parameter"]
    pub param_size: u8,
}

#[doc = " @brief XPI NOR device information structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_device_info_t {
    #[doc = "< Device Size in Kilobytes, offset 0x00"]
    pub size_in_kbytes: u32,
    #[doc = "< Page size, offset 0x04"]
    pub page_size: u16,
    #[doc = "< Sector size in kilobytes, offset 0x06"]
    pub sector_size_kbytes: u16,
    #[doc = "< Block size in kilobytes, offset 0x08"]
    pub block_size_kbytes: u16,
    #[doc = "< Busy offset, offset 0x0a"]
    pub busy_offset: u8,
    #[doc = "< Busy polarity, offset 0x0b"]
    pub busy_polarity: u8,
    #[doc = "< Device Size in Kilobytes, offset 0x0c"]
    pub data_pads: u8,
    #[doc = "< Enable DDR mode, offset 0x0d"]
    pub en_ddr_mode: u8,
    #[doc = "< Clk frequency for device configuration offset 0x0e"]
    pub clk_freq_for_device_cfg: u8,
    #[doc = "< Working mode after POR reset offset 0x0f"]
    pub working_mode_por: u8,
    #[doc = "< The device working mode, offset 0x10"]
    pub working_mode: u8,
    #[doc = "< Enable Differential clock, offset 0x11"]
    pub en_diff_clk: u8,
    #[doc = "< Data valid time, in 0.1ns, offset 0x12"]
    pub data_valid_time: u8,
    #[doc = "< Enable half clock for non-read command, offset 0x13"]
    pub en_half_clk_for_non_read_cmd: u8,
    #[doc = "< Enable safe clock for non-read command, offset 0x14"]
    pub clk_freq_for_non_read_cmd: u8,
    #[doc = "< XPI DLL Delay Target, offset 0x15"]
    pub dll_dly_target: u8,
    #[doc = "< IO voltage, offset 0x16"]
    pub io_voltage: u8,
    #[doc = "< Reserved for future use, offset 0x17"]
    pub reserved0: u8,
    #[doc = "< CS hold time, 0 - default value, others - user specified value, offset 0x18"]
    pub cs_hold_time: u8,
    #[doc = "< CS setup time, 0 - default value, others - user specified value, offset 0x19"]
    pub cs_setup_time: u8,
    #[doc = "< CS interval, intervals between to CS active, offset 0x1a"]
    pub cs_interval: u8,
    #[doc = "< Enable device mode configuration, offset 0x1b"]
    pub en_dev_mode_cfg: u8,
    #[doc = "< Flash state context, offset 0x1c"]
    pub flash_state_ctx: u32,
    #[doc = "< Mode configuration sequences, offset 0x20"]
    pub mode_cfg_list: [device_mode_cfg_t; 2usize],
    #[doc = "< Mode configuration parameters, offset 0x24"]
    pub mode_cfg_param: [u32; 2usize],
    #[doc = "< Reserved for future use, offset 0x2C"]
    pub reserved1: u32,
    #[doc = "< Mode Configuration Instruction sequence, offset 0x30"]
    pub cfg_instr_seq: [xpi_device_info_t__bindgen_ty_1; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_device_info_t__bindgen_ty_1 {
    pub entry: [u32; 4usize],
}

// Core tables

#[doc = " @brief Bootloader API table"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bootloader_api_table_t {
    pub version: u32,
    pub copyright: *const ::core::ffi::c_char,
    pub run_bootloader: Option<unsafe extern "C" fn(arg: *mut ::core::ffi::c_void) -> hpm_stat_t>,
    pub otp_driver_if: *const otp_driver_interface_t,
    pub xpi_driver_if: *const xpi_driver_interface_t,
    pub xpi_nor_driver_if: *const xpi_nor_driver_interface_t,
    pub reserved0: u32,
    pub sdp_driver_if: *const sdp_driver_interface_t,
    pub reserved1: [u32; 3usize],
    pub exip_api_if: *const exip_driver_interface_t,
    pub family_id: u32,
}

#[doc = " @brief OTP driver interface"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct otp_driver_interface_t {
    pub version: u32,
    pub init: Option<unsafe extern "C" fn()>,
    pub deinit: Option<unsafe extern "C" fn()>,
    pub read_from_shadow: Option<unsafe extern "C" fn(addr: u32) -> u32>,
    pub read_from_ip: Option<unsafe extern "C" fn(addr: u32) -> u32>,
    pub program:
        Option<unsafe extern "C" fn(addr: u32, src: *const u32, num_of_words: u32) -> hpm_stat_t>,
    pub reload: Option<unsafe extern "C" fn(region: otp_region_t) -> hpm_stat_t>,
    pub lock: Option<unsafe extern "C" fn(addr: u32, lock_option: otp_lock_option_t) -> hpm_stat_t>,
    pub lock_shadow:
        Option<unsafe extern "C" fn(addr: u32, lock_option: otp_lock_option_t) -> hpm_stat_t>,
    pub set_configurable_region:
        Option<unsafe extern "C" fn(start: u32, num_of_words: u32) -> hpm_stat_t>,
    pub write_shadow_register: Option<unsafe extern "C" fn(addr: u32, data: u32) -> hpm_stat_t>,
}
#[doc = " @brief XPI driver interface"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_driver_interface_t {
    pub version: u32,
    pub get_default_config:
        Option<unsafe extern "C" fn(xpi_config: *mut xpi_config_t) -> hpm_stat_t>,
    pub get_default_device_config:
        Option<unsafe extern "C" fn(dev_config: *mut xpi_device_config_t) -> hpm_stat_t>,
    pub init: Option<
        unsafe extern "C" fn(base: *mut XPI_Type, xpi_config: *mut xpi_config_t) -> hpm_stat_t,
    >,
    pub config_ahb_buffer: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            ahb_buf_cfg: *mut xpi_ahb_buffer_cfg_t,
        ) -> hpm_stat_t,
    >,
    pub config_device: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            dev_cfg: *mut xpi_device_config_t,
            channel: xpi_channel_t,
        ) -> hpm_stat_t,
    >,
    pub update_instr_table: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            inst_base: *const u32,
            seq_idx: u32,
            num: u32,
        ) -> hpm_stat_t,
    >,
    pub transfer_blocking:
        Option<unsafe extern "C" fn(base: *mut XPI_Type, xfer: *mut xpi_xfer_ctx_t) -> hpm_stat_t>,
    pub software_reset: Option<unsafe extern "C" fn(base: *mut XPI_Type)>,
    pub is_idle: Option<unsafe extern "C" fn(base: *mut XPI_Type) -> bool>,
    pub update_dllcr: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            serial_root_clk_freq: u32,
            data_valid_time: u32,
            channel: xpi_channel_t,
            dly_target: u32,
        ),
    >,
    pub get_abs_apb_xfer_addr: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            in_addr: u32,
            out_addr: *mut u32,
        ) -> hpm_stat_t,
    >,
}

#[doc = " @brief XPI NOR configuration option\n        The ROM SW can detect the FLASH configuration based on the following structure specified by the end-user"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xpi_nor_config_option_t {
    pub header: u32,
    pub option0: u32,
    pub option1: u32,
    pub option2: u32,
}

#[doc = " @brief XPI NOR driver interface"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xpi_nor_driver_interface_t {
    pub version: u32,
    pub get_config: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            nor_cfg: *mut xpi_nor_config_t,
            cfg_option: *mut xpi_nor_config_option_t,
        ) -> hpm_stat_t,
    >,
    pub init: Option<
        unsafe extern "C" fn(base: *mut XPI_Type, nor_config: *mut xpi_nor_config_t) -> hpm_stat_t,
    >,
    pub enable_write: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            addr: u32,
        ) -> hpm_stat_t,
    >,
    pub get_status: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            addr: u32,
            out_status: *mut u16,
        ) -> hpm_stat_t,
    >,
    pub wait_busy: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            addr: u32,
        ) -> hpm_stat_t,
    >,
    pub erase: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            start: u32,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub erase_chip: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
        ) -> hpm_stat_t,
    >,
    pub erase_sector: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            addr: u32,
        ) -> hpm_stat_t,
    >,
    pub erase_block: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            addr: u32,
        ) -> hpm_stat_t,
    >,
    pub program: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            src: *const u32,
            dst_addr: u32,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub read: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            dst: *mut u32,
            start: u32,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub page_program_nonblocking: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            src: *const u32,
            dst_addr: u32,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub erase_sector_nonblocking: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            addr: u32,
        ) -> hpm_stat_t,
    >,
    pub erase_block_nonblocking: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
            addr: u32,
        ) -> hpm_stat_t,
    >,
    pub erase_chip_nonblocking: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            channel: xpi_xfer_channel_t,
            nor_config: *const xpi_nor_config_t,
        ) -> hpm_stat_t,
    >,
    pub reserved0: [u32; 3usize],
    pub auto_config: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            nor_cfg: *mut xpi_nor_config_t,
            cfg_option: *mut xpi_nor_config_option_t,
        ) -> hpm_stat_t,
    >,
    pub get_property: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            nor_cfg: *mut xpi_nor_config_t,
            property_id: u32,
            value: *mut u32,
        ) -> hpm_stat_t,
    >,
}

//pub type sdp_aes_ctx_t = c_void;
#[doc = " @brief SDP DMA context"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdp_dma_ctx_t {
    #[doc = "< SDP packet for DMA operation (memset/memcpy)"]
    pub sdp_pkt: sdp_pkt_struct_t,
}
#[doc = " @brief SDP AES context structure"]
pub type sdp_aes_ctx_t = sdp_crypto_ctx_t;
#[doc = " @brief SDP AES context structure"]
pub type sdp_sm4_ctx_t = sdp_crypto_ctx_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub union _sdp_packet_struct__bindgen_ty_1 {
    pub __bindgen_anon_1: _sdp_packet_struct__bindgen_ty_1__bindgen_ty_1,
    #[doc = "< Packet control word"]
    pub PKT_CTRL: u32,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct _sdp_packet_struct__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: [u8; 4usize],
}
//pub type sdp_sm4_ctx_t = c_void;
#[doc = " @brief SDP packet data structure"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _sdp_packet_struct {
    pub next_cmd: *mut _sdp_packet_struct,
    pub pkt_ctrl: _sdp_packet_struct__bindgen_ty_1,
    #[doc = "< Source address"]
    pub src_addr: u32,
    #[doc = "< Destination address"]
    pub dst_addr: u32,
    #[doc = "< Data buffer size in bytes"]
    pub buf_size: u32,
    pub reserved: [u32; 3usize],
}
#[doc = " @brief SDP packet data structure"]
pub type sdp_pkt_struct_t = _sdp_packet_struct;
#[doc = " @brief SDP AES context structure"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdp_crypto_ctx_t {
    #[doc = "< AES key index"]
    pub key_idx: u8,
    #[doc = "< AES key bits"]
    pub key_bits: u8,
    pub crypto_algo: u16,
    #[doc = "< SDP packet for AES operation"]
    pub sdp_pkt: sdp_pkt_struct_t,
    #[doc = "< buf0"]
    pub buf0: [u32; 4usize],
    #[doc = "< buf1"]
    pub buf1: [u32; 4usize],
    #[doc = "< buf2"]
    pub buf2: [u32; 4usize],
    #[doc = "< buf3"]
    pub buf3: [u32; 4usize],
}
#[doc = " @brief SDP HASH context"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdp_hash_ctx_t {
    #[doc = "< SDP packet for HASH operation"]
    pub sdp_pkt: sdp_pkt_struct_t,
    #[doc = "< internal buffer"]
    pub internal: [u32; 64usize],
}

#[doc = "< 128 bit AES key"]
pub const sdp_crypto_key_bits_t_sdp_aes_keybits_128: sdp_crypto_key_bits_t = 0;
#[doc = "< 256 bit AES key"]
pub const sdp_crypto_key_bits_t_sdp_aes_keybits_256: sdp_crypto_key_bits_t = 1;
pub const sdp_crypto_key_bits_t_sdp_sm4_keybits_128: sdp_crypto_key_bits_t = 0;

#[doc = " Definitions\n/\n/**\n @brief SDP AES key bit options"]
pub type sdp_crypto_key_bits_t = ::core::ffi::c_uint;
#[doc = " Definitions\n/\n/**\n @brief SDP AES key bit options"]
pub use self::sdp_crypto_key_bits_t as sdp_aes_key_bits_t;
#[doc = " Definitions\n/\n/**\n @brief SDP AES key bit options"]
pub use self::sdp_crypto_key_bits_t as sdp_sm4_key_bits_t;
#[doc = " @brief Crypto operation option"]
pub type sdp_crypto_op_t = ::core::ffi::c_uint;
#[doc = " @brief Crypto operation option"]
pub use self::sdp_crypto_op_t as sdp_aes_op_t;
#[doc = " @brief Crypto operation option"]
pub use self::sdp_crypto_op_t as sdp_sm4_op_t;

#[doc = "< SDP SHA1"]
pub const sdp_hash_alg_t_sdp_hash_alg_sha1: sdp_hash_alg_t = 0;
#[doc = "< SDP CRC32"]
pub const sdp_hash_alg_t_sdp_hash_alg_crc32: sdp_hash_alg_t = 1;
#[doc = "< SDP SHA256"]
pub const sdp_hash_alg_t_sdp_hash_alg_sha256: sdp_hash_alg_t = 2;
#[doc = "< SDP SM3"]
pub const sdp_hash_alg_t_sdp_hash_alg_sm3: sdp_hash_alg_t = 8;
pub const sdp_hash_alg_t_sdp_hash_alg_max: sdp_hash_alg_t = 8;
#[doc = " @brief SDP HASH algorithm definitions"]
pub type sdp_hash_alg_t = ::core::ffi::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct exip_region_context_t {
    pub region_start: u32,
    pub region_end: u32,
    pub aes_key: [u8; 16usize],
    pub nonce: [u8; 8usize],
    pub index: u8,
    pub enable: bool,
    pub valid: bool,
    pub lock: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct exip_region_param_t {
    #[doc = "< Start address, must be 4KB aligned"]
    pub start: u32,
    #[doc = "< Must be 4KB aligned"]
    pub len: u32,
    #[doc = "< AES Key"]
    pub key: [u8; 16usize],
    #[doc = "< Initial Vector/Counter"]
    pub ctr: [u8; 8usize],
}

#[doc = " @brief SDP API interface"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sdp_driver_interface_t {
    pub version: u32,
    pub sdp_ip_init: Option<unsafe extern "C" fn() -> hpm_stat_t>,
    pub sdp_ip_deinit: Option<unsafe extern "C" fn() -> hpm_stat_t>,
    pub aes_set_key: Option<
        unsafe extern "C" fn(
            aes_ctx: *mut sdp_aes_ctx_t,
            key: *const u8,
            keybits: sdp_aes_key_bits_t,
            key_idx: u32,
        ) -> hpm_stat_t,
    >,
    pub aes_crypt_ecb: Option<
        unsafe extern "C" fn(
            aes_ctx: *mut sdp_aes_ctx_t,
            op: sdp_aes_op_t,
            len: u32,
            in_: *const u8,
            out: *mut u8,
        ) -> hpm_stat_t,
    >,
    pub aes_crypt_cbc: Option<
        unsafe extern "C" fn(
            aes_ctx: *mut sdp_aes_ctx_t,
            op: sdp_aes_op_t,
            length: u32,
            iv: *mut u8,
            input: *const u8,
            output: *mut u8,
        ) -> hpm_stat_t,
    >,
    pub aes_crypt_ctr: Option<
        unsafe extern "C" fn(
            aes_ctx: *mut sdp_aes_ctx_t,
            nonce_ctr: *mut u8,
            input: *mut u8,
            output: *mut u8,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub aes_ccm_gen_enc: Option<
        unsafe extern "C" fn(
            aes_ctx: *mut sdp_aes_ctx_t,
            input_len: u32,
            nonce: *const u8,
            nonce_len: u32,
            aad: *const u8,
            aad_len: u32,
            input: *const u8,
            output: *mut u8,
            tag: *mut u8,
            tag_len: u32,
        ) -> hpm_stat_t,
    >,
    pub aes_ccm_dec_verify: Option<
        unsafe extern "C" fn(
            aes_ctx: *mut sdp_aes_ctx_t,
            input_len: u32,
            nonce: *const u8,
            nonce_len: u32,
            aad: *const u8,
            aad_len: u32,
            input: *const u8,
            output: *mut u8,
            tag: *const u8,
            tag_len: u32,
        ) -> hpm_stat_t,
    >,
    pub memcpy: Option<
        unsafe extern "C" fn(
            dma_ctx: *mut sdp_dma_ctx_t,
            dst: *mut ::core::ffi::c_void,
            src: *const ::core::ffi::c_void,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub memset: Option<
        unsafe extern "C" fn(
            dma_ctx: *mut sdp_dma_ctx_t,
            dst: *mut ::core::ffi::c_void,
            pattern: u8,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub hash_init: Option<
        unsafe extern "C" fn(hash_ctx: *mut sdp_hash_ctx_t, alg: sdp_hash_alg_t) -> hpm_stat_t,
    >,
    pub hash_update: Option<
        unsafe extern "C" fn(
            hash_ctx: *mut sdp_hash_ctx_t,
            data: *const u8,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub hash_finish:
        Option<unsafe extern "C" fn(hash_ctx: *mut sdp_hash_ctx_t, digest: *mut u8) -> hpm_stat_t>,
    pub sm4_set_key: Option<
        unsafe extern "C" fn(
            sm4_ctx: *mut sdp_sm4_ctx_t,
            key: *const u8,
            keybits: sdp_sm4_key_bits_t,
            key_idx: u32,
        ) -> hpm_stat_t,
    >,
    pub sm4_crypt_ecb: Option<
        unsafe extern "C" fn(
            sm4_ctx: *mut sdp_sm4_ctx_t,
            op: sdp_sm4_op_t,
            len: u32,
            in_: *const u8,
            out: *mut u8,
        ) -> hpm_stat_t,
    >,
    pub sm4_crypt_cbc: Option<
        unsafe extern "C" fn(
            sm4_ctx: *mut sdp_sm4_ctx_t,
            op: sdp_sm4_op_t,
            length: u32,
            iv: *mut u8,
            input: *const u8,
            output: *mut u8,
        ) -> hpm_stat_t,
    >,
    pub sm4_crypt_ctr: Option<
        unsafe extern "C" fn(
            sm4_ctx: *mut sdp_sm4_ctx_t,
            nonce_ctr: *mut u8,
            input: *mut u8,
            output: *mut u8,
            length: u32,
        ) -> hpm_stat_t,
    >,
    pub sm4_ccm_gen_enc: Option<
        unsafe extern "C" fn(
            sm4_ctx: *mut sdp_sm4_ctx_t,
            input_len: u32,
            nonce: *const u8,
            nonce_len: u32,
            aad: *const u8,
            aad_len: u32,
            input: *const u8,
            output: *mut u8,
            tag: *mut u8,
            tag_len: u32,
        ) -> hpm_stat_t,
    >,
    pub sm4_ccm_dec_verify: Option<
        unsafe extern "C" fn(
            sm4_ctx: *mut sdp_sm4_ctx_t,
            input_len: u32,
            nonce: *const u8,
            nonce_len: u32,
            aad: *const u8,
            aad_len: u32,
            input: *const u8,
            output: *mut u8,
            tag: *const u8,
            tag_len: u32,
        ) -> hpm_stat_t,
    >,
}

#[doc = " @brief EXIP driver interface"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct exip_driver_interface_t {
    pub version: u32,
    pub enable: Option<unsafe extern "C" fn(base: *mut XPI_Type) -> hpm_stat_t>,
    pub disable: Option<unsafe extern "C" fn(base: *mut XPI_Type) -> hpm_stat_t>,
    pub lock_reg_access: Option<unsafe extern "C" fn(base: *mut XPI_Type) -> hpm_stat_t>,
    pub configure_region: Option<
        unsafe extern "C" fn(base: *mut XPI_Type, ctx: *const exip_region_context_t) -> hpm_stat_t,
    >,
    pub remap_config: Option<
        unsafe extern "C" fn(base: *mut XPI_Type, start: u32, len: u32, offset: u32) -> bool,
    >,
    pub remap_enabled: Option<unsafe extern "C" fn(base: *mut XPI_Type) -> bool>,
    pub remap_disable: Option<unsafe extern "C" fn(base: *mut XPI_Type)>,
    pub exip_region_config: Option<
        unsafe extern "C" fn(
            base: *mut XPI_Type,
            index: u32,
            param: *mut exip_region_param_t,
        ) -> bool,
    >,
    pub exip_region_disable: Option<unsafe extern "C" fn(base: *mut XPI_Type, index: u32)>,
}
