use crossbeam_channel;
use glium;
use image;
use rayon;

use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };

use crossbeam_channel::{ self as channel, Receiver };
use glium::Display;
use glium::glutin::dpi::PhysicalSize;
use glium::texture::{  SrgbTexture2d, RawImage2d };
use image::imageops::FilterType;
use image::{ Rgba, DynamicImage, ImageBuffer };
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

pub const SIZE_FRAMES: usize = 680;
pub const NUM_FRAMES: usize = 22;

pub static FRAMES: [&'static [u8]; NUM_FRAMES] = [
    include_bytes!("frame01.png"),
    include_bytes!("frame02.png"),
    include_bytes!("frame03.png"),
    include_bytes!("frame04.png"),
    include_bytes!("frame05.png"),
    include_bytes!("frame06.png"),
    include_bytes!("frame07.png"),
    include_bytes!("frame08.png"),
    include_bytes!("frame09.png"),
    include_bytes!("frame10.png"),
    include_bytes!("frame11.png"),
    include_bytes!("frame12.png"),
    include_bytes!("frame13.png"),
    include_bytes!("frame14.png"),
    include_bytes!("frame15.png"),
    include_bytes!("frame16.png"),
    include_bytes!("frame17.png"),
    include_bytes!("frame18.png"),
    include_bytes!("frame19.png"),
    include_bytes!("frame20.png"),
    include_bytes!("frame21.png"),
    include_bytes!("frame22.png"),
];

#[derive(Debug)]
pub struct Frames {
    completed: Vec<Rc< SrgbTexture2d>>,
    current: usize,
    display: Display,
    full: bool,
    receiver: Option<Receiver<Vec<u8>>>,
    size: PhysicalSize<u32>,
}

impl Frames {
    pub fn new(display: &Display, ready: Arc<AtomicBool>) -> Self {
        let (width, height, size) = {
            let size = display.gl_window().window().inner_size();
            (size.width as i32, size.height as i32, size)
        };

        let (sender, receiver) =
            channel::unbounded::<Vec<u8>>();

        rayon::spawn(move || {
            FRAMES
                .par_iter()
                .map(|bytes: &'_ &'_ [u8]| image::load_from_memory(bytes))
                .filter(|result| result.is_ok())
                .map(|result| result.unwrap())
                .map(|image: DynamicImage| {
                    image.resize(
                        width as u32,
                        height as u32,
                        FilterType::Triangle)
                })
                .map(|image: DynamicImage| image.into_rgba8())
                .map(|rgba: ImageBuffer<Rgba<u8>, Vec<u8>>| rgba.clone().into_raw())
                .collect::<Vec<Vec<u8>>>()
                .iter()
                .cloned()
                .for_each(|raw: Vec<u8>| { let _ = sender.send(raw); });
            ready.store(true, Ordering::Relaxed);
        });

        Frames {
            completed: Vec::new(),
            current: 0,
            display: display.clone(),
            full: false,
            size,
            receiver: Some(receiver),
        }
    }
}

impl Iterator for Frames {
    type Item = Rc< SrgbTexture2d>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.full {
            let current = self.current;
            let value = match self.receiver {
                Some(ref receiver) => match receiver.recv() {
                    Ok(value) => {
                        let image = RawImage2d::from_raw_rgba_reversed(
                            &value,
                            (self.size.width, self.size.height));
                        let texture =  SrgbTexture2d::new(
                            &self.display,
                            image);
                        match texture {
                            Ok(texture) => texture,
                            Err(..) => return None,
                        }
                    },
                    Err(..) => return None,
                },
                None => return None,
            };
            self.completed.push(Rc::new(value));
            self.current += 1;
            if self.current >= NUM_FRAMES {
                self.current = 0;
                self.full = true;
                self.receiver = None;
            }
            match self.completed.get(current) {
                Some(completed) => Some(completed.clone()),
                None => None,
            }
        } else {
            let value = self.completed.get(self.current).unwrap();
            self.current += 1;
            if self.current >= NUM_FRAMES { self.current = 0; }
            Some(value.clone())
        }
    }
}
