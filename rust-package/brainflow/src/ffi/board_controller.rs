/* automatically generated by rust-bindgen 0.59.1 */

#![allow(non_camel_case_types)]


extern "C" {
    pub fn get_board_descr(
        board_id: ::std::os::raw::c_int,
        board_descr: *mut ::std::os::raw::c_char,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_sampling_rate(
        board_id: ::std::os::raw::c_int,
        sampling_rate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_package_num_channel(
        board_id: ::std::os::raw::c_int,
        package_num_channel: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_timestamp_channel(
        board_id: ::std::os::raw::c_int,
        timestamp_channel: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_marker_channel(
        board_id: ::std::os::raw::c_int,
        marker_channel: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_battery_channel(
        board_id: ::std::os::raw::c_int,
        battery_channel: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_num_rows(
        board_id: ::std::os::raw::c_int,
        num_rows: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_eeg_names(
        board_id: ::std::os::raw::c_int,
        eeg_names: *mut ::std::os::raw::c_char,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_exg_channels(
        board_id: ::std::os::raw::c_int,
        exg_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_eeg_channels(
        board_id: ::std::os::raw::c_int,
        eeg_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_emg_channels(
        board_id: ::std::os::raw::c_int,
        emg_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_ecg_channels(
        board_id: ::std::os::raw::c_int,
        ecg_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_eog_channels(
        board_id: ::std::os::raw::c_int,
        eog_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_ppg_channels(
        board_id: ::std::os::raw::c_int,
        ppg_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_eda_channels(
        board_id: ::std::os::raw::c_int,
        eda_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_accel_channels(
        board_id: ::std::os::raw::c_int,
        accel_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_analog_channels(
        board_id: ::std::os::raw::c_int,
        analog_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_gyro_channels(
        board_id: ::std::os::raw::c_int,
        gyro_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_other_channels(
        board_id: ::std::os::raw::c_int,
        other_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_temperature_channels(
        board_id: ::std::os::raw::c_int,
        temperature_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_resistance_channels(
        board_id: ::std::os::raw::c_int,
        resistance_channels: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_device_name(
        board_id: ::std::os::raw::c_int,
        name: *mut ::std::os::raw::c_char,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn prepare_session(
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn start_stream(
        buffer_size: ::std::os::raw::c_int,
        streamer_params: *const ::std::os::raw::c_char,
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stop_stream(
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn release_session(
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_current_board_data(
        num_samples: ::std::os::raw::c_int,
        data_buf: *mut f64,
        returned_samples: *mut ::std::os::raw::c_int,
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_board_data_count(
        result: *mut ::std::os::raw::c_int,
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_board_data(
        data_count: ::std::os::raw::c_int,
        data_buf: *mut f64,
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn config_board(
        config: *mut ::std::os::raw::c_char,
        response: *mut ::std::os::raw::c_char,
        response_len: *mut ::std::os::raw::c_int,
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn is_prepared(
        prepared: *mut ::std::os::raw::c_int,
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn insert_marker(
        marker_value: f64,
        board_id: ::std::os::raw::c_int,
        json_brainflow_input_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn release_all_sessions() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn set_log_level_board_controller(
        log_level: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn set_log_file_board_controller(
        log_file: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn log_message_board_controller(
        log_level: ::std::os::raw::c_int,
        message: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JNINativeInterface {
    _unused: [u8; 0],
}
pub type JNIEnv = *const JNINativeInterface;
extern "C" {
    pub fn java_set_jnienv(java_jnienv: *mut JNIEnv) -> ::std::os::raw::c_int;
}