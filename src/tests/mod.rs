use crate::audio_analyze::analyze_audio_file;
use crate::key_detect::analyze_key_from_file;

//#[test]
//fn analyze_audio() {
//    let file_path =
//        "/Users/dobbikov/Desktop/coding/projects/key-sort-app/audio/Akrom-Deathmatch.mp3";
//    let (samples1, channels1, frame_rate1) = analyze_audio_file(file_path)
//        .expect("Coudln't process the file")
//        .data();
//
//    loggit::debug!("channels: {}, frame_rate1: {}", channels1, frame_rate1);
//    assert_eq!(3, 4);
//}

#[test]
fn get_key() {
    let file_path =
        "/Users/dobbikov/Desktop/coding/projects/key-sort-app/audio/Akrom-Deathmatch.mp3";

    let key = analyze_key_from_file(file_path);
    println!("{}", key);
    assert_eq!(3, 4);
}
