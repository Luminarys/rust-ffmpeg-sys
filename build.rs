use std::env;

fn main() {
    let link_method;
    let link_dir;
    if let Ok(target_dir) = env::var("FFMPEG_DIR") {
        link_method = "static";
        link_dir = format!("{}/lib", target_dir);
    } else if let Ok(dir) = env::var("FFMPEG_LIB_DIR") {
        link_method = "dylib";
        link_dir = format!("{}", dir);
    } else {
        link_method = "dylib";
        link_dir = format!("/usr/lib");
    }

    println!("cargo:rustc-link-search={}", link_dir);

    println!("cargo:rustc-link-lib={}=z", link_method);

    println!("cargo:rustc-link-lib={}=mp3lame", link_method);
    println!("cargo:rustc-link-lib={}=opus", link_method);
    println!("cargo:rustc-link-lib={}=ogg", link_method);
    println!("cargo:rustc-link-lib={}=vorbis", link_method);
    println!("cargo:rustc-link-lib={}=vorbisfile", link_method);
    println!("cargo:rustc-link-lib={}=vorbisenc", link_method);

    println!("cargo:rustc-link-lib={}=avcodec", link_method);
    println!("cargo:rustc-link-lib={}=avutil", link_method);
    println!("cargo:rustc-link-lib={}=avformat", link_method);
    println!("cargo:rustc-link-lib={}=avfilter", link_method);
    println!("cargo:rustc-link-lib={}=avdevice", link_method);
    println!("cargo:rustc-link-lib={}=swresample", link_method);
    println!("cargo:rustc-link-lib={}=swscale", link_method);
}
