use philipshue::bridge;

#[cfg(all(feature = "nupnp", not(feature = "upnp")))]
pub fn discover() -> Vec<String> {
    use philipshue::hue::Discovery;
    bridge::discover().unwrap().into_iter().map(Discovery::into_ip).collect()
}

#[cfg(feature = "upnp")]
pub fn discover() -> Vec<String> {
    let mut ips = bridge::discover_upnp().unwrap();
    ips.dedup();
    ips
}

#[cfg(all(not(feature = "nupnp"), not(feature = "upnp")))]
pub fn discover() -> Vec<String> {
    panic!("Either UPnP or NUPnP is required for discovering!")
}
#[allow(dead_code)]
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u16, u8, u8) {
    let r = r as f64 / 255f64;
    let g = g as f64 / 255f64;
    let b = b as f64 / 255f64;
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));

    if max == min {
        (0, 0, (max * 255.) as u8)
    } else {
        let d = max - min;
        let s = d / max;
        let h = if max == r {
            (g - b) / d + (if g < b { 6f64 } else { 0f64 })
        } else if max == g {
            (b - r) / d + 2f64
        } else {
            (r - g) / d + 4f64
        };
        ((65535. * h / 6.) as u16, (s * 255.) as u8, (max * 255.) as u8)
    }
}