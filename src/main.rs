use x11rb::connect;
use x11rb::connection::Connection;
use x11rb::protocol::randr::{
    get_crtc_gamma, get_crtc_gamma_size, get_crtc_info, get_screen_resources_current,
    set_crtc_gamma,
};

const TEMP: f64 = 2000.0;

// { 1.00000000,  0.71976951,  0.42860152, },

struct Gamma {
    red: Vec<f64>,
    green: Vec<f64>,
    blue: Vec<f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    let resources = get_screen_resources_current(&conn, screen.root)?.reply()?;

    let ratio = TEMP % 500.0 / 500.0;

    let gamma_r = 1.0 * (1.0 - ratio) + (1.0 * ratio);
    let gamma_g = 0.71976951 * (1.0 - ratio) + (0.677747 * ratio);
    let gamma_b = 0.42860152 * (1.0 - ratio) + (0.320666 * ratio);

    for crtc in resources.crtcs {
        let _crtc_info = get_crtc_info(&conn, crtc, crtc)?.reply()?;

        let size = get_crtc_gamma_size(&conn, crtc)?.reply()?;

        let _gamma = get_crtc_gamma(&conn, crtc)?.reply()?;

        let mut red: Vec<f64> = Vec::new();
        let mut green: Vec<f64> = Vec::new();
        let mut blue: Vec<f64> = Vec::new();

        for j in _gamma.red {
            let g = f64::from(j) / f64::from(size.size);
            println!("{} {}", j, f64::from(g) * 65535.0 * gamma_r);
        }

        for i in 0..size.size {
            let g = 65535.0 * f64::from(i) / f64::from(size.size);

            red.push(g * gamma_r);
            green.push(g * gamma_g);
            blue.push(g * gamma_b);
        }

        let mut redu16: Vec<u16> = Vec::new();
        let mut greenu16: Vec<u16> = Vec::new();
        let mut blueu16: Vec<u16> = Vec::new();

        for i in red {
            redu16.push(i as u16);
        }

        for i in green {
            greenu16.push(i as u16);
        }

        for i in blue {
            blueu16.push(i as u16);
        }

        let _f = set_crtc_gamma(&conn, crtc, &redu16, &greenu16, &blueu16);
    }

    let _f = conn.flush();

    Ok(())
}
