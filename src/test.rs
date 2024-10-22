use std::env::{current_dir, current_exe};
use std::ffi::CStr;
use std::thread;
use std::time::Duration;

use crate::{obs_add_data_path, obs_add_module_path, obs_audio_encoder_create, obs_audio_info, obs_data_create, obs_data_release, obs_data_set_bool, obs_data_set_int, obs_data_set_string, obs_encoder_set_audio, obs_encoder_set_video, obs_get_audio, obs_get_version_string, obs_get_video, obs_load_all_modules, obs_log_loaded_modules, obs_output_create, obs_output_get_last_error, obs_output_set_audio_encoder, obs_output_set_video_encoder, obs_output_start, obs_output_stop, obs_post_load_modules, obs_reset_audio, obs_reset_video, obs_scale_type_OBS_SCALE_BILINEAR, obs_set_output_source, obs_source_create, obs_startup, obs_video_encoder_create, obs_video_info, speaker_layout_SPEAKERS_STEREO, video_colorspace_VIDEO_CS_DEFAULT, video_format_VIDEO_FORMAT_NV12, video_range_type_VIDEO_RANGE_DEFAULT};
use crate::wrapper::ObsString;


#[test]
pub fn test_obs() {
    unsafe {
        let version = CStr::from_ptr(obs_get_version_string());
        println!("LibOBS version {}", version.to_str().unwrap());

        let locale = ObsString::from("en_US");
        let res = obs_startup(locale.as_ptr(), std::ptr::null(), std::ptr::null_mut());

        if !res {
            println!("Failed to start OBS");
        } else {
            println!("OBS started successfully");
        }

        let parent = current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();

        let tmp1 = parent.clone() + "/data/libobs/";
        let tmp2 = parent.clone() + "/obs-plugins/64bit/";
        let tmp3 = parent + "/data/obs-plugins/%module%";

        println!("{} {} {}", tmp1, tmp2, tmp3);
        let data = ObsString::new(&tmp1);
        let module_bin = ObsString::new(&tmp2);
        let data_str = ObsString::new(&tmp3);

        obs_add_data_path(data.as_ptr());
        obs_add_module_path(module_bin.as_ptr(), data_str.as_ptr());
        let audio_info = obs_audio_info {
            samples_per_sec: 44100,
            speakers: speaker_layout_SPEAKERS_STEREO
        };

        let reset_audio_code = obs_reset_audio(&audio_info as *const _);
        println!("Reset: {}", reset_audio_code);
        let main_width = 1360;
        let main_height = 768;

        let opengl = ObsString::new("libobs-opengl");
        let mut ovi = obs_video_info {
            adapter: 0,
            graphics_module: opengl.as_ptr(),
            fps_num: 60,
            fps_den: 1,
            base_width: main_width,
            base_height: main_height,
            output_width: main_width,
            output_height: main_height,
            output_format: video_format_VIDEO_FORMAT_NV12,
            gpu_conversion: true,
            colorspace: video_colorspace_VIDEO_CS_DEFAULT,
            range: video_range_type_VIDEO_RANGE_DEFAULT,
            scale_type: obs_scale_type_OBS_SCALE_BILINEAR
        };

        let reset_video_code = obs_reset_video(&mut ovi);
        if reset_video_code != 0 {
            panic!("Could not reset video {}", reset_video_code);
        }

        obs_load_all_modules();
        obs_log_loaded_modules();
        obs_post_load_modules();


        let vid_src_id = ObsString::new("monitor_capture");
        let vid_name = ObsString::new("Screen Capture Source");

        let vid_settings = obs_data_create();
        let restore_token = ObsString::new("RestoreToken");
        let restore_token_val = ObsString::new("2cd8ddf7-5d1c-4d97-823d-07d528677f88");

        obs_data_set_string(vid_settings, restore_token.as_ptr(), restore_token_val.as_ptr());
        let vid_src = obs_source_create(vid_src_id.as_ptr(), vid_name.as_ptr(), vid_settings, std::ptr::null_mut());
        obs_data_release(vid_settings);


        obs_set_output_source(0, vid_src);

        let vid_enc_settings = obs_data_create();
        let use_buf_size = ObsString::new("use_bufsize");
        let profile = ObsString::new("profile");
        let profile_val = ObsString::new("high");

        let preset = ObsString::new("preset");
        let preset_val = ObsString::new("veryfast");

        let rate_control = ObsString::new("rate_control");
        let rate_control_val = ObsString::new("CRF");
        let crf = ObsString::new("crf");

        obs_data_set_bool(vid_enc_settings, use_buf_size.as_ptr(), true);
        obs_data_set_string(vid_enc_settings, profile.as_ptr(), profile_val.as_ptr());
        obs_data_set_string(vid_enc_settings, preset.as_ptr(), preset_val.as_ptr());
        obs_data_set_string(vid_enc_settings, rate_control.as_ptr(), rate_control_val.as_ptr());
    
        obs_data_set_int(vid_enc_settings, crf.as_ptr(), 20);

        let vid_enc_id = ObsString::new("obs_x264");
        let vid_enc_idk = ObsString::new("simple_h264_recording");

        let vid_enc = obs_video_encoder_create(vid_enc_id.as_ptr(), vid_enc_idk.as_ptr(), vid_enc_settings, std::ptr::null_mut());
        obs_encoder_set_video(vid_enc, obs_get_video());

        obs_data_release(vid_enc_settings);

        let audio_enc_settings = obs_data_create();
        let device_id = ObsString::new("device_id");
        let device_id_val = ObsString::new("default");

        obs_data_set_string(audio_enc_settings, device_id.as_ptr(), device_id_val.as_ptr());

        let audio_enc_id = ObsString::new("pulse_output_capture");
        let audio_enc_name = ObsString::new("Audio Capture Source");

        let audio_src = obs_source_create(audio_enc_id.as_ptr(), audio_enc_name.as_ptr(), audio_enc_settings, std::ptr::null_mut());
        obs_data_release(audio_enc_settings);

        obs_set_output_source(1, audio_src);

        let audio_enc_id = ObsString::new("ffmpeg_aac");
        let audio_enc_name = ObsString::new("simple_aac_recording");
        let audio_enc = obs_audio_encoder_create(audio_enc_id.as_ptr(), audio_enc_name.as_ptr(), std::ptr::null_mut(), 0, std::ptr::null_mut());
        obs_encoder_set_audio(audio_enc, obs_get_audio());

        let rec_settings = obs_data_create();
        let rec_path = ObsString::new("path");

        let out_path = current_dir().unwrap().to_str().unwrap().to_owned() + "/recording.mp4";
        println!("Outputting to {}", out_path);
        let rec_path_val = ObsString::new(&out_path);

        obs_data_set_string(rec_settings, rec_path.as_ptr(), rec_path_val.as_ptr());

        let rec_id = ObsString::new("ffmpeg_muxer");
        let rec_name = ObsString::new("simple_ffmpeg_output");

        let rec_out = obs_output_create(rec_id.as_ptr(), rec_name.as_ptr(), rec_settings, std::ptr::null_mut());
        obs_data_release(rec_settings);

        obs_output_set_video_encoder(rec_out,vid_enc);
        obs_output_set_audio_encoder(rec_out, audio_enc, 0);


        let b = obs_output_start(rec_out);
        if !b {
            let err = obs_output_get_last_error(rec_out);
            let c_str = CStr::from_ptr(err);
            panic!("Failed to start recording {}", c_str.to_str().unwrap());
        } else {
            println!("Recording started");
        }

        thread::sleep(Duration::new(5, 0));
        obs_output_stop(rec_out);

        thread::sleep(Duration::new(3, 0));
    }
}