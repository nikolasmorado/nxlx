use x11rb::connect;
use x11rb::connection::Connection;
use x11rb::protocol::randr::{
    get_crtc_gamma_size, get_crtc_info, get_screen_resources_current, get_crtc_gamma
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    let resources = get_screen_resources_current(&conn, screen.root)?.reply()?;

    for crtc in resources.crtcs {
        let crtc_info = get_crtc_info(&conn, crtc, crtc)?.reply()?;

        let size = get_crtc_gamma_size(&conn, crtc)?.reply()?;

        let gamma = get_crtc_gamma(&conn, crtc)?.reply()?;
        
        for i in 0..size.size {
            let g = 65535 * i / size.size;
            
            // gamma.red[i] = g * 
        }
    }

    let _f = conn.flush();

    Ok(())
}
