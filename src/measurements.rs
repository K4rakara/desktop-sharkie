#[cfg(platform_windows)] use winapi;
use glium;

use std::ops::Sub;

use glium::glutin::event_loop::EventLoop;
use glium::glutin::monitor::MonitorHandle;

use crate::UserEvent;

#[derive(Debug, Copy, Clone)]
pub struct Measurements {
    pub area_min_pos: (i32, i32),
    pub area_max_pos: (i32, i32),
    pub area_size: (i32, i32),
    pub monitor_size: (i32, i32),
    pub taskbar_height: i32,
    pub shark_pos: (i32, i32),
    pub shark_size: (i32, i32),
}

impl Measurements {
    pub fn new(event_loop: &EventLoop<UserEvent>) -> Self {
        let (area_min_pos, area_max_pos, area_size) = {
            let (mnx, mny, mxx, mxy) = event_loop.available_monitors()
                .fold(None, |a, b| match (a, b) {
                    (Some((ax, ay, amx, amy)), b) => {
                        let (bx, by) = {
                            let bp = b.position();
                            (bp.x, bp.y)
                        };
                        let (bmx, bmy) = {
                            let bs = b.size();
                            (bx + (bs.width as i32), by + (bs.height as i32))
                        };
                        Some((
                            if ax < bx { ax } else { bx },
                            if ay < by { ay } else { by },
                            if amx > bmx { amx } else { bmx },
                            if amy > bmy { amy } else { bmy },
                        ))
                    },
                    (None, b) => {
                        let (bx, by) = {
                            let bp = b.position();
                            (bp.x, bp.y)
                        };
                        let (bmx, bmy) = {
                            let bs = b.size();
                            (bx + (bs.width as i32), by + (bs.height as i32))
                        };
                        Some((bx, by, bmx, bmy))
                    },
                })
                .expect("No monitors found");
            (
                (mnx, mny),
                (mxx, mxy),
                (mxx - mnx, mxy - mny),
            )
        };
        
        let monitor_size = event_loop
            .available_monitors()
            .fold(None, |last: Option<MonitorHandle>, next| match last {
                Some(last) => {
                    let ax = last.position().x;
                    let bx = next.position().x;
                    Some(if ax < bx { last } else { next })
                },
                None => Some(next),
            })
            .map(|monitor| {
                let size = monitor.size();
                (size.width as i32, size.height as i32)
            })
            .unwrap();

        let taskbar_height = {
            #[cfg(platform_windows)] unsafe {
                use winapi::shared::windef::*;
                use winapi::um::shellapi::*;
                use winapi::um::winuser::*;

                let app_bar = (&mut APPBARDATA {
                    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
                    hWnd: FindWindowA(
                        b"Shell_TrayWnd\0".as_ptr() as *const i8,
                        std::ptr::null()),
                    uCallbackMessage: 0,
                    uEdge: 0,
                    rc: RECT {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0,
                    },
                    lParam: 0,
                }) as *mut APPBARDATA;

                SHAppBarMessage(ABM_GETTASKBARPOS, app_bar);

                ((*app_bar).rc.bottom - (*app_bar).rc.top) as i32
            } #[cfg(not(platform_windows))] {
                // A rough approximation :(
                ((32.0 / 900.0) * (monitor_size.1 as f32)).round() as i32
            }
        };

        let shark_size = {
            let shark_w = (
                  (180.0 / 900.0)
                * (monitor_size.1 as f32)
            ).round() as i32;
            (shark_w, shark_w)
        };

        let shark_pos = {
            #[cfg(platform_windows)] {
                (
                    area_min_pos.0 - shark_size.0,
                    ((area_size.1 as f32)
                        .sub((taskbar_height as f32) - (((shark_size.1 as f32) / 4.0) * 0.5))
                        .sub(shark_size.1 as f32))
                        .round() as i32,
                )
            } #[cfg(not(platform_windows))] {
                (
                    area_min_pos.0,
                    ((area_size.1 as f32)
                        .sub((taskbar_height as f32) * (3.5 / 4.0))
                        .sub(shark_size.1 as f32))
                        .round() as i32,
                )
            }
        };

        Measurements {
            area_min_pos,
            area_max_pos,
            area_size,
            monitor_size,
            taskbar_height,
            shark_pos,
            shark_size,
        }
    }
}
