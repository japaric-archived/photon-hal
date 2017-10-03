//! Low level bindings to the HAL

use String;
use cty::{c_char, c_int, c_ulong, c_uint, int32_t, uint16_t, uint32_t, uint8_t};

pub type pin_t = u16;
pub type p_user_function_int_str_t = extern "C" fn(&String) -> c_int;
pub type system_tick_t = u32;

pub type sock_handle_t = uint32_t;
pub type sock_result_t = int32_t;
pub type socklen_t = usize;
pub type network_interface_t = uint32_t;

pub const AF_INET: uint8_t = 2;
pub const AF_INET6: uint8_t = 23;

#[repr(C)]
pub struct sockaddr_t {
    pub sa_family: uint16_t,
    pub sa_data: [uint8_t; 14],
}

#[repr(C)]
pub struct spark_variable_t {
    pub size: uint16_t,
    pub update: extern "C" fn(name: *const c_char,
                              ty: Spark_Data_TypeDef,
                              var: *const c_void,
                              reserved: *mut c_void),
}

// NOTE copied from the libc crate
// Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help enable
// more optimization opportunities around it recognizing things like
// malloc/free.
#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[repr(u32)]
pub enum PinMode {
    INPUT = 0,
    OUTPUT = 1,
    INPUT_PULLUP = 2,
    INPUT_PULLDOWN = 3,
    AF_OUTPUT_PUSHPULL = 4,
    AF_OUTPUT_DRAIN = 5,
    AN_INPUT = 6,
    AN_OUTPUT = 7,
    PIN_MODE_NONE = 255,
}

#[repr(u8)]
pub enum Spark_Data_TypeDef {
    CLOUD_VAR_BOOLEAN = 1,
    CLOUD_VAR_INT = 2,
    CLOUD_VAR_STRING = 4,
    CLOUD_VAR_DOUBLE = 9,
}

extern "C" {
    // hal
    /// like `delay` but prevents the system thread from running
    pub fn HAL_Delay_Milliseconds(ms: uint32_t);
    /// `delayMicroseconds`
    pub fn HAL_Delay_Microseconds(us: uint32_t);

    // hal_core
    /// Low level version of `deviceID`
    pub fn HAL_device_ID(
        dest: *mut uint8_t,
        dest_len: c_uint,
    ) -> c_uint;

    // hal_gpio
    /// `digitalWrite`
    pub fn HAL_GPIO_Read(pin: pin_t) -> int32_t;
    /// `digitalRead`
    pub fn HAL_GPIO_Write(pin: pin_t, value: uint8_t);
    /// `pinMode`
    pub fn HAL_Pin_Mode(pin: pin_t, mode: PinMode);

    // hal_usart (old API)
    /// `Serial.begin`
    pub fn USB_USART_Init(baud_rate: uint32_t);
    /// `Serial.write`
    pub fn USB_USART_Send_Data(byte: uint8_t);

    // system
    /// `delay`
    pub fn system_delay_ms(ms: c_ulong, force_no_background_loop: bool);

    // system_cloud
    /// `Particle.function`
    pub fn spark_function(
        name: *const c_char,
        f: p_user_function_int_str_t,
        _: *mut c_void,
    ) -> bool;
    /// `Particle.variable`
    pub fn spark_variable(
        name: *const c_char,
        var: *const c_void,
        ty: Spark_Data_TypeDef,
        _: *mut spark_variable_t,
    ) -> bool;
    /// `deviceID`
    pub fn spark_deviceID() -> String;
    /// `micros`
    pub fn HAL_Timer_Get_Micro_Seconds() -> system_tick_t;

    ///
    pub fn socket_active_status(handle: sock_handle_t) -> uint8_t;

    ///
    pub fn socket_create(
        family: uint8_t,
        type_: uint8_t,
        protocol: uint8_t,
        port: uint16_t,
        nif: network_interface_t,
    ) -> sock_handle_t;

    ///
    pub fn socket_connect(handle: sock_handle_t, addr: *const sockaddr_t) -> int32_t;

    // DYNALIB_FN(6, hal_socket, socket_send, sock_result_t(sock_handle_t, const void*, socklen_t))
    ///
    pub fn socket_send(handle: sock_handle_t, data: *const c_void, len: socklen_t) -> sock_result_t;

    // DYNALIB_FN(7, hal_socket, socket_sendto, sock_result_t(sock_handle_t, const void*, socklen_t, uint32_t, sockaddr_t*, socklen_t))
    ///
    pub fn socket_sendto(handle: sock_handle_t, data: *const c_void, len: socklen_t, addr: *const sockaddr_t) -> sock_result_t;

    // DYNALIB_FN(4, hal_socket, socket_receive, sock_result_t(sock_handle_t, void*, socklen_t, system_tick_t))
    ///
    pub fn socket_receive(handle: sock_handle_t, data: *const c_void, len: socklen_t, timeout: system_tick_t) -> sock_result_t;

    ///
    pub fn socket_close(handle: sock_handle_t) -> sock_result_t;
}

// TODO add bindings for all functions below, but be sure to know which
// Firmware API it maps to first so you can document it.
//
// To do that grab one of the DYNALIB_FN lines below and extract the third and
// fourth arguments. For instance:
//
// line:         DYNALIB_FN(3, hal_gpio, HAL_Get_Pin_Mode, PinMode(pin_t))
// 3rd argument: HAL_Get_Pin_Mode
// 4th argument: PinMode(pin_t)
//
// The 3rd argument is the function's name; the 4th argument is the
// function's signature. The signature needs to be converted to Rust syntax; the
// return type is specified first and the arguments types appear inside the
// parentheses. For the above example:
//
// 4th argument: PinMode(pin_t)
// return type:  PinMode
// arguments:    (pin_t)
// Rust syntax:  fn(pin_t) -> PinMode
//
// Finally write the binding in the `extern "C"` block above:
//
// extern "C" {
//     /// `GetPinMode`
//     fn HAL_Get_Pin_Mode(pin: pin_t) -> PinMode;
// }
//

