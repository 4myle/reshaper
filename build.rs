
fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/Reshaper.ico");
        res.compile().unwrap();
    }
}
