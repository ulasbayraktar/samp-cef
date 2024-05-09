fn main() {
    println!("cargo:rustc-link-search=native=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\dxsdk\\Lib\\x86");
    println!("cargo:rustc-link-lib=static=d3dx9");
}
