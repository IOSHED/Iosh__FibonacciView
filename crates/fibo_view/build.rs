extern crate winres;

const PATH_TO_ICON: &str = "../../resources/logo.ico";

fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();

        res.set_icon(PATH_TO_ICON);
        res.set("FileDescription", "Calculator for Fibonacci numbers");
        res.set("ProductName", "Fibonacci View");

        res.compile().unwrap();
    }
}