// NOTE this list was generated by running `grep -h '^ *DYNALIB_FN(' **/*.h` in
// the spark/firmware repo checked out at tag v0.6.2
//
// DYNALIB_FN(0, communication, spark_protocol_instance, ProtocolFacade*(void))
// DYNALIB_FN(1, communication, spark_protocol_set_product_id, void(ProtocolFacade*, product_id_t, unsigned, void*))
// DYNALIB_FN(2, communication, spark_protocol_set_product_firmware_version, void(ProtocolFacade*, product_firmware_version_t, unsigned, void*))
// DYNALIB_FN(3, communication, spark_protocol_get_product_details, void(ProtocolFacade*, product_details_t*, void*))
// DYNALIB_FN(4, communication, spark_protocol_communications_handlers, void(ProtocolFacade*, CommunicationsHandlers*))
// DYNALIB_FN(5, communication, spark_protocol_init, void(ProtocolFacade*, const char*, const SparkKeys&, const SparkCallbacks&, const SparkDescriptor&, void*))
// DYNALIB_FN(6, communication, spark_protocol_handshake, int(ProtocolFacade*, void*))
// DYNALIB_FN(7, communication, spark_protocol_event_loop, bool(ProtocolFacade* protocol, void*))
// DYNALIB_FN(8, communication, spark_protocol_is_initialized, bool(ProtocolFacade*))
// DYNALIB_FN(9, communication, spark_protocol_presence_announcement, int(ProtocolFacade*, uint8_t*, const uint8_t*, void*))
// DYNALIB_FN(10, communication, spark_protocol_send_event, bool(ProtocolFacade*, const char*, const char*, int, uint32_t, void*))
// DYNALIB_FN(11, communication, spark_protocol_send_subscription_device, bool(ProtocolFacade*, const char*, const char*, void*))
// DYNALIB_FN(12, communication, spark_protocol_send_subscription_scope, bool(ProtocolFacade*, const char*, SubscriptionScope::Enum, void*))
// DYNALIB_FN(13, communication, spark_protocol_add_event_handler, bool(ProtocolFacade*, const char*, EventHandler, SubscriptionScope::Enum, const char*, void*))
// DYNALIB_FN(14, communication, spark_protocol_send_time_request, bool(ProtocolFacade*, void*))
// DYNALIB_FN(15, communication, spark_protocol_send_subscriptions, void(ProtocolFacade*, void*))
// DYNALIB_FN(16, communication, decrypt_rsa, int(const uint8_t*, const uint8_t*, uint8_t*, int))
// DYNALIB_FN(17, communication, gen_rsa_key, int(uint8_t*, size_t, int(*)(void*), void*))
// DYNALIB_FN(18, communication, extract_public_rsa_key, void(uint8_t*, const uint8_t*))
// DYNALIB_FN(BASE_IDX + 0, communication, spark_protocol_remove_event_handlers, void(ProtocolFacade*, const char*, void*))
// DYNALIB_FN(BASE_IDX + 1, communication, gen_ec_key, int(uint8_t*, size_t, int(*)(void*, uint8_t*, size_t), void*))
// DYNALIB_FN(BASE_IDX + 2, communication, extract_public_ec_key, int(uint8_t*, size_t, const uint8_t*))
// DYNALIB_FN(BASE_IDX2 + 0, communication, spark_protocol_set_connection_property,
// DYNALIB_FN(BASE_IDX2 + 1, communication, spark_protocol_command, int(ProtocolFacade* protocol, ProtocolCommands::Enum cmd, uint32_t data, void* reserved))
// DYNALIB_FN(BASE_IDX2 + 2, communication, spark_protocol_time_request_pending, bool(ProtocolFacade*, void*))
// DYNALIB_FN(BASE_IDX2 + 3, communication, spark_protocol_time_last_synced, system_tick_t(ProtocolFacade*, time_t*, void*))
// DYNALIB_FN(0, hal_bootloader, HAL_Bootloader_Image, const uint8_t*(uint32_t*, void*))
// DYNALIB_FN(0, hal_can, HAL_CAN_Init, void(HAL_CAN_Channel, uint16_t, uint16_t, void*))
// DYNALIB_FN(1, hal_can, HAL_CAN_Begin, void(HAL_CAN_Channel, uint32_t, uint32_t, void*))
// DYNALIB_FN(2, hal_can, HAL_CAN_End, void(HAL_CAN_Channel, void*))
// DYNALIB_FN(3, hal_can, HAL_CAN_Transmit, bool(HAL_CAN_Channel, const CANMessage*, void*))
// DYNALIB_FN(4, hal_can, HAL_CAN_Receive, bool(HAL_CAN_Channel, CANMessage*, void*))
// DYNALIB_FN(5, hal_can, HAL_CAN_Available_Messages, uint8_t(HAL_CAN_Channel, void*))
// DYNALIB_FN(6, hal_can, HAL_CAN_Add_Filter, bool(HAL_CAN_Channel, uint32_t, uint32_t, HAL_CAN_Filters, void*))
// DYNALIB_FN(7, hal_can, HAL_CAN_Clear_Filters, void(HAL_CAN_Channel, void*))
// DYNALIB_FN(8, hal_can, HAL_CAN_Is_Enabled, bool(HAL_CAN_Channel))
// DYNALIB_FN(9, hal_can, HAL_CAN_Error_Status, HAL_CAN_Errors(HAL_CAN_Channel))
// DYNALIB_FN(0, hal_cellular, cellular_off, cellular_result_t(void*))
// DYNALIB_FN(1, hal_cellular, cellular_on, cellular_result_t(void*))
// DYNALIB_FN(2, hal_cellular, cellular_init, cellular_result_t(void*))
// DYNALIB_FN(3, hal_cellular, cellular_register, cellular_result_t(void*))
// DYNALIB_FN(4, hal_cellular, cellular_pdp_activate, cellular_result_t(CellularCredentials*, void*))
// DYNALIB_FN(5, hal_cellular, cellular_pdp_deactivate, cellular_result_t(void*))
// DYNALIB_FN(6, hal_cellular, cellular_gprs_attach, cellular_result_t(CellularCredentials*, void*))
// DYNALIB_FN(7, hal_cellular, cellular_gprs_detach, cellular_result_t(void*))
// DYNALIB_FN(8, hal_cellular, cellular_fetch_ipconfig, cellular_result_t(CellularConfig*, void*))
// DYNALIB_FN(9, hal_cellular, cellular_device_info, cellular_result_t(CellularDevice*, void*))
// DYNALIB_FN(10, hal_cellular, cellular_credentials_set, cellular_result_t(const char*, const char*, const char*, void*))
// DYNALIB_FN(11, hal_cellular, cellular_credentials_get, CellularCredentials*(void*))
// DYNALIB_FN(12, hal_cellular, cellular_sim_ready, bool(void*))
// DYNALIB_FN(13, hal_cellular, cellular_cancel, void(bool, bool, void*))
// DYNALIB_FN(14, hal_cellular, HAL_NET_SetNetWatchDog, uint32_t(uint32_t))
// DYNALIB_FN(15, hal_cellular, inet_gethostbyname, int(const char*, uint16_t, HAL_IPAddress*, network_interface_t, void*))
// DYNALIB_FN(16, hal_cellular, inet_ping, int(const HAL_IPAddress*, network_interface_t, uint8_t, void*))
// DYNALIB_FN(17, hal_cellular, cellular_signal, cellular_result_t(CellularSignalHal&, void*))
// DYNALIB_FN(18, hal_cellular, cellular_command, cellular_result_t(_CALLBACKPTR_MDM, void*, system_tick_t, const char*, ...))
// DYNALIB_FN(19, hal_cellular, cellular_data_usage_set, cellular_result_t(CellularDataHal*,void*))
// DYNALIB_FN(20, hal_cellular, cellular_data_usage_get, cellular_result_t(CellularDataHal*,void*))
// DYNALIB_FN(21, hal_cellular, cellular_band_select_set, cellular_result_t(MDM_BandSelect* bands, void* reserved))
// DYNALIB_FN(22, hal_cellular, cellular_band_select_get, cellular_result_t(MDM_BandSelect* bands, void* reserved))
// DYNALIB_FN(23, hal_cellular, cellular_band_available_get, cellular_result_t(MDM_BandSelect* bands, void* reserved))
// DYNALIB_FN(24, hal_cellular, cellular_sms_received_handler_set, cellular_result_t(_CELLULAR_SMS_CB_MDM cb, void* data, void* reserved))
// DYNALIB_FN(25, hal_cellular, HAL_USART3_Handler_Impl, void(void*))
// DYNALIB_FN(26, hal_cellular, HAL_NET_SetCallbacks, void(const HAL_NET_Callbacks*, void*))
// DYNALIB_FN(27, hal_cellular, cellular_pause, cellular_result_t(void*))
// DYNALIB_FN(28, hal_cellular, cellular_resume, cellular_result_t(void*))
// DYNALIB_FN(29, hal_cellular, cellular_imsi_to_network_provider, cellular_result_t(void*))
// DYNALIB_FN(30, hal_cellular, cellular_network_provider_data_get, const CellularNetProvData(void*))
// DYNALIB_FN(0, hal_concurrent, __gthread_equal, bool(__gthread_t, __gthread_t))
// DYNALIB_FN(1, hal_concurrent, os_thread_create, os_result_t(os_thread_t*, const char*, os_thread_prio_t, os_thread_fn_t, void*, size_t))
// DYNALIB_FN(2, hal_concurrent, os_thread_is_current, bool(os_thread_t))
// DYNALIB_FN(3, hal_concurrent, os_thread_yield, os_result_t(void))
// DYNALIB_FN(4, hal_concurrent, os_thread_join, os_result_t(os_thread_t))
// DYNALIB_FN(5, hal_concurrent, os_thread_cleanup, os_result_t(os_thread_t))
// DYNALIB_FN(6, hal_concurrent, os_thread_delay_until, os_result_t(system_tick_t*, system_tick_t))
// DYNALIB_FN(7, hal_concurrent, os_thread_scheduling, void(bool, void*))
// DYNALIB_FN(8, hal_concurrent, os_timer_create, int(os_timer_t*, unsigned, void(*)(os_timer_t), void*, bool, void*))
// DYNALIB_FN(9, hal_concurrent, os_timer_destroy, int(os_timer_t, void*))
// DYNALIB_FN(10, hal_concurrent, os_timer_get_id, int(os_timer_t, void**))
// DYNALIB_FN(11, hal_concurrent, os_timer_change, int(os_timer_t, os_timer_change_t, bool, unsigned, unsigned, void*))
// DYNALIB_FN(12, hal_concurrent, os_mutex_create, int(os_mutex_t*))
// DYNALIB_FN(13, hal_concurrent, os_mutex_destroy, int(os_mutex_t))
// DYNALIB_FN(14, hal_concurrent, os_mutex_lock, int(os_mutex_t))
// DYNALIB_FN(15, hal_concurrent, os_mutex_trylock, int(os_mutex_t))
// DYNALIB_FN(16, hal_concurrent, os_mutex_unlock, int(os_mutex_t))
// DYNALIB_FN(17, hal_concurrent, os_mutex_recursive_create, int(os_mutex_recursive_t*))
// DYNALIB_FN(18, hal_concurrent, os_mutex_recursive_destroy, int(os_mutex_recursive_t))
// DYNALIB_FN(19, hal_concurrent, os_mutex_recursive_lock, int(os_mutex_recursive_t))
// DYNALIB_FN(20, hal_concurrent, os_mutex_recursive_trylock, int(os_mutex_recursive_t))
// DYNALIB_FN(21, hal_concurrent, os_mutex_recursive_unlock, int(os_mutex_recursive_t))
// DYNALIB_FN(22, hal_concurrent, os_timer_is_active, int(os_timer_t, void*))
// DYNALIB_FN(23, hal_concurrent, os_queue_create, int(os_queue_t*, size_t, size_t, void*))
// DYNALIB_FN(24, hal_concurrent, os_queue_destroy, int(os_queue_t, void*))
// DYNALIB_FN(25, hal_concurrent, os_queue_put, int(os_queue_t, const void* item, system_tick_t, void*))
// DYNALIB_FN(26, hal_concurrent, os_queue_take, int(os_queue_t, void* item, system_tick_t, void*))
// DYNALIB_FN(0, hal_core, HAL_core_subsystem_version, int(char*, int))
// DYNALIB_FN(1, hal_core, HAL_Core_Init, void(void))
// DYNALIB_FN(2, hal_core, HAL_Core_Config, void(void))
// DYNALIB_FN(3, hal_core, HAL_Core_Mode_Button_Pressed, bool(uint16_t))
// DYNALIB_FN(4, hal_core, HAL_Core_Mode_Button_Reset, void(uint16_t))
// DYNALIB_FN(5, hal_core, HAL_Core_System_Reset, void(void))
// DYNALIB_FN(6, hal_core, HAL_Core_Factory_Reset, void(void))
// DYNALIB_FN(7, hal_core, HAL_Core_Enter_Bootloader, void(bool))
// DYNALIB_FN(8, hal_core, HAL_Core_Enter_Stop_Mode, void(uint16_t, uint16_t, long))
// DYNALIB_FN(9, hal_core, HAL_Core_Execute_Stop_Mode, void(void))
// DYNALIB_FN(10, hal_core, HAL_Core_Enter_Standby_Mode, void(uint32_t, void*))
// DYNALIB_FN(11, hal_core, HAL_Core_Execute_Standby_Mode, void(void))
// DYNALIB_FN(12, hal_core, HAL_Core_Compute_CRC32, uint32_t(const uint8_t*, uint32_t))
// DYNALIB_FN(14, hal_core, HAL_Get_Sys_Health, eSystemHealth(void))
// DYNALIB_FN(15, hal_core, HAL_Set_Sys_Health, void(eSystemHealth))
// DYNALIB_FN(16, hal_core, HAL_watchdog_reset_flagged, bool(void))
// DYNALIB_FN(17, hal_core, HAL_Notify_WDT, void(void))
// DYNALIB_FN(18, hal_core, HAL_Bootloader_Get_Flag, uint16_t(BootloaderFlag))
// DYNALIB_FN(19, hal_core, HAL_Bootloader_Lock, void(bool))
// DYNALIB_FN(20, hal_core, HAL_Core_System_Reset_FlagSet, bool(RESET_TypeDef))
// DYNALIB_FN(21, hal_core, HAL_Core_Runtime_Info, uint32_t(runtime_info_t*, void*))
// DYNALIB_FN(22, hal_core, HAL_Set_System_Config, int(hal_system_config_t, const void*, unsigned))
// DYNALIB_FN(23, hal_core, HAL_Core_Enter_Safe_Mode, void(void*))
// DYNALIB_FN(24, hal_core, HAL_Feature_Get, bool(HAL_Feature))
// DYNALIB_FN(25, hal_core, HAL_Feature_Set, int(HAL_Feature, bool))
// DYNALIB_FN(26, hal_core, HAL_Core_System_Reset_Ex, void(int, uint32_t, void*))
// DYNALIB_FN(27, hal_core, HAL_Core_Get_Last_Reset_Info, int(int*, uint32_t*, void*))
// DYNALIB_FN(28, hal_core, HAL_Core_Button_Mirror_Pin, void(uint16_t, InterruptMode, uint8_t, uint8_t, void*))
// DYNALIB_FN(29, hal_core, HAL_Core_Button_Mirror_Pin_Disable, void(uint8_t, uint8_t, void*))
// DYNALIB_FN(30, hal_core, HAL_Core_Led_Mirror_Pin, void(uint8_t, pin_t, uint32_t, uint8_t, void*))
// DYNALIB_FN(31, hal_core, HAL_Core_Led_Mirror_Pin_Disable, void(uint8_t, uint8_t, void*))
// DYNALIB_FN(32, hal_core, HAL_Set_Event_Callback, void(HAL_Event_Callback, void*))
// DYNALIB_FN(0, hal_gpio, HAL_Pin_Map, STM32_Pin_Info*(void))
// DYNALIB_FN(1, hal_gpio, HAL_Validate_Pin_Function, PinFunction(pin_t, PinFunction))
// DYNALIB_FN(3, hal_gpio, HAL_Get_Pin_Mode, PinMode(pin_t))
// DYNALIB_FN(6, hal_gpio, HAL_Interrupts_Attach, void(uint16_t, HAL_InterruptHandler, void*, InterruptMode, HAL_InterruptExtraConfiguration*))
// DYNALIB_FN(7, hal_gpio, HAL_Interrupts_Detach, void(uint16_t))
// DYNALIB_FN(8, hal_gpio, HAL_Interrupts_Enable_All, void(void))
// DYNALIB_FN(9, hal_gpio, HAL_Interrupts_Disable_All, void(void))
// DYNALIB_FN(10, hal_gpio, HAL_DAC_Write, void(pin_t, uint16_t))
// DYNALIB_FN(11, hal_gpio, HAL_ADC_Set_Sample_Time, void(uint8_t))
// DYNALIB_FN(12, hal_gpio, HAL_ADC_Read, int32_t(uint16_t))
// DYNALIB_FN(13, hal_gpio, HAL_PWM_Write, void(uint16_t, uint8_t))
// DYNALIB_FN(14, hal_gpio, HAL_PWM_Get_Frequency, uint16_t(uint16_t))
// DYNALIB_FN(15, hal_gpio, HAL_PWM_Get_AnalogValue, uint16_t(uint16_t))
// DYNALIB_FN(16, hal_gpio, HAL_Set_System_Interrupt_Handler, uint8_t(hal_irq_t, const HAL_InterruptCallback*, HAL_InterruptCallback*, void*))
// DYNALIB_FN(17, hal_gpio, HAL_Get_System_Interrupt_Handler, uint8_t(hal_irq_t, HAL_InterruptCallback*, void*))
// DYNALIB_FN(18, hal_gpio, HAL_System_Interrupt_Trigger, void(hal_irq_t, void*))
// DYNALIB_FN(19, hal_gpio, HAL_Pulse_In, uint32_t(pin_t, uint16_t))
// DYNALIB_FN(20, hal_gpio, HAL_Interrupts_Suspend, void(void))
// DYNALIB_FN(21, hal_gpio, HAL_Interrupts_Restore, void(void))
// DYNALIB_FN(22, hal_gpio, HAL_PWM_Write_With_Frequency, void(uint16_t, uint8_t, uint16_t))
// DYNALIB_FN(23, hal_gpio, HAL_DAC_Is_Enabled, uint8_t(pin_t))
// DYNALIB_FN(24, hal_gpio, HAL_DAC_Enable, uint8_t(pin_t, uint8_t))
// DYNALIB_FN(25, hal_gpio, HAL_DAC_Get_Resolution, uint8_t(pin_t))
// DYNALIB_FN(26, hal_gpio, HAL_DAC_Set_Resolution, void(pin_t, uint8_t))
// DYNALIB_FN(27, hal_gpio, HAL_DAC_Enable_Buffer, void(pin_t pin, uint8_t state))
// DYNALIB_FN(28, hal_gpio, HAL_PWM_Get_Resolution, uint8_t(uint16_t))
// DYNALIB_FN(29, hal_gpio, HAL_PWM_Set_Resolution, void(uint16_t, uint8_t))
// DYNALIB_FN(30, hal_gpio, HAL_PWM_Write_Ext, void(uint16_t, uint32_t))
// DYNALIB_FN(31, hal_gpio, HAL_PWM_Write_With_Frequency_Ext, void(uint16_t, uint32_t, uint32_t))
// DYNALIB_FN(32, hal_gpio, HAL_PWM_Get_Frequency_Ext, uint32_t(uint16_t))
// DYNALIB_FN(33, hal_gpio, HAL_PWM_Get_AnalogValue_Ext, uint32_t(uint16_t))
// DYNALIB_FN(34, hal_gpio, HAL_PWM_Get_Max_Frequency, uint32_t(uint16_t))
// DYNALIB_FN(35, hal_gpio, HAL_Interrupts_Detach_Ext, void(uint16_t, uint8_t, void*))
// DYNALIB_FN(0, hal, HAL_RNG_Configuration, void(void))
// DYNALIB_FN(1, hal, HAL_RNG_GetRandomNumber, uint32_t(void))
// DYNALIB_FN(BASE_IDX + 2, hal, HAL_Timer_Get_Micro_Seconds, system_tick_t(void))
// DYNALIB_FN(BASE_IDX + 3, hal, HAL_Timer_Get_Milli_Seconds, system_tick_t(void))
// DYNALIB_FN(BASE_IDX + 4, hal, HAL_RTC_Configuration, void(void))
// DYNALIB_FN(BASE_IDX + 5, hal, HAL_RTC_Get_UnixTime, time_t(void))
// DYNALIB_FN(BASE_IDX + 6, hal, HAL_RTC_Set_UnixTime, void(time_t))
// DYNALIB_FN(BASE_IDX + 7, hal, HAL_RTC_Set_UnixAlarm, void(time_t))
// DYNALIB_FN(BASE_IDX + 8, hal, HAL_EEPROM_Init, void(void))
// DYNALIB_FN(BASE_IDX + 9, hal, HAL_EEPROM_Read, uint8_t(uint32_t))
// DYNALIB_FN(BASE_IDX + 10, hal, HAL_EEPROM_Write, void(uint32_t, uint8_t))
// DYNALIB_FN(BASE_IDX + 11, hal, HAL_EEPROM_Length, size_t(void))
// DYNALIB_FN(BASE_IDX + 12, hal, HAL_disable_irq, int(void))
// DYNALIB_FN(BASE_IDX + 13, hal, HAL_enable_irq, void(int))
// DYNALIB_FN(BASE_IDX + 14, hal, HAL_RTC_Cancel_UnixAlarm, void(void))
// DYNALIB_FN(BASE_IDX + 15, hal,HAL_EEPROM_Get, void(uint32_t, void *, size_t))
// DYNALIB_FN(BASE_IDX + 16, hal,HAL_EEPROM_Put, void(uint32_t, const void *, size_t))
// DYNALIB_FN(BASE_IDX + 17, hal,HAL_EEPROM_Clear, void(void))
// DYNALIB_FN(BASE_IDX + 18, hal,HAL_EEPROM_Has_Pending_Erase, bool(void))
// DYNALIB_FN(BASE_IDX + 19, hal,HAL_EEPROM_Perform_Pending_Erase, void(void))
// DYNALIB_FN(BASE_IDX + 20, hal, HAL_RTC_Time_Is_Valid, uint8_t(void*))
// DYNALIB_FN(0, hal_i2c, HAL_I2C_Set_Speed_v1, void(uint32_t))
// DYNALIB_FN(1, hal_i2c, HAL_I2C_Enable_DMA_Mode_v1, void(bool))
// DYNALIB_FN(2, hal_i2c, HAL_I2C_Stretch_Clock_v1, void(bool))
// DYNALIB_FN(3, hal_i2c, HAL_I2C_Begin_v1, void(I2C_Mode, uint8_t))
// DYNALIB_FN(4, hal_i2c, HAL_I2C_End_v1, void(void))
// DYNALIB_FN(5, hal_i2c, HAL_I2C_Request_Data_v1, uint32_t(uint8_t, uint8_t, uint8_t))
// DYNALIB_FN(6, hal_i2c, HAL_I2C_Begin_Transmission_v1, void(uint8_t))
// DYNALIB_FN(7, hal_i2c, HAL_I2C_End_Transmission_v1, uint8_t(uint8_t))
// DYNALIB_FN(8, hal_i2c, HAL_I2C_Write_Data_v1, uint32_t(uint8_t))
// DYNALIB_FN(9, hal_i2c, HAL_I2C_Available_Data_v1, int32_t(void))
// DYNALIB_FN(10, hal_i2c, HAL_I2C_Read_Data_v1, int32_t(void))
// DYNALIB_FN(11, hal_i2c, HAL_I2C_Peek_Data_v1, int32_t(void))
// DYNALIB_FN(12, hal_i2c, HAL_I2C_Flush_Data_v1, void(void))
// DYNALIB_FN(13, hal_i2c, HAL_I2C_Is_Enabled_v1, bool(void))
// DYNALIB_FN(14, hal_i2c, HAL_I2C_Set_Callback_On_Receive_v1, void(void(*)(int)))
// DYNALIB_FN(15, hal_i2c, HAL_I2C_Set_Callback_On_Request_v1, void(void(*)(void)))
// DYNALIB_FN(BASE_IDX + 0, hal_i2c, HAL_I2C_Set_Speed, void(HAL_I2C_Interface, uint32_t, void*))
// DYNALIB_FN(BASE_IDX + 1, hal_i2c, HAL_I2C_Enable_DMA_Mode, void(HAL_I2C_Interface, bool, void*))
// DYNALIB_FN(BASE_IDX + 2, hal_i2c, HAL_I2C_Stretch_Clock, void(HAL_I2C_Interface, bool, void*))
// DYNALIB_FN(BASE_IDX + 3, hal_i2c, HAL_I2C_Begin, void(HAL_I2C_Interface, I2C_Mode, uint8_t, void*))
// DYNALIB_FN(BASE_IDX + 4, hal_i2c, HAL_I2C_End, void(HAL_I2C_Interface, void*))
// DYNALIB_FN(BASE_IDX + 5, hal_i2c, HAL_I2C_Request_Data, uint32_t(HAL_I2C_Interface, uint8_t, uint8_t, uint8_t, void*))
// DYNALIB_FN(BASE_IDX + 6, hal_i2c, HAL_I2C_Begin_Transmission, void(HAL_I2C_Interface, uint8_t, void*))
// DYNALIB_FN(BASE_IDX + 7, hal_i2c, HAL_I2C_End_Transmission, uint8_t(HAL_I2C_Interface, uint8_t, void*))
// DYNALIB_FN(BASE_IDX + 8, hal_i2c, HAL_I2C_Write_Data, uint32_t(HAL_I2C_Interface, uint8_t, void*))
// DYNALIB_FN(BASE_IDX + 9, hal_i2c, HAL_I2C_Available_Data, int32_t(HAL_I2C_Interface, void*))
// DYNALIB_FN(BASE_IDX + 10, hal_i2c, HAL_I2C_Read_Data, int32_t(HAL_I2C_Interface, void*))
// DYNALIB_FN(BASE_IDX + 11, hal_i2c, HAL_I2C_Peek_Data, int32_t(HAL_I2C_Interface, void*))
// DYNALIB_FN(BASE_IDX + 12, hal_i2c, HAL_I2C_Flush_Data, void(HAL_I2C_Interface, void*))
// DYNALIB_FN(BASE_IDX + 13, hal_i2c, HAL_I2C_Is_Enabled, bool(HAL_I2C_Interface, void*))
// DYNALIB_FN(BASE_IDX + 14, hal_i2c, HAL_I2C_Set_Callback_On_Receive, void(HAL_I2C_Interface, void(*)(int), void*))
// DYNALIB_FN(BASE_IDX + 15, hal_i2c, HAL_I2C_Set_Callback_On_Request, void(HAL_I2C_Interface, void(*)(void), void*))
// DYNALIB_FN(BASE_IDX + 16, hal_i2c, HAL_I2C_Init, void(HAL_I2C_Interface, void*))
// DYNALIB_FN(0, hal_ota, HAL_OTA_FlashAddress, uint32_t(void))
// DYNALIB_FN(1, hal_ota, HAL_OTA_FlashLength, uint32_t(void))
// DYNALIB_FN(2, hal_ota, HAL_OTA_ChunkSize, uint16_t(void))
// DYNALIB_FN(3, hal_ota, HAL_OTA_Flashed_GetStatus, bool(void))
// DYNALIB_FN(4, hal_ota, HAL_OTA_Flashed_ResetStatus, void(void))
// DYNALIB_FN(5, hal_ota, HAL_FLASH_Begin, bool(uint32_t, uint32_t, void*))
// DYNALIB_FN(6, hal_ota, HAL_FLASH_Update, int(const uint8_t*, uint32_t, uint32_t, void*))
// DYNALIB_FN(7, hal_ota, HAL_FLASH_End, hal_update_complete_t(hal_module_t*))
// DYNALIB_FN(0, hal_peripherals, HAL_Tone_Start, void(uint8_t, uint32_t, uint32_t))
// DYNALIB_FN(1, hal_peripherals, HAL_Tone_Stop, void(uint8_t))
// DYNALIB_FN(2, hal_peripherals, HAL_Tone_Get_Frequency, uint32_t(uint8_t))
// DYNALIB_FN(3, hal_peripherals, HAL_Tone_Is_Stopped, bool(uint8_t))
// DYNALIB_FN(4, hal_peripherals, HAL_Servo_Attach, void(uint16_t))
// DYNALIB_FN(5, hal_peripherals, HAL_Servo_Detach, void(uint16_t))
// DYNALIB_FN(6, hal_peripherals, HAL_Servo_Write_Pulse_Width, void(uint16_t, uint16_t))
// DYNALIB_FN(7, hal_peripherals, HAL_Servo_Read_Pulse_Width, uint16_t(uint16_t))
// DYNALIB_FN(8, hal_peripherals, HAL_Servo_Read_Frequency, uint16_t(uint16_t))
// DYNALIB_FN(0, hal_rgbled, HAL_Led_Rgb_Set_Values, void(uint16_t, uint16_t, uint16_t, void*))
// DYNALIB_FN(1, hal_rgbled, HAL_Led_Rgb_Get_Values, void(uint16_t*, void*))
// DYNALIB_FN(2, hal_rgbled, HAL_Led_Rgb_Get_Max_Value, uint32_t(void*))
// DYNALIB_FN(3, hal_rgbled, HAL_Led_User_Set, void(uint8_t, void*))
// DYNALIB_FN(4, hal_rgbled, HAL_Led_User_Toggle, void(void*))
// DYNALIB_FN(0, hal_socket, socket_active_status, uint8_t(sock_handle_t))
// DYNALIB_FN(1, hal_socket, socket_handle_valid, uint8_t(sock_handle_t))
// DYNALIB_FN(2, hal_socket, socket_create, sock_handle_t(uint8_t, uint8_t, uint8_t, uint16_t, network_interface_t))
// DYNALIB_FN(3, hal_socket, socket_connect, int32_t(sock_handle_t, const sockaddr_t*, long))
// DYNALIB_FN(4, hal_socket, socket_receive, sock_result_t(sock_handle_t, void*, socklen_t, system_tick_t))
// DYNALIB_FN(5, hal_socket, socket_receivefrom, sock_result_t(sock_handle_t, void*, socklen_t, uint32_t, sockaddr_t*, socklen_t*))
// DYNALIB_FN(6, hal_socket, socket_send, sock_result_t(sock_handle_t, const void*, socklen_t))
// DYNALIB_FN(7, hal_socket, socket_sendto, sock_result_t(sock_handle_t, const void*, socklen_t, uint32_t, sockaddr_t*, socklen_t))
// DYNALIB_FN(8, hal_socket, socket_close, sock_result_t(sock_handle_t))
// DYNALIB_FN(9, hal_socket, socket_reset_blocking_call, sock_result_t(void))
// DYNALIB_FN(10, hal_socket, socket_create_tcp_server, sock_result_t(uint16_t, network_interface_t))
// DYNALIB_FN(11, hal_socket, socket_accept, sock_result_t(sock_handle_t))
// DYNALIB_FN(12, hal_socket, socket_handle_invalid, sock_handle_t(void))
// DYNALIB_FN(13, hal_socket, socket_join_multicast, sock_result_t(const HAL_IPAddress*, network_interface_t, socket_multicast_info_t*))
// DYNALIB_FN(14, hal_socket, socket_leave_multicast, sock_result_t(const HAL_IPAddress*, network_interface_t, socket_multicast_info_t*))
// DYNALIB_FN(15, hal_socket, socket_peer, sock_result_t(sock_handle_t, sock_peer_t*, void*))
// DYNALIB_FN(0, hal_spi, HAL_SPI_Begin, void(HAL_SPI_Interface, uint16_t))
// DYNALIB_FN(1, hal_spi, HAL_SPI_End, void(HAL_SPI_Interface))
// DYNALIB_FN(2, hal_spi, HAL_SPI_Set_Bit_Order, void(HAL_SPI_Interface, uint8_t))
// DYNALIB_FN(3, hal_spi, HAL_SPI_Set_Data_Mode, void(HAL_SPI_Interface, uint8_t))
// DYNALIB_FN(4, hal_spi, HAL_SPI_Set_Clock_Divider, void(HAL_SPI_Interface, uint8_t))
// DYNALIB_FN(5, hal_spi, HAL_SPI_Send_Receive_Data, uint16_t(HAL_SPI_Interface, uint16_t))
// DYNALIB_FN(6, hal_spi, HAL_SPI_Is_Enabled_Old, bool(void))
// DYNALIB_FN(7, hal_spi, HAL_SPI_Init, void(HAL_SPI_Interface))
// DYNALIB_FN(8, hal_spi, HAL_SPI_Is_Enabled, bool(HAL_SPI_Interface))
// DYNALIB_FN(9, hal_spi, HAL_SPI_Info, void(HAL_SPI_Interface, hal_spi_info_t*, void*))
// DYNALIB_FN(10, hal_spi, HAL_SPI_DMA_Transfer, void(HAL_SPI_Interface, void*, void*, uint32_t, HAL_SPI_DMA_UserCallback))
// DYNALIB_FN(11, hal_spi, HAL_SPI_Begin_Ext, void(HAL_SPI_Interface, SPI_Mode, uint16_t, void*))
// DYNALIB_FN(12, hal_spi, HAL_SPI_Set_Callback_On_Select, void(HAL_SPI_Interface, HAL_SPI_Select_UserCallback, void*))
// DYNALIB_FN(13, hal_spi, HAL_SPI_DMA_Transfer_Cancel, void(HAL_SPI_Interface))
// DYNALIB_FN(14, hal_spi, HAL_SPI_DMA_Transfer_Status, int32_t(HAL_SPI_Interface, HAL_SPI_TransferStatus*))
// DYNALIB_FN(15, hal_spi, HAL_SPI_Set_Settings, int32_t(HAL_SPI_Interface, uint8_t, uint8_t, uint8_t, uint8_t, void*))
// DYNALIB_FN(1, hal_usart, USB_USART_Available_Data, uint8_t(void))
// DYNALIB_FN(2, hal_usart, USB_USART_Receive_Data, int32_t(uint8_t))
// DYNALIB_FN(4, hal_usart, USB_USART_Baud_Rate, unsigned(void))
// DYNALIB_FN(5, hal_usart, USB_USART_LineCoding_BitRate_Handler, void(void(*)(uint32_t)))
// DYNALIB_FN(BASE_IDX + 0, hal_usart, HAL_USART_Init, void(HAL_USART_Serial, Ring_Buffer*, Ring_Buffer*))
// DYNALIB_FN(BASE_IDX + 1, hal_usart, HAL_USART_Begin, void(HAL_USART_Serial, uint32_t))
// DYNALIB_FN(BASE_IDX + 2, hal_usart, HAL_USART_End, void(HAL_USART_Serial))
// DYNALIB_FN(BASE_IDX + 3, hal_usart, HAL_USART_Write_Data, uint32_t(HAL_USART_Serial, uint8_t))
// DYNALIB_FN(BASE_IDX + 4, hal_usart, HAL_USART_Available_Data, int32_t(HAL_USART_Serial))
// DYNALIB_FN(BASE_IDX + 5, hal_usart, HAL_USART_Read_Data, int32_t(HAL_USART_Serial))
// DYNALIB_FN(BASE_IDX + 6, hal_usart, HAL_USART_Peek_Data, int32_t(HAL_USART_Serial))
// DYNALIB_FN(BASE_IDX + 7, hal_usart, HAL_USART_Flush_Data, void(HAL_USART_Serial))
// DYNALIB_FN(BASE_IDX + 8, hal_usart, HAL_USART_Is_Enabled, bool(HAL_USART_Serial))
// DYNALIB_FN(BASE_IDX + 9, hal_usart, HAL_USART_Half_Duplex, void(HAL_USART_Serial, bool))
// DYNALIB_FN(BASE_IDX + 10, hal_usart, HAL_USART_Available_Data_For_Write, int32_t(HAL_USART_Serial))
// DYNALIB_FN(BASE_IDX + 11, hal_usart, USB_USART_Available_Data_For_Write, int32_t(void))
// DYNALIB_FN(BASE_IDX + 12, hal_usart, USB_USART_Flush_Data, void(void))
// DYNALIB_FN(BASE_IDX2 + 0, hal_usart, HAL_USART_BeginConfig, void(HAL_USART_Serial serial, uint32_t baud, uint32_t config, void *ptr))
// DYNALIB_FN(BASE_IDX2 + 1, hal_usart, HAL_USART_Write_NineBitData, uint32_t(HAL_USART_Serial serial, uint16_t data))
// DYNALIB_FN(BASE_IDX2 + 2, hal_usart, HAL_USART_Send_Break, void(HAL_USART_Serial, void*))
// DYNALIB_FN(BASE_IDX2 + 3, hal_usart, HAL_USART_Break_Detected, uint8_t(HAL_USART_Serial))
// DYNALIB_FN(0, hal_usb, HAL_USB_USART_Init, void(HAL_USB_USART_Serial, const HAL_USB_USART_Config*))
// DYNALIB_FN(1, hal_usb, HAL_USB_USART_Begin, void(HAL_USB_USART_Serial, uint32_t, void *))
// DYNALIB_FN(2, hal_usb, HAL_USB_USART_End, void(HAL_USB_USART_Serial))
// DYNALIB_FN(3, hal_usb, HAL_USB_USART_Baud_Rate, unsigned int(HAL_USB_USART_Serial))
// DYNALIB_FN(4, hal_usb, HAL_USB_USART_Available_Data, int32_t(HAL_USB_USART_Serial))
// DYNALIB_FN(5, hal_usb, HAL_USB_USART_Available_Data_For_Write, int32_t(HAL_USB_USART_Serial))
// DYNALIB_FN(6, hal_usb, HAL_USB_USART_Receive_Data, int32_t(HAL_USB_USART_Serial, uint8_t))
// DYNALIB_FN(7, hal_usb, HAL_USB_USART_Send_Data, int32_t(HAL_USB_USART_Serial, uint8_t))
// DYNALIB_FN(8, hal_usb, HAL_USB_USART_Flush_Data, void(HAL_USB_USART_Serial))
// DYNALIB_FN(9, hal_usb, HAL_USB_USART_Is_Enabled, bool(HAL_USB_USART_Serial))
// DYNALIB_FN(10, hal_usb, HAL_USB_USART_Is_Connected, bool(HAL_USB_USART_Serial))
// DYNALIB_FN(11, hal_usb, HAL_USB_USART_LineCoding_BitRate_Handler, int32_t(void (*handler)(uint32_t bitRate), void* reserved))
// DYNALIB_FN(BASE_IDX + 0, hal_usb, HAL_USB_HID_Init, void(uint8_t, void*))
// DYNALIB_FN(BASE_IDX + 1, hal_usb, HAL_USB_HID_Begin, void(uint8_t, void*))
// DYNALIB_FN(BASE_IDX + 2, hal_usb, HAL_USB_HID_Send_Report, void(uint8_t, void*, uint16_t, void*))
// DYNALIB_FN(BASE_IDX + 3, hal_usb, HAL_USB_HID_End, void(uint8_t))
// DYNALIB_FN(BASE_IDX1 + 0, hal_usb, HAL_USB_Init, void(void))
// DYNALIB_FN(BASE_IDX1 + 1, hal_usb, HAL_USB_Attach, void(void))
// DYNALIB_FN(BASE_IDX1 + 2, hal_usb, HAL_USB_Detach, void(void))
// DYNALIB_FN(BASE_IDX2 + 0, hal_usb, HAL_USB_Set_Vendor_Request_Callback, void(HAL_USB_Vendor_Request_Callback, void*))
// DYNALIB_FN(BASE_IDX4 + 0, hal_usb, HAL_USB_HID_Status, int32_t(uint8_t, void*))
// DYNALIB_FN(BASE_IDX4 + 1, hal_usb, HAL_USB_HID_Set_State, uint8_t(uint8_t, uint8_t, void*))
// DYNALIB_FN(0, hal_wlan, wlan_connect_init, wlan_result_t(void))
// DYNALIB_FN(1, hal_wlan, wlan_connect_finalize, wlan_result_t(void))
// DYNALIB_FN(2, hal_wlan, wlan_reset_credentials_store_required, bool(void))
// DYNALIB_FN(3, hal_wlan, wlan_reset_credentials_store, wlan_result_t(void))
// DYNALIB_FN(4, hal_wlan, wlan_disconnect_now, wlan_result_t(void))
// DYNALIB_FN(5, hal_wlan, wlan_activate, wlan_result_t(void))
// DYNALIB_FN(6, hal_wlan, wlan_deactivate, wlan_result_t(void))
// DYNALIB_FN(7, hal_wlan, wlan_connected_rssi, int(void))
// DYNALIB_FN(8, hal_wlan, wlan_clear_credentials, int(void))
// DYNALIB_FN(9, hal_wlan, wlan_has_credentials, int(void))
// DYNALIB_FN(10, hal_wlan, wlan_set_credentials, int(WLanCredentials*))
// DYNALIB_FN(11, hal_wlan, wlan_smart_config_init, void(void))
// DYNALIB_FN(12, hal_wlan, wlan_smart_config_cleanup, void(void))
// DYNALIB_FN(13, hal_wlan, wlan_smart_config_finalize, bool(void))
// DYNALIB_FN(14, hal_wlan, wlan_set_error_count, void(uint32_t))
// DYNALIB_FN(15, hal_wlan, wlan_fetch_ipconfig, void(WLanConfig*))
// DYNALIB_FN(16, hal_wlan, wlan_setup, void(void))
// DYNALIB_FN(17, hal_wlan, HAL_NET_SetNetWatchDog, uint32_t(uint32_t))
// DYNALIB_FN(18, hal_wlan, inet_gethostbyname, int(const char*, uint16_t, HAL_IPAddress*, network_interface_t, void*))
// DYNALIB_FN(19, hal_wlan, inet_ping, int(const HAL_IPAddress*, network_interface_t, uint8_t, void*))
// DYNALIB_FN(20, hal_wlan, wlan_select_antenna, int(WLanSelectAntenna_TypeDef))
// DYNALIB_FN(21, hal_wlan, wlan_set_ipaddress, void(const HAL_IPAddress*, const HAL_IPAddress*, const HAL_IPAddress*, const HAL_IPAddress*, const HAL_IPAddress*, void*))
// DYNALIB_FN(22, hal_wlan, wlan_set_ipaddress_source, void(IPAddressSource, bool, void*))
// DYNALIB_FN(23, hal_wlan, wlan_scan, int(wlan_scan_result_t, void*))
// DYNALIB_FN(24, hal_wlan, wlan_get_credentials, int(wlan_scan_result_t, void*))
// DYNALIB_FN(25, hal_wlan,softap_set_application_page_handler, int(PageProvider* provider, void* reserved))
// DYNALIB_FN(0, wifi_resource, wwd_firmware_image_resource, const resource_hnd_t*(void))
// DYNALIB_FN(1, wifi_resource, wwd_nvram_image_resource, const resource_hnd_t*(void))
// DYNALIB_FN(2, wifi_resource, wwd_select_nvram_image_resource, int(uint8_t, void*))
// DYNALIB_FN(0, system_module_part1, module_system_part1_pre_init, void*(void))
// DYNALIB_FN(1, system_module_part1, module_system_part1_init, void(void))
// DYNALIB_FN(0, system_module_part3, module_system_part3_pre_init, void*(void))
// DYNALIB_FN(1, system_module_part3, module_system_part3_init, void(void))
// DYNALIB_FN(0, user, module_user_pre_init, void*(void))
// DYNALIB_FN(1, user, module_user_init, void(void))
// DYNALIB_FN(2, user, module_user_setup, void(void))
// DYNALIB_FN(3, user, module_user_loop, void(void))
// DYNALIB_FN(0, rt, malloc, void*(size_t))
// DYNALIB_FN(1, rt, free, void(void*))
// DYNALIB_FN(2, rt, realloc, void*(void*, size_t))
// DYNALIB_FN(3, rt, sprintf, int(char*, const char*, ...))
// DYNALIB_FN(4, rt, siprintf, int(char*, const char*, ...))
// DYNALIB_FN(5, rt, sscanf, int(const char*, const char*, ...))
// DYNALIB_FN(6, rt, siscanf, int(const char*, const char*, ...))
// DYNALIB_FN(7, rt, snprintf, int(char*, size_t, const char*, ...))
// DYNALIB_FN(8, rt, sniprintf, int(char*, size_t, const char*, ...))
// DYNALIB_FN(9, rt, vsnprintf, int(char*, size_t, const char*, va_list))
// DYNALIB_FN(10, rt, vsniprintf, int(char*, size_t, const char*, va_list))
// DYNALIB_FN(11, rt, abort, void(void))
// DYNALIB_FN(12, rt, _malloc_r, void*(struct _reent*, size_t))
// DYNALIB_FN(13, rt, _free_r, void(struct _reent*, void*))
// DYNALIB_FN(14, rt, _realloc_r, void*(struct _reent*, void*, size_t))
// DYNALIB_FN(0, services, LED_SetRGBColor, void(uint32_t))
// DYNALIB_FN(1, services, LED_SetSignalingColor, void(uint32_t))
// DYNALIB_FN(2, services, LED_Signaling_Start, void(void))
// DYNALIB_FN(3, services, LED_Signaling_Stop, void(void))
// DYNALIB_FN(4, services, LED_SetBrightness, void(uint8_t))
// DYNALIB_FN(5, services, LED_RGB_Get, void(uint8_t*))
// DYNALIB_FN(6, services, LED_RGB_IsOverRidden, bool(void))
// DYNALIB_FN(7, services, LED_On, void(Led_TypeDef))
// DYNALIB_FN(8, services, LED_Off, void(Led_TypeDef))
// DYNALIB_FN(9, services, LED_Toggle, void(Led_TypeDef))
// DYNALIB_FN(10, services, LED_Fade, void(Led_TypeDef))
// DYNALIB_FN(11, services, Get_LED_Brightness, uint8_t(void))
// DYNALIB_FN(12, services, set_logger_output, void(debug_output_fn, LoggerOutputLevel)) // Deprecated
// DYNALIB_FN(13, services, panic_, void(ePanicCode, void*, void(*)(uint32_t)))
// DYNALIB_FN(14, services, jsmn_init, void(jsmn_parser*, void*))
// DYNALIB_FN(15, services, jsmn_parse, jsmnerr_t(jsmn_parser*, const char*, size_t, jsmntok_t*, unsigned int, void*))
// DYNALIB_FN(16, services, log_print_, void(int, int, const char*, const char*, const char*, ...)) // Deprecated
// DYNALIB_FN(17, services, LED_RGB_SetChangeHandler, void(led_update_handler_fn, void*))
// DYNALIB_FN(18, services, log_print_direct_, void(int, void*, const char*, ...)) // Deprecated
// DYNALIB_FN(19, services, LED_GetColor, uint32_t(uint32_t, void*))
// DYNALIB_FN(20, services, log_message, void(int, const char*, LogAttributes*, void*, const char*, ...))
// DYNALIB_FN(21, services, log_message_v, void(int, const char*, LogAttributes*, void*, const char*, va_list))
// DYNALIB_FN(22, services, log_write, void(int, const char*, const char*, size_t, void*))
// DYNALIB_FN(23, services, log_printf, void(int, const char*, void*, const char*, ...))
// DYNALIB_FN(24, services, log_printf_v, void(int, const char*, void*, const char*, va_list))
// DYNALIB_FN(25, services, log_dump, void(int, const char*, const void*, size_t, int, void*))
// DYNALIB_FN(26, services, log_enabled, int(int, const char*, void*))
// DYNALIB_FN(27, services, log_level_name, const char*(int, void*))
// DYNALIB_FN(28, services, log_set_callbacks, void(log_message_callback_type, log_write_callback_type, log_enabled_callback_type, void*))
// DYNALIB_FN(29, services, set_thread_current_function_pointers, void(void*, void*, void*, void*, void*))
// DYNALIB_FN(30, services, system_error_message, const char*(int, void*))
// DYNALIB_FN(31, services, LED_SetCallbacks, void(LedCallbacks, void*))
// DYNALIB_FN(32, services, led_set_status_active, void(LEDStatusData*, int, void*))
// DYNALIB_FN(33, services, led_set_update_enabled, void(int, void*))
// DYNALIB_FN(34, services, led_update_enabled, int(void*))
// DYNALIB_FN(35, services, led_update, void(system_tick_t, LEDStatusData*, void*))
// DYNALIB_FN(2, system_cloud, spark_process, void(void))
// DYNALIB_FN(3, system_cloud, spark_cloud_flag_connect, void(void))
// DYNALIB_FN(4, system_cloud, spark_cloud_flag_disconnect, void(void))
// DYNALIB_FN(5, system_cloud, spark_cloud_flag_connected, bool(void))
// DYNALIB_FN(6, system_cloud, system_cloud_protocol_instance, ProtocolFacade*(void))
// DYNALIB_FN(8, system_cloud, spark_send_event, bool(const char*, const char*, int, uint32_t, void*))
// DYNALIB_FN(9, system_cloud, spark_subscribe, bool(const char*, EventHandler, void*, Spark_Subscription_Scope_TypeDef, const char*, void*))
// DYNALIB_FN(10, system_cloud, spark_unsubscribe, void(void*))
// DYNALIB_FN(11, system_cloud, spark_sync_time, bool(void*))
// DYNALIB_FN(12, system_cloud, spark_sync_time_pending, bool(void*))
// DYNALIB_FN(13, system_cloud, spark_sync_time_last, system_tick_t(time_t*, void*))
// DYNALIB_FN(14, system_cloud, spark_set_connection_property, int(unsigned, unsigned, void*, void*))
// DYNALIB_FN(0, system, system_mode, System_Mode_TypeDef(void))
// DYNALIB_FN(1, system, set_system_mode, void(System_Mode_TypeDef))
// DYNALIB_FN(2, system, set_ymodem_serial_flash_update_handler, void(ymodem_serial_flash_update_handler))
// DYNALIB_FN(3, system, system_firmwareUpdate, bool(Stream*, void*))
// DYNALIB_FN(4, system, system_fileTransfer, bool(system_file_transfer_t*, void*))
// DYNALIB_FN(6, system, system_sleep, void(Spark_Sleep_TypeDef, long, uint32_t, void*))
// DYNALIB_FN(7, system, system_sleep_pin, void(uint16_t, uint16_t, long, uint32_t, void*))
// DYNALIB_FN(8, system, system_subscribe_event, int(system_event_t, system_event_handler_t*, void*))
// DYNALIB_FN(9, system, system_unsubscribe_event, void(system_event_t, system_event_handler_t*, void*))
// DYNALIB_FN(10, system, system_button_pushed_duration, uint16_t(uint8_t, void*))
// DYNALIB_FN(11, system, system_thread_set_state, void(spark::feature::State, void*))
// DYNALIB_FN(12, system, system_version_info, int(SystemVersionInfo*, void*))
// DYNALIB_FN(13, system, system_internal, void*(int item, void*))
// DYNALIB_FN(14, system, system_set_flag, int(system_flag_t, uint8_t, void*))
// DYNALIB_FN(15, system, system_get_flag, int(system_flag_t, uint8_t*, void*))
// DYNALIB_FN(16, system, Spark_Prepare_For_Firmware_Update, int(FileTransfer::Descriptor&, uint32_t, void*))
// DYNALIB_FN(17, system, Spark_Save_Firmware_Chunk, int(FileTransfer::Descriptor&, const uint8_t*, void*))
// DYNALIB_FN(18, system, Spark_Finish_Firmware_Update, int(FileTransfer::Descriptor&, uint32_t, void*))
// DYNALIB_FN(19, system, application_thread_current, uint8_t(void*))
// DYNALIB_FN(20, system, system_thread_current, uint8_t(void*))
// DYNALIB_FN(21, system, application_thread_invoke, uint8_t(void(*)(void*), void*, void*))
// DYNALIB_FN(22, system, system_thread_get_state, spark::feature::State(void*))
// DYNALIB_FN(23, system, system_notify_time_changed, void(uint32_t, void*, void*))
// DYNALIB_FN(24, system, main_thread_current, uint8_t(void*))
// DYNALIB_FN(25, system, system_set_usb_request_app_handler, void(usb_request_app_handler_type, void*))
// DYNALIB_FN(26, system, system_set_usb_request_result, void(USBRequest*, int, void*))
// DYNALIB_FN(BASE_IDX + 0, system, led_start_signal, int(int, uint8_t, int, void*))
// DYNALIB_FN(BASE_IDX + 1, system, led_stop_signal, void(int, int, void*))
// DYNALIB_FN(BASE_IDX + 2, system, led_signal_started, int(int, void*))
// DYNALIB_FN(BASE_IDX + 3, system, led_set_signal_theme, int(const LEDSignalThemeData*, int, void*))
// DYNALIB_FN(BASE_IDX + 4, system, led_get_signal_theme, int(LEDSignalThemeData*, int, void*))
// DYNALIB_FN(BASE_IDX + 5, system, led_signal_status, const LEDStatusData*(int, void*))
// DYNALIB_FN(BASE_IDX + 6, system, led_pattern_period, uint16_t(int, int, void*))
// DYNALIB_FN(0, system_net, network_config, const void*(network_handle_t, uint32_t, void*))
// DYNALIB_FN(1, system_net, network_connect, void(network_handle_t, uint32_t, uint32_t, void*))
// DYNALIB_FN(2, system_net, network_connecting, bool(network_handle_t, uint32_t, void*))
// DYNALIB_FN(3, system_net, network_disconnect, void(network_handle_t, uint32_t, void*))
// DYNALIB_FN(4, system_net, network_ready, bool(network_handle_t, uint32_t, void*))
// DYNALIB_FN(5, system_net, network_on, void(network_handle_t, uint32_t, uint32_t, void*))
// DYNALIB_FN(6, system_net, network_off, void(network_handle_t, uint32_t, uint32_t, void*))
// DYNALIB_FN(7, system_net, network_listen, void(network_handle_t, uint32_t, void*))
// DYNALIB_FN(8, system_net, network_listening, bool(network_handle_t, uint32_t, void*))
// DYNALIB_FN(9, system_net, network_has_credentials, bool(network_handle_t, uint32_t, void*))
// DYNALIB_FN(10, system_net, network_set_credentials, int(network_handle_t, uint32_t, NetworkCredentials*, void*))
// DYNALIB_FN(11, system_net, network_clear_credentials, bool(network_handle_t, uint32_t, NetworkCredentials*, void*))
// DYNALIB_FN(12, system_net, network_set_listen_timeout, void(network_handle_t, uint16_t, void*))
// DYNALIB_FN(13, system_net, network_get_listen_timeout, uint16_t(network_handle_t, uint32_t, void*))
