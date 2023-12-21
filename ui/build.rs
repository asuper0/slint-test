fn main() {
    slint_build::compile("./ui/window.slint").unwrap();

    // if cfg!(target_os = "windows") {
    //     let res = winres::WindowsResource::new();
    //     // res.set_icon("test.ico");
    //     res.compile().unwrap();
    // }
}
