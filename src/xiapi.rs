#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
extern crate libc;

pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type size_t = usize; //unsure
pub type wchar_t = libc::c_char;

// xiApi parameters

// Device info parameters
pub static XI_PRM_DEVICE_NAME: &'static str = "device_name";             // Return device name
pub static XI_PRM_DEVICE_TYPE: &'static str = "device_type";             // Return device type
pub static XI_PRM_DEVICE_MODEL_ID: &'static str = "device_model_id";         // Return device model id
pub static XI_PRM_DEVICE_SN: &'static str = "device_sn";               // Return device serial number
pub static XI_PRM_DEVICE_SENS_SN: &'static str = "device_sens_sn";          // Return sensor serial number
pub static XI_PRM_DEVICE_INSTANCE_PATH: &'static str = "device_inst_path";        // Return device system instance path.
pub static XI_PRM_DEVICE_USER_ID: &'static str = "device_user_id";          // Return custom ID of camera.
// Device acquisition settings
pub static XI_PRM_EXPOSURE: &'static str = "exposure";                // Exposure time in microseconds
pub static XI_PRM_GAIN_SELECTOR: &'static str = "gain_selector";           // Gain selector for parameter Gain allows to select different type of gains. XI_GAIN_SELECTOR_TYPE
pub static XI_PRM_GAIN: &'static str = "gain";                    // Gain in dB
pub static XI_PRM_DOWNSAMPLING: &'static str = "downsampling";            // Change image resolution by binning or skipping.
pub static XI_PRM_DOWNSAMPLING_TYPE: &'static str = "downsampling_type";       // Change image downsampling type. XI_DOWNSAMPLING_TYPE
pub static XI_PRM_SHUTTER_TYPE: &'static str = "shutter_type";            // Change sensor shutter type(CMOS sensor). XI_SHUTTER_TYPE
pub static XI_PRM_IMAGE_DATA_FORMAT: &'static str = "imgdataformat";           // Output data format. XI_IMG_FORMAT
pub static XI_PRM_IMAGE_PAYLOAD_SIZE: &'static str = "imgpayloadsize";          // Buffer size in bytes sufficient for output image returned by xiGetImage
pub static XI_PRM_TRANSPORT_PIXEL_FORMAT: &'static str = "transport_pixel_format";  // Current format of pixels on transport layer. XI_GenTL_Image_Format_e
pub static XI_PRM_SENSOR_TAPS: &'static str = "sensor_taps";             // Number of taps
pub static XI_PRM_SENSOR_CLOCK_FREQ_HZ: &'static str = "sensor_clock_freq_hz";    // Sensor clock frequency in Hz.
pub static XI_PRM_SENSOR_CLOCK_FREQ_INDEX: &'static str = "sensor_clock_freq_index"; // Sensor clock frequency index. Sensor with selected frequencies have possibility to set the frequency only by this index.
pub static XI_PRM_SENSOR_OUTPUT_CHANNEL_COUNT: &'static str = "sensor_output_channel_count";// Number of output channels from sensor used for data transfer.
pub static XI_PRM_SENSOR_DATA_BIT_DEPTH: &'static str = "sensor_bit_depth";        // Sensor output data bit depth.
pub static XI_PRM_OUTPUT_DATA_BIT_DEPTH: &'static str = "output_bit_depth";        // Device output data bit depth.
pub static XI_PRM_IMAGE_DATA_BIT_DEPTH: &'static str = "image_data_bit_depth";    // bitdepth of data returned by function xiGetImage
pub static XI_PRM_OUTPUT_DATA_PACKING: &'static str = "output_bit_packing";      // Device output data packing (or grouping) enabled. Packing could be enabled if output_data_bit_depth > 8 and packing capability is available. XI_SWITCH
pub static XI_PRM_OUTPUT_DATA_PACKING_TYPE: &'static str = "output_bit_packing_type"; // . XI_OUTPUT_DATA_PACKING_TYPE
pub static XI_PRM_FRAMERATE: &'static str = "framerate";               // Define framerate in Hz
pub static XI_PRM_COUNTER_SELECTOR: &'static str = "counter_selector";        // Select counter XI_COUNTER_SELECTOR
pub static XI_PRM_COUNTER_VALUE: &'static str = "counter_value";           // Counter status
pub static XI_PRM_ACQ_TIMING_MODE: &'static str = "acq_timing_mode";         // Type of sensor frames timing. XI_ACQ_TIMING_MODE
pub static XI_PRM_AVAILABLE_BANDWIDTH: &'static str = "available_bandwidth";     // Calculate and return available interface bandwidth(int Megabits)
pub static XI_PRM_LIMIT_BANDWIDTH: &'static str = "limit_bandwidth";         // Set/get bandwidth(datarate)(in Megabits)
pub static XI_PRM_BUFFER_POLICY: &'static str = "buffer_policy";           // Data move policy XI_BP
pub static XI_PRM_WIDTH: &'static str = "width";                   // Width of the Image provided by the device (in pixels).
pub static XI_PRM_HEIGHT: &'static str = "height";                  // Height of the Image provided by the device (in pixels).
pub static XI_PRM_OFFSET_X: &'static str = "offsetX";                 // Horizontal offset from the origin to the area of interest (in pixels).
pub static XI_PRM_OFFSET_Y: &'static str = "offsetY";                 // Vertical offset from the origin to the area of interest (in pixels).
pub static XI_PRM_LUT_EN: &'static str = "LUTEnable";               // Activates LUT. XI_SWITCH
pub static XI_PRM_LUT_INDEX: &'static str = "LUTIndex";                // Control the index (offset) of the coefficient to access in the LUT.
pub static XI_PRM_LUT_VALUE: &'static str = "LUTValue";                // Value at entry LUTIndex of the LUT
pub static XI_PRM_TRG_SOURCE: &'static str = "trigger_source";          // Defines source of trigger. XI_TRG_SOURCE
pub static XI_PRM_TRG_SELECTOR: &'static str = "trigger_selector";        // Selects the type of trigger. XI_TRG_SELECTOR
pub static XI_PRM_TRG_SOFTWARE: &'static str = "trigger_software";        // Generates an internal trigger. XI_PRM_TRG_SOURCE must be set to TRG_SOFTWARE.
pub static XI_PRM_TRG_DELAY: &'static str = "trigger_delay";           // Specifies the delay in microseconds (us) to apply after the trigger reception before activating it.
pub static XI_PRM_GPI_SELECTOR: &'static str = "gpi_selector";            // Selects GPI
pub static XI_PRM_GPI_MODE: &'static str = "gpi_mode";                // Defines GPI functionality XI_GPI_MODE
pub static XI_PRM_GPI_LEVEL: &'static str = "gpi_level";               // GPI level
pub static XI_PRM_GPO_SELECTOR: &'static str = "gpo_selector";            // Selects GPO
pub static XI_PRM_GPO_MODE: &'static str = "gpo_mode";                // Defines GPO functionality XI_GPO_MODE
pub static XI_PRM_LED_SELECTOR: &'static str = "led_selector";            // Selects LED
pub static XI_PRM_LED_MODE: &'static str = "led_mode";                // Defines LED functionality XI_LED_MODE
pub static XI_PRM_TS_RST_MODE: &'static str = "ts_rst_mode";             // Defines how time stamp reset engine will be armed XI_TS_RST_MODE
pub static XI_PRM_TS_RST_SOURCE: &'static str = "ts_rst_source";           // Defines which source will be used for timestamp reset. Writing this parameter will trigger settings of engine (arming) XI_TS_RST_SOURCE
pub static XI_PRM_ACQ_FRAME_BURST_COUNT: &'static str = "acq_frame_burst_count";   // Sets number of frames acquired by burst. This burst is used only if trigger is set to FrameBurstStart
// Extended Device parameters
pub static XI_PRM_IS_DEVICE_EXIST: &'static str = "isexist";                 // Returns 1 if camera connected and works properly. XI_SWITCH
pub static XI_PRM_ACQ_BUFFER_SIZE: &'static str = "acq_buffer_size";         // Acquisition buffer size in buffer_size_unit. Default bytes.
pub static XI_PRM_ACQ_BUFFER_SIZE_UNIT: &'static str = "acq_buffer_size_unit";    // Acquisition buffer size unit in bytes. Default 1. E.g. Value 1024 means that buffer_size is in KiBytes
pub static XI_PRM_ACQ_TRANSPORT_BUFFER_SIZE: &'static str = "acq_transport_buffer_size";// Acquisition transport buffer size in bytes
pub static XI_PRM_BUFFERS_QUEUE_SIZE: &'static str = "buffers_queue_size";      // Queue of field/frame buffers
pub static XI_PRM_RECENT_FRAME: &'static str = "recent_frame";            // GetImage returns most recent frame XI_SWITCH
// Color management settings
pub static XI_PRM_CMS: &'static str = "cms";                     // Mode of color management system. XI_CMS_MODE
pub static XI_PRM_APPLY_CMS: &'static str = "apply_cms";               // Enable applying of CMS profiles to xiGetImage (see XI_PRM_INPUT_CMS_PROFILE, XI_PRM_OUTPUT_CMS_PROFILE). XI_SWITCH
pub static XI_PRM_INPUT_CMS_PROFILE: &'static str = "input_cms_profile";       // Filename for input cms profile (e.g. input.icc)
pub static XI_PRM_OUTPUT_CMS_PROFILE: &'static str = "output_cms_profile";      // Filename for output cms profile (e.g. input.icc)
pub static XI_PRM_IMAGE_IS_COLOR: &'static str = "iscolor";                 // Returns 1 for color cameras. XI_SWITCH
pub static XI_PRM_COLOR_FILTER_ARRAY: &'static str = "cfa";                     // Returns color filter array type of RAW data. XI_COLOR_FILTER_ARRAY
pub static XI_PRM_WB_KR: &'static str = "wb_kr";                   // White balance red coefficient
pub static XI_PRM_WB_KG: &'static str = "wb_kg";                   // White balance green coefficient
pub static XI_PRM_WB_KB: &'static str = "wb_kb";                   // White balance blue coefficient
pub static XI_PRM_MANUAL_WB: &'static str = "manual_wb";               // Calculates White Balance(xiGetImage function must be called) XI_SWITCH
pub static XI_PRM_AUTO_WB: &'static str = "auto_wb";                 // Automatic white balance XI_SWITCH
pub static XI_PRM_GAMMAY: &'static str = "gammaY";                  // Luminosity gamma
pub static XI_PRM_GAMMAC: &'static str = "gammaC";                  // Chromaticity gamma
pub static XI_PRM_SHARPNESS: &'static str = "sharpness";               // Sharpness Strenght
pub static XI_PRM_CC_MATRIX_00: &'static str = "ccMTX00";                 // Color Correction Matrix element [0][0]
pub static XI_PRM_CC_MATRIX_01: &'static str = "ccMTX01";                 // Color Correction Matrix element [0][1]
pub static XI_PRM_CC_MATRIX_02: &'static str = "ccMTX02";                 // Color Correction Matrix element [0][2]
pub static XI_PRM_CC_MATRIX_03: &'static str = "ccMTX03";                 // Color Correction Matrix element [0][3]
pub static XI_PRM_CC_MATRIX_10: &'static str = "ccMTX10";                 // Color Correction Matrix element [1][0]
pub static XI_PRM_CC_MATRIX_11: &'static str = "ccMTX11";                 // Color Correction Matrix element [1][1]
pub static XI_PRM_CC_MATRIX_12: &'static str = "ccMTX12";                 // Color Correction Matrix element [1][2]
pub static XI_PRM_CC_MATRIX_13: &'static str = "ccMTX13";                 // Color Correction Matrix element [1][3]
pub static XI_PRM_CC_MATRIX_20: &'static str = "ccMTX20";                 // Color Correction Matrix element [2][0]
pub static XI_PRM_CC_MATRIX_21: &'static str = "ccMTX21";                 // Color Correction Matrix element [2][1]
pub static XI_PRM_CC_MATRIX_22: &'static str = "ccMTX22";                 // Color Correction Matrix element [2][2]
pub static XI_PRM_CC_MATRIX_23: &'static str = "ccMTX23";                 // Color Correction Matrix element [2][3]
pub static XI_PRM_CC_MATRIX_30: &'static str = "ccMTX30";                 // Color Correction Matrix element [3][0]
pub static XI_PRM_CC_MATRIX_31: &'static str = "ccMTX31";                 // Color Correction Matrix element [3][1]
pub static XI_PRM_CC_MATRIX_32: &'static str = "ccMTX32";                 // Color Correction Matrix element [3][2]
pub static XI_PRM_CC_MATRIX_33: &'static str = "ccMTX33";                 // Color Correction Matrix element [3][3]
pub static XI_PRM_DEFAULT_CC_MATRIX: &'static str = "defccMTX";                // Set default Color Correction Matrix
// Automatic exposure/gain
pub static XI_PRM_AEAG: &'static str = "aeag";                    // Automatic exposure/gain XI_SWITCH
pub static XI_PRM_AEAG_ROI_OFFSET_X: &'static str = "aeag_roi_offset_x";       // Automatic exposure/gain ROI offset X
pub static XI_PRM_AEAG_ROI_OFFSET_Y: &'static str = "aeag_roi_offset_y";       // Automatic exposure/gain ROI offset Y
pub static XI_PRM_AEAG_ROI_WIDTH: &'static str = "aeag_roi_width";          // Automatic exposure/gain ROI Width
pub static XI_PRM_AEAG_ROI_HEIGHT: &'static str = "aeag_roi_height";         // Automatic exposure/gain ROI Height
pub static XI_PRM_EXP_PRIORITY: &'static str = "exp_priority";            // Exposure priority (0.8 - exposure 80%, gain 20%).
pub static XI_PRM_AE_MAX_LIMIT: &'static str = "ae_max_limit";            // Maximum time (us) used for exposure in AEAG procedure
pub static XI_PRM_AG_MAX_LIMIT: &'static str = "ag_max_limit";            // Maximum limit of gain in AEAG procedure
pub static XI_PRM_AEAG_LEVEL: &'static str = "aeag_level";              // Average intensity of output signal AEAG should achieve(in %)
// Bad Pixels Correction
pub static XI_PRM_BPC: &'static str = "bpc";                     // Correction of bad pixels XI_SWITCH
// Debounce
pub static XI_PRM_DEBOUNCE_EN: &'static str = "dbnc_en";                 // Enable/Disable debounce to selected GPI XI_SWITCH
pub static XI_PRM_DEBOUNCE_T0: &'static str = "dbnc_t0";                 // Debounce time (x * 10us)
pub static XI_PRM_DEBOUNCE_T1: &'static str = "dbnc_t1";                 // Debounce time (x * 10us)
pub static XI_PRM_DEBOUNCE_POL: &'static str = "dbnc_pol";                // Debounce polarity (pol = 1 t0 - falling edge, t1 - rising edge)
// Temperature control
pub static XI_PRM_IS_COOLED: &'static str = "iscooled";                // Returns 1 for cameras that support cooling.
pub static XI_PRM_COOLING: &'static str = "cooling";                 // Start camera cooling. XI_SWITCH
pub static XI_PRM_TARGET_TEMP: &'static str = "target_temp";             // Set sensor target temperature for cooling.
pub static XI_PRM_CHIP_TEMP: &'static str = "chip_temp";               // Camera sensor temperature
pub static XI_PRM_HOUS_TEMP: &'static str = "hous_temp";               // Camera housing tepmerature
// Sensor features
pub static XI_PRM_SENSOR_MODE: &'static str = "sensor_mode";             // Current sensor mode. Allows to select sensor mode by one integer. Setting of this parameter affects: image dimensions and downsampling.
pub static XI_PRM_HDR: &'static str = "hdr";                     // Enable High Dynamic Range feature. XI_SWITCH
pub static XI_PRM_HDR_KNEEPOINT_COUNT: &'static str = "hdr_kneepoint_count";     // The number of kneepoints in the PWLR.
pub static XI_PRM_HDR_T1: &'static str = "hdr_t1";                  // position of first kneepoint(in % of XI_PRM_EXPOSURE)
pub static XI_PRM_HDR_T2: &'static str = "hdr_t2";                  // position of second kneepoint (in % of XI_PRM_EXPOSURE)
pub static XI_PRM_KNEEPOINT1: &'static str = "hdr_kneepoint1";          // value of first kneepoint (% of sensor saturation)
pub static XI_PRM_KNEEPOINT2: &'static str = "hdr_kneepoint2";          // value of second kneepoint (% of sensor saturation)
pub static XI_PRM_IMAGE_BLACK_LEVEL: &'static str = "image_black_level";       // Last image black level counts. Can be used for Offline processing to recall it.
// Version info
pub static XI_PRM_API_VERSION: &'static str = "api_version";             // Returns version of API.
pub static XI_PRM_DRV_VERSION: &'static str = "drv_version";             // Returns version of current device driver.
pub static XI_PRM_MCU1_VERSION: &'static str = "version_mcu1";            // Returns version of MCU1 firmware.
pub static XI_PRM_MCU2_VERSION: &'static str = "version_mcu2";            // Returns version of MCU2 firmware.
pub static XI_PRM_FPGA1_VERSION: &'static str = "version_fpga1";           // Returns version of FPGA1 firmware.
// API features
pub static XI_PRM_DEBUG_LEVEL: &'static str = "debug_level";             // Set debug level XI_DEBUG_LEVEL
pub static XI_PRM_AUTO_BANDWIDTH_CALCULATION: &'static str = "auto_bandwidth_calculation";// Automatic bandwidth calculation, XI_SWITCH
// Camera FFS
pub static XI_PRM_READ_FILE_FFS: &'static str = "read_file_ffs";           // Read file from camera flash filesystem.
pub static XI_PRM_WRITE_FILE_FFS: &'static str = "write_file_ffs";          // Write file to camera flash filesystem.
pub static XI_PRM_FFS_FILE_NAME: &'static str = "ffs_file_name";           // Set name of file to be written/read from camera FFS.
pub static XI_PRM_FREE_FFS_SIZE: &'static str = "free_ffs_size";           // Size of free camera FFS.
pub static XI_PRM_USED_FFS_SIZE: &'static str = "used_ffs_size";           // Size of used camera FFS.
pub static XI_PRM_FFS_ACCESS_KEY: &'static str = "ffs_access_key";          // Setting of key enables file operations on some cameras.
// APIContextControl
pub static XI_PRM_API_CONTEXT_LIST: &'static str = "xiapi_context_list";      // List of current parameters settings context - parameters with values. Used for offline processing.
// Lens Control
pub static XI_PRM_LENS_MODE: &'static str = "lens_mode";               // Status of lens control interface. This shall be set to XI_ON before any Lens operations. XI_SWITCH
pub static XI_PRM_LENS_APERTURE_VALUE: &'static str = "lens_aperture_value";     // Current lens aperture value in stops. Examples: 2.8, 4, 5.6, 8, 11
pub static XI_PRM_LENS_FOCUS_MOVEMENT_VALUE: &'static str = "lens_focus_movement_value";// Lens current focus movement value to be used by XI_PRM_LENS_FOCUS_MOVE in motor steps.
pub static XI_PRM_LENS_FOCUS_MOVE: &'static str = "lens_focus_move";         // Moves lens focus motor by steps set in XI_PRM_LENS_FOCUS_MOVEMENT_VALUE.
pub static XI_PRM_LENS_FOCUS_DISTANCE: &'static str = "lens_focus_distance";     // Lens focus distance in cm.
pub static XI_PRM_LENS_FOCAL_LENGTH: &'static str = "lens_focal_length";       // Lens focal distance in mm.
pub static XI_PRM_LENS_FEATURE_SELECTOR: &'static str = "lens_feature_selector";   // Selects the current feature which is accessible by XI_PRM_LENS_FEATURE. XI_LENS_FEATURE
pub static XI_PRM_LENS_FEATURE: &'static str = "lens_feature";            // Allows access to lens feature value currently selected by XI_PRM_LENS_FEATURE_SELECTOR.
// Sensor Control
pub static XI_PRM_SENSOR_FEATURE_SELECTOR: &'static str = "sensor_feature_selector"; // Selects the current feature which is accessible by XI_PRM_SENSOR_FEATURE_VALUE. XI_SENSOR_FEATURE_SELECTOR
pub static XI_PRM_SENSOR_FEATURE_VALUE: &'static str = "sensor_feature_value";    // Allows access to sensor feature value currently selected by XI_PRM_SENSOR_FEATURE_SELECTOR.


include!(concat!(env!("OUT_DIR"), "/lib/xiapi.rs"));
