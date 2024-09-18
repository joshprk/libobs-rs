use std::env::{current_dir, current_exe};
use std::ffi::{CStr, CString};
use std::thread;
use std::time::Duration;

use crate::{vec4_create, vec4_set, obs_add_data_path, obs_add_module_path, obs_audio_encoder_create, obs_audio_info, obs_data_create, obs_data_release, obs_data_set_bool, obs_data_set_int, obs_data_set_string, obs_encoder_set_audio, obs_encoder_set_video, obs_get_audio, obs_get_version_string, obs_get_video, obs_load_all_modules, obs_log_loaded_modules, obs_output_create, obs_output_get_last_error, obs_output_set_audio_encoder, obs_output_set_video_encoder, obs_output_start, obs_output_stop, obs_post_load_modules, obs_reset_audio, obs_reset_video, obs_scale_type_OBS_SCALE_BILINEAR, obs_set_output_source, obs_source_create, obs_startup, obs_video_encoder_create, obs_video_info, speaker_layout_SPEAKERS_STEREO, video_colorspace_VIDEO_CS_DEFAULT, video_format_VIDEO_FORMAT_NV12, video_range_type_VIDEO_RANGE_DEFAULT};

#[test]
pub fn test_obs() {
    unsafe {
        let version = CStr::from_ptr(obs_get_version_string());
        println!("LibOBS version {}", version.to_str().unwrap());

        let locale = CString::new("en-US").unwrap();
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
        let data = CString::new(tmp1.as_str()).unwrap();
        let module_bin = CString::new(tmp2.as_str()).unwrap();
        let data_str = CString::new(tmp3.as_str()).unwrap();

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

        let opengl = CString::new("libobs-opengl").unwrap();
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


        let vid_src_id = CString::new("monitor_capture").unwrap();
        let vid_name = CString::new("Screen Capture Source").unwrap();

        let vid_data = obs_data_create();
        let vid_data_id = CString::new("monitor_id").unwrap();
        let vid_data_id_1 = CString::new("monitor").unwrap();
        let vid_data_id_val = CString::new("\\\\.\\DISPLAY1").unwrap();

        obs_data_set_int(vid_data, vid_data_id_1.as_ptr(), 1);
        obs_data_set_string(vid_data, vid_data_id.as_ptr(), vid_data_id_val.as_ptr());


        let vid_src = obs_source_create(vid_src_id.as_ptr(), vid_name.as_ptr(), vid_data, std::ptr::null_mut());
        obs_data_release(vid_data);

        obs_set_output_source(0, vid_src);

        let vid_enc_settings = obs_data_create();
        let use_buf_size = CString::new("use_bufsize").unwrap();
        let profile = CString::new("profile").unwrap();
        let profile_val = CString::new("high").unwrap();

        let preset = CString::new("preset").unwrap();
        let preset_val = CString::new("veryfast").unwrap();

        let rate_control = CString::new("rate_control").unwrap();
        let rate_control_val = CString::new("CRF").unwrap();
        let crf = CString::new("crf").unwrap();

        obs_data_set_bool(vid_enc_settings, use_buf_size.as_ptr(), true);
        obs_data_set_string(vid_enc_settings, profile.as_ptr(), profile_val.as_ptr());
        obs_data_set_string(vid_enc_settings, preset.as_ptr(), preset_val.as_ptr());
        obs_data_set_string(vid_enc_settings, rate_control.as_ptr(), rate_control_val.as_ptr());

        obs_data_set_int(vid_enc_settings, crf.as_ptr(), 20);

        let vid_enc_id = CString::new("obs_x264").unwrap();
        let vid_enc_idk = CString::new("simple_h264_recording").unwrap();

        let vid_enc = obs_video_encoder_create(vid_enc_id.as_ptr(), vid_enc_idk.as_ptr(), vid_enc_settings, std::ptr::null_mut());
        obs_encoder_set_video(vid_enc, obs_get_video());

        obs_data_release(vid_enc_settings);

        let audio_enc_settings = obs_data_create();
        let device_id = CString::new("device_id").unwrap();
        let device_id_val = CString::new("default").unwrap();

        obs_data_set_string(audio_enc_settings, device_id.as_ptr(), device_id_val.as_ptr());

        let audio_enc_id = CString::new("pulse_output_capture").unwrap();
        let audio_enc_name = CString::new("Audio Capture Source").unwrap();

        let audio_src = obs_source_create(audio_enc_id.as_ptr(), audio_enc_name.as_ptr(), audio_enc_settings, std::ptr::null_mut());
        obs_data_release(audio_enc_settings);

        obs_set_output_source(1, audio_src);

        let audio_enc_id = CString::new("ffmpeg_aac").unwrap();
        let audio_enc_name = CString::new("simple_aac_recording").unwrap();
        let audio_enc = obs_audio_encoder_create(audio_enc_id.as_ptr(), audio_enc_name.as_ptr(), std::ptr::null_mut(), 0, std::ptr::null_mut());
        obs_encoder_set_audio(audio_enc, obs_get_audio());

        let rec_settings = obs_data_create();
        let rec_path = CString::new("path").unwrap();

        let out_path = current_dir().unwrap().to_str().unwrap().to_owned() + "/recording.mp4";
        println!("Outputting to {}", out_path);
        let rec_path_val = CString::new(out_path.as_str()).unwrap();

        obs_data_set_string(rec_settings, rec_path.as_ptr(), rec_path_val.as_ptr());

        let rec_id = CString::new("ffmpeg_muxer").unwrap();
        let rec_name = CString::new("simple_ffmpeg_output").unwrap();

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

        crate::obs_source_release(vid_src);
        crate::obs_encoder_release(audio_enc);
        crate::obs_source_release(audio_src);
        crate::obs_encoder_release(vid_enc);
        crate::obs_output_release(rec_out);
        crate::obs_shutdown();

        println!("OBS shutdown");
        println!("Allocs {}", unsafe { crate::bnum_allocs() });
    }
}

#[test]
pub fn vec_test() {
    unsafe {
        let x = 0.3f32;
        let y = 0.2f32;
        let z = 0.7f32;
        let w = 0.1f32;

        let mut v = vec4_create();
        let v = vec4_set(&mut v, x, y, z, w);
        let m = v.__bindgen_anon_1.m.m128_f32;

        println!("{:?}", m);
        assert_eq!(m[0], x);
        assert_eq!(m[1], y);
        assert_eq!(m[2], z);
        assert_eq!(m[3], w);
    }

}